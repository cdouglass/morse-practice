extern crate rand;

use rand::Rng;
use std::io::BufRead;

mod audio;
mod encoding;
mod words;

use words::WordGenerator;

const WORD_COUNTS: [usize; 4] = [1, 2, 2, 3];
const WORD_LENGTHS: [usize; 21] = [1, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6];
const CHARS_SO_FAR: [char; 16] = ['e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'u', 'm', 'w', 'k', 'p'];
const DOUBLES: [char; 4] = ['a', 'i', 'n', 'm'];
const TRIPLES: [char; 7] = ['o', 's', 'r', 'd', 'u', 'w', 'k'];
const QUARTETS: [char; 3] = ['h', 'l', 'p'];

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
    let mut word_gen = WordGenerator::new(&CHARS_SO_FAR, &WORD_LENGTHS);
    let mut rng = rand::thread_rng();

    let mut total_correct = 0;
    let mut total_answered = 0;

    println!("Press ENTER to start");
    let stdin = std::io::stdin();
    stdin.lock().lines().next();

    while total_answered < 25 {
        let n = *rng.choose(&WORD_COUNTS).unwrap();
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
