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
mod encoding;
mod words;

use words::WordGenerator;

const USAGE: &'static str = "
Usage: morse [-p <pitch>] [-l <dot>] [-d] [-c <chars>] <wl-min> <wl-max> <wc-min> <wc-max>
       morse [-p <pitch>] [-l <dot>] -m <mode> [-c <chars>] <wl-min> <wl-max> <wc-min> <wc-max>
       morse [-p <pitch>] [-l <dot>] -t <filename> [-o <offset>] <wc-min> <wc-max>

Options:
    -p, --pitch <pitch>    Pitch in Hz [default: 440]
    -l, --length <dot>     Dot length in ms. 60ms is about 20wpm. [default: 80]
    -d, --dict             Use real words from dictionary file
    -c, --chars <chars>    Require every word to have all the listed characters
    -m, --mode <mode>      Mode, denermines character set. [default: abc]
                               Valid values: abc, all, num
    -t, --text <filename>  Read through a text
    -o, --offset <offset>  Start this many words into the text. [default: 0]
";

#[derive(Deserialize, Debug)]
struct Args {
    flag_pitch:  u32,
    flag_length: u32,
    flag_mode:   Mode,
    flag_dict:   bool,
    flag_chars:  String,
    flag_text:   Option<String>,
    flag_offset: usize,
    arg_wl_min:  usize,
    arg_wl_max:  usize,
    arg_wc_min:  usize,
    arg_wc_max:  usize,
}

#[derive(Deserialize, Debug)]
enum Mode { Abc, All, Num }

const DICT_FILENAME: &str = "/usr/share/dict/words";

//TODO store as set
//TODO handle subsets of alphabet
fn char_set(m: &Mode) -> Vec<char> {
    match *m {
      Mode::Abc => "abcdefghijklmnopqrstuvwxyz".chars().collect(),
      Mode::All => encoding::all_chars(),
      Mode::Num => vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
    }
}

fn quiz(message: &Vec<String>, stdin: &std::io::Stdin, pitch: u32, dot_length: u32) -> usize {
    let mut n_correct = None;
    let mut num_tries = 0;

    let msg_str = message.join(" ");
    let elements = encoding::encode(&msg_str);

    loop {
        audio::play(&elements, pitch, dot_length).output().unwrap();
        let answer = stdin.lock().lines().next().unwrap().unwrap().clone();
        let pairs = answer.trim().split(' ').zip(message.iter());

        if n_correct == None {
            n_correct = Some(pairs.clone().filter(|&(a, b)| a == b).count());
        }

        if answer.trim() == msg_str {
            break
        } else {
            audio::bzzt().output().unwrap();
            let feedback = pairs
                .map(|(a, c)| feedback(a, c, num_tries))
                .collect::<Vec<String>>()
                .join(" ");
            println!("{}\nPress ENTER to try again.", feedback);
            stdin.lock().lines().next();
            num_tries += 1;
        }
    }

    n_correct.unwrap()
}

fn feedback(answer_word: &str, correct_word: &str, num_tries: usize) -> String {
    if correct_word == answer_word || num_tries > 1 {
        String::from(correct_word)
    } else if num_tries == 0 {
        String::from("_")
    } else {
        correct_word.chars().into_iter().map(|_| "_").collect::<Vec<&str>>().join("")
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(env::args()).deserialize())
        .unwrap_or_else(|e| e.exit());

    let pitch = args.flag_pitch;
    let dot_length = args.flag_length;

    let max_words: usize;
    let offset = args.flag_offset;

    let mut rng = rand::thread_rng();
    let mut word_gen = match args.flag_text {
        Some(ref text_filename) => {
            max_words = 100;
            WordGenerator::text_reader(text_filename, encoding::all_chars())
        },
        None => {
            max_words = 25;
            WordGenerator::new(
                char_set(&args.flag_mode),
                args.arg_wl_min,
                args.arg_wl_max,
                if args.flag_dict { Some(DICT_FILENAME) } else { None })
        }
    }.filter(|w| args.flag_chars.chars().all(|c| w.contains(c))).skip(offset);

    let mut messages_correct = 0;
    let mut messages_answered = 0;
    let mut words_correct = 0;
    let mut words_answered = 0;

    println!("Press ENTER to start");
    let stdin = std::io::stdin();
    stdin.lock().lines().next();

    while words_answered < max_words {
        let n = rng.gen_range(args.arg_wc_min, args.arg_wc_max + 1);
        let message: Vec<String> = (&mut word_gen).take(n).map(|w| w.to_lowercase()).collect();

        words_answered += n;
        messages_answered += 1;

        if args.arg_wc_min != args.arg_wc_max {
            println!("Check: {}", message.len()); // convention from radiogram preamble
        }
        let n_correct = quiz(&message, &stdin, pitch, dot_length);
        words_correct += n_correct;
        if n_correct == n { messages_correct += 1; }
    }

    let message_percentage = messages_correct * 100 / messages_answered;
    let word_percentage = words_correct * 100 / words_answered;
    println!("You've correctly copied {} of {} messages, or {}%.", messages_correct, messages_answered, message_percentage);
    println!("You've correctly copied {} of {} words, or {}%.", words_correct, words_answered, word_percentage);

    if word_percentage >= 90 {
        println!("Good work!");
    } else if word_percentage > 70 {
        println!("Getting there...");
    } else {
        println!("I think you need to take a break.");
    }
}
