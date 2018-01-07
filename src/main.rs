extern crate rand;
use rand::Rng;
use std::io::BufRead;

mod audio;
mod encoding;

// "Farnsworth timing" is technique of adding extra space between words and characters, while
// transmitting each individual character at normal rate
const FARNSWORTH: bool = true;

fn make_word(chars: &Vec<char>) -> String {
    let mut rng = rand::thread_rng();
    let lengths = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 4, 4, 5, 5];
    let mut n = *rng.choose(&lengths).unwrap();

    let mut word = String::new();

    while n > 0 {
        let ch = rng.choose(chars).unwrap();
        word.push(*ch);
        n -= 1;
    }

    word
}

//TODO dry up (this basically repeats make_word)
fn make_message(chars: &Vec<char>) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let lengths = [1, 2, 1];
    let mut n = *rng.choose(&lengths).unwrap();

    let mut words = vec![];

    while n > 0 {
        words.push(make_word(chars));
        n -= 1;
    }

    words
}

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
            println!("Good job!");
            break
        } else {
            println!("Oops, you copied {}, but I sent {}. Let's try that one again.", answer, message);
            passing = false;
        }
    }

    passing
}

fn good_enough(correct: i64, total: i64) -> bool {
    total >= 25 && (correct as f64 >= total as f64 * 0.9)
}

fn main() {
    let chars_so_far = vec!['e', 't'];
    let mut total_correct = 0;
    let mut total_answered = 0;
    loop {
        let words = make_message(&chars_so_far);
        println!("Check: {}", words.len());

        let message = words.join(" ");
        let correct = quiz(&message);
        total_answered += 1;
        if correct { total_correct += 1; }

        let percentage = 100.0 * total_correct as f64 / total_answered as f64;
        if total_answered % 5 == 0 {
            println!("{}/{}={}%\n", total_correct, total_answered, percentage);
        }

        if good_enough(total_correct, total_answered) {
            println!("Awesome, you've correctly copied {} of {} words, or {}%.\nTake it up a notch by adding a new letter.", total_correct, total_answered, percentage);
            break
        }
    }
}
