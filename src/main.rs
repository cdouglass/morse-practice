extern crate rand;
extern crate regex;

use rand::Rng;
use regex::Regex;

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod audio;
mod dvorak;
mod encoding;
mod words;

use words::WordGenerator;

const MAX_WORD_COUNT: usize = 3;
const MAX_WORD_LENGTH: usize = 7; // only limits random words, not real ones
const DICT_FILENAME: &str = "/usr/share/dict/words";

// TODO better interface to choose
fn characters(a: u32) -> Vec<char> {
    let doubles = "ainm";
    let triples = "osrduwkg";
    let quartets = "hlpcybfqjvxz";
    let chosen = match a {
      0 => { return dvorak::minimal(); },
      1 => { return dvorak::home(); },
      2 => doubles.to_owned(),
      3 => triples.to_owned(),
      4 => quartets.to_owned(),
      _ => String::from(doubles) + triples + quartets,
    };
    chosen.chars().collect()
}

//TODO filtering might belong in WordPicker::new instead
fn load_dict(filename: &str, charset: &Vec<char>) -> Vec<String> {
    //TODO below is amazingly stupid
    let cs = String::from(charset.iter().map(|c| vec![c.clone()].into_iter().collect()).collect::<Vec<String>>().join(""));
    let r = String::from("^[") + &cs + "]*$";
    let regex = Regex::new(&r).unwrap();

    let dict_file = File::open(filename).unwrap();
    BufReader::new(dict_file)
        .lines()
        .map(|line| { line.unwrap() })
        .filter(|word| { regex.is_match(word) })
        .collect()
}

fn quiz(message: &String, stdin: &std::io::Stdin) -> bool {
    let mut passing = true;
    let elements = encoding::encode(message);

    loop {
        audio::play(&elements).output().unwrap();
        let answer = stdin.lock().lines().next().unwrap().unwrap().clone();

        if &answer == message {
            println!("--------");
            break
        } else {
            passing = false;
            println!("You copied _{}_,\nbut I sent _{}_.\nPress ENTER to try it again.", answer, message);
            stdin.lock().lines().next();
        }
    }

    passing
}

fn main() {
    let mut args = env::args();
    args.next();
    let arg: String = args.next().unwrap_or(String::new());

    let mut word_gen = match arg.trim().parse().ok() {
        Some(n) => {
            let char_set = characters(n);
            WordGenerator::new(&char_set, MAX_WORD_LENGTH)
        },
        None => {
            let char_set = characters(999);
            let dict = load_dict(DICT_FILENAME, &char_set);
            WordGenerator::new_with_dict(dict)
        }
    };

    let mut rng = rand::thread_rng();

    let mut total_correct = 0;
    let mut total_answered = 0;

    println!("Press ENTER to start");
    let stdin = std::io::stdin();
    stdin.lock().lines().next();

    while total_answered < 25 {
        let n = rng.gen_range(1, MAX_WORD_COUNT + 1);
        println!("Check: {}", n); // convention from radiogram preamble

        let message = word_gen.get_n_words(n);
        let correct = quiz(&message, &stdin);

        total_answered += 1;
        if correct { total_correct += 1; }
    }

    let percentage = total_correct * 4;
    println!("You've correctly copied {} of {} words, or {}%.", total_correct, total_answered, percentage);

    if percentage >= 90 {
        println!("Good work. Time to add a new letter!");
    } else if percentage > 50 {
        println!("Getting there...");
    } else {
        println!("I think you need to take a break.");
    }
}
