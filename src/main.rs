extern crate rand;

use rand::Rng;
use std::io::BufRead;

mod audio;
mod encoding;
mod words;

use words::WordGenerator;

fn quiz(message: &String) -> bool {
    let mut passing = true;
    let elements = encoding::encode(message);

    loop {
        audio::play(&elements).output().unwrap();
        let stdin = std::io::stdin();
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
    let word_counts = vec![1, 2, 2, 3];
    let word_lengths = vec![1, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6];
    let chars_so_far = vec!['e', 't', 'a', 'o', 'i', 'n'];

    let mut word_gen = WordGenerator::new(chars_so_far, word_lengths);
    let mut rng = rand::thread_rng();

    let mut total_correct = 0;
    let mut total_answered = 0;

    while total_answered < 25 {
        let n = *rng.choose(&word_counts).unwrap();
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
