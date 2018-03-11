use std::fs::File;

use rand;
use rand::Rng;
use regex::Regex;
use std::io::BufRead;
use std::io::BufReader;

pub struct WordGenerator {
    reservoir: Reservoir,
    rng: rand::ThreadRng
}

enum Reservoir {
    Dict(Vec<String>),
    Chars(Vec<char>, (usize, usize))
}
use self::Reservoir::*;

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
            }
        }
    }
}

impl WordGenerator {
    pub fn get_n_words(&mut self, n: usize) -> String {
        self.take(n).collect::<Vec<String>>().join(" ")
    }

    pub fn new(characters: &[char], min_length: usize, max_length: usize, dict_filename: Option<&str>) -> WordGenerator {
        let reservoir = match dict_filename {
            Some(f) => Dict(load_dict(f, characters, min_length, max_length)),
            None => Chars(characters.into_iter().map(|x| *x).collect(), (min_length, max_length))
        };
        WordGenerator {
            reservoir: reservoir,
            rng: rand::thread_rng()
        }
    }
}

fn load_dict(filename: &str, charset: &[char], min_length: usize, max_length: usize) -> Vec<String> {
    let mut cs = String::new();
    for c in charset {cs.push(*c)}
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
