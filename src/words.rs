use std::fs::File;

use rand;
use rand::Rng;
use regex::Regex;
use std::io::BufRead;
use std::io::BufReader;

pub struct WordGenerator {
    min_length: usize,
    max_length: usize,
    //TODO only one of the following
    characters: Vec<char>,
    dict: Option<Vec<String>>,
    rng: rand::ThreadRng
}

impl Iterator for WordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.dict {
            Some(ref d) => {
                Some(get_item(d, &mut self.rng))
            },
            None => {
                let length = self.rng.gen_range(self.min_length, self.max_length + 1);
                let word = (0..length)
                    .map(|_| { get_item(&self.characters, &mut self.rng) })
                    .collect::<String>();
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
        let dict = dict_filename.map(|f| load_dict(f, characters, min_length, max_length));
        WordGenerator {
            characters: characters.into_iter().map(|x| *x).collect(),
            min_length: min_length,
            max_length: max_length,
            dict: dict,
            rng: rand::thread_rng()
        }
    }
}

//TODO do I actually need to hold an rng for this?
//TODO also seems like slices should be fine instead of Vec if I just knew how to make lifetimes
//work
fn get_item<T: Clone>(vals: &Vec<T>, rng: &mut rand::ThreadRng) -> T {
    rng.choose(vals).unwrap().clone()
}

fn load_dict(filename: &str, charset: &[char], min_length: usize, max_length: usize) -> Vec<String> {
    //TODO below is amazingly stupid
    let cs = String::from(
        charset.iter()
            .map(|c| vec![c.clone()].into_iter().collect())
            .collect::<Vec<String>>()
            .join(""));
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
