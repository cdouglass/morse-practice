extern crate rand;
use rand::Rng;
use std::io::BufRead;

mod audio;
mod encoding;

fn make_word(chars: &Vec<char>) -> String {
    let mut rng = rand::thread_rng();
    let lengths = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 7, 8];
    let mut n = *rng.choose(&lengths).unwrap();

    let mut word = String::new();

    while n > 0 {
        let ch = rng.choose(chars).unwrap();
        word.push(*ch);
        n -= 1;
    }

    word
}

fn quiz(message: &String) -> String {
    let sounds = encoding::encode(message)
        .into_iter()
        .map(|e| e.to_sound());
    audio::play(sounds).spawn().unwrap();

    // get input
    let stdin = std::io::stdin();
    let answer = stdin.lock().lines().next().unwrap().unwrap().clone();

    answer
}

fn good_enough(correct: i64, total: i64) -> bool {
    total >= 50 && (correct as f64 >= total as f64 * 0.9)
}

fn main() {
    let chars_so_far = vec!['e', 't'];
    let mut total_correct = 0;
    let mut total_answered = 0;
    loop {
        //TODO multi-word messages
        let message = make_word(&chars_so_far);
        let answer = quiz(&message);
        total_answered += 1;
        if answer == message {
            total_correct += 1;
            println!("GOOD JOB");
        } else {
            println!("Oops, you copied {}, but I sent {}. Let's try that one again.", answer, message);
            quiz(&message);
        }

        if good_enough(total_correct, total_answered) {
            let percentage = 100.0 * total_correct as f64 / total_answered as f64;
            println!("Awesome, you've correctly copied {} of {} words, or {}%.\nTake it up a notch by adding a new letter.", total_correct, total_answered, percentage);
            break
        }
    }
}
