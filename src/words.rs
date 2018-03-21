use std::fs::File;

use rand;
use rand::Rng;
use regex::Regex;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

pub struct WordGenerator {
    reservoir: Reservoir,
    rng: rand::ThreadRng
}

enum Reservoir {
    Dict(Vec<String>),
    Chars(Vec<char>, (usize, usize)),
    Reader(Read_)
}
use self::Reservoir::*;

struct Read_ {
    pub words: Box<Iterator<Item=String>>
}

impl Iterator for WordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.reservoir {
            Dict(ref d) => {
                self.rng.choose(&d).map(|s| s.clone())
            },
            Chars(ref chars, (ref min, ref max)) => {
                let mut word = String::new();
                for _ in 0..(self.rng.gen_range(*min, *max + 1)) {
                    word.push(*self.rng.choose(&chars).unwrap());
                }
                Some(word)
            },
            Reader(ref mut read) => {
                read.words.next().map(|s| s.to_lowercase())
            }
        }
    }
}

impl WordGenerator {
    pub fn new(characters: Vec<char>, min_length: usize, max_length: usize, dict_filename: Option<&str>) -> WordGenerator {
        let reservoir = match dict_filename {
            Some(f) => Dict(load_dict(f, characters, min_length, max_length)),
            None => Chars(characters, (min_length, max_length))
        };
        WordGenerator {
            reservoir: reservoir,
            rng: rand::thread_rng()
        }
    }

    pub fn text_reader(filename: &str, mut char_set: Vec<char>) -> WordGenerator {
        char_set.push(' ');
        char_set.push('\n');
        let mut text_file = File::open(filename).unwrap();
        let mut contents = String::new();
        text_file.read_to_string(&mut contents).expect(&format!("Couldn't read file {}", filename));
        let filtered_contents: String = contents.to_lowercase().chars().filter(|c| char_set.contains(c)).collect();
        let words: Vec<String> = filtered_contents.split_whitespace().map(|w| w.to_owned()).collect();
        let reader = Read_ {
            words: Box::new(words.into_iter())
        };
        WordGenerator {
            reservoir: Reader(reader),
            rng: rand::thread_rng()
        }
    }
}

fn load_dict(filename: &str, charset: Vec<char>, min_length: usize, max_length: usize) -> Vec<String> {
    let mut cs = String::new();
    for c in charset {cs.push(c)}
    let r = String::from("^[") + &cs + "]*$";
    let regex = Regex::new(&r).unwrap();

    let dict_file = File::open(filename).unwrap();
    BufReader::new(dict_file)
        .lines()
        .map(|line| { line.unwrap() })
        .filter(|word| { regex.is_match(word) })
        .filter(|word| { word.len() >= min_length })
        .filter(|word| { word.len() <= max_length })
        .collect()
}
