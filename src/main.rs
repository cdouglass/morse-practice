extern crate docopt;
extern crate rand;
extern crate regex;

#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use rand::Rng;

use std::env;
use std::io::BufRead;

mod audio;
mod dvorak;
mod encoding;
mod words;

use words::WordGenerator;

const USAGE: &'static str = "
Usage: morse [-p <pitch>] [-d]

Options:
    -p, --pitch <pitch>    Pitch in Hz
    -d, --dict             Use real words from dictionary file
";

#[derive(Deserialize)]
#[derive(Debug)]
struct Args {
    flag_pitch: Option<u32>,
    flag_dict: bool,
}

const DEFAULT_PITCH: u32 = 440;
const MIN_WORD_COUNT: usize = 2;
const MAX_WORD_COUNT: usize = 4;
const MIN_WORD_LENGTH: usize = 2; // only limits random words, not real ones
const MAX_WORD_LENGTH: usize = 7;
const DICT_FILENAME: &str = "/usr/share/dict/words";

// TODO better interface to choose
fn characters(a: u32) -> Vec<char> {
    let doubles = "ainm";
    let triples = "osrduwkg";
    let quartets = "hlpcybfqjvxz";
    let digits = "23456789";
    let chosen = match a {
      0 => { return dvorak::minimal(); },
      1 => { return dvorak::home(); },
      2 => doubles.to_owned(),
      3 => triples.to_owned(),
      4 => quartets.to_owned(),
      5 => digits.to_owned(),
      _ => String::from(doubles) + triples + quartets,
    };
    chosen.chars().collect()
}

fn quiz(message: &String, stdin: &std::io::Stdin, pitch: u32) -> bool {
    let mut passing = true;
    let elements = encoding::encode(message);

    loop {
        audio::play(&elements, pitch).output().unwrap();
        let answer = stdin.lock().lines().next().unwrap().unwrap().clone();

        if &answer.trim() == message {
            println!("--------");
            break
        } else {
            passing = false;
            audio::bzzt().output().unwrap();
            println!("You copied _{}_,\nbut I sent _{}_.\nPress ENTER to try it again.", answer, message);
            stdin.lock().lines().next();
        }
    }

    passing
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(env::args()).deserialize())
        .unwrap_or_else(|e| e.exit());

    let char_set = characters(0);
    let pitch = args.flag_pitch.unwrap_or(DEFAULT_PITCH);

    let mut rng = rand::thread_rng();
    let mut word_gen = if args.flag_dict {
        WordGenerator::new(&char_set, MIN_WORD_LENGTH, MAX_WORD_LENGTH, Some(DICT_FILENAME))
    } else {
        WordGenerator::new(&char_set, MIN_WORD_LENGTH, MAX_WORD_LENGTH, None)
    };

    let mut total_correct = 0;
    let mut total_answered = 0;

    println!("Press ENTER to start");
    let stdin = std::io::stdin();
    stdin.lock().lines().next();

    while total_answered < 25 {
        let n = rng.gen_range(MIN_WORD_COUNT, MAX_WORD_COUNT + 1);
        println!("Check: {}", n); // convention from radiogram preamble

        let message = word_gen.get_n_words(n);
        let correct = quiz(&message, &stdin, pitch);

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
