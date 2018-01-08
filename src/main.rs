extern crate rand;

use rand::Rng;
use std::io::BufRead;

mod audio;
mod encoding;
mod message_generator;

use message_generator::WordGenerator;

// "Farnsworth timing" is technique of adding extra space between words and characters, while
// transmitting each individual character at normal rate
const FARNSWORTH: bool = true;

fn quiz(message: &String) -> bool {
    let mut passing = true;

    let sounds = encoding::encode(message)
        .into_iter()
        .map(|e| e.to_sound())
        .collect();

    loop {
        audio::play(&sounds, FARNSWORTH).output().unwrap();
        let stdin = std::io::stdin();
        let answer = stdin.lock().lines().next().unwrap().unwrap().clone();

        if &answer == message {
            println!("--------");
            break
        } else {
            passing = false;
            println!("You copied _{}_,\nbut I sent _{}_.\nLet's try that one again. Press ENTER when you're ready.", answer, message);
           stdin.lock().lines().next();
        }
    }

    passing
}

fn main() {
    let mut word_gen = WordGenerator::new(vec!['e', 't', 'a', 'o', 'i', 'n']);
    let wc_distribution = [1, 2];
    let mut rng = rand::thread_rng();

    let mut total_correct = 0;
    let mut total_answered = 0;

    while total_answered < 25 {
        let n = *rng.choose(&wc_distribution).unwrap();
        println!("Check: {}", n); // convention from radiogram preamble

        let message = word_gen.get_n_words(n);
        let correct = quiz(&message);

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
