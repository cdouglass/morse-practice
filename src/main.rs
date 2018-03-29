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
Usage: morse [-p <pitch>] [-m <mode>] [-d] [-c <chars>] <wl-min> <wl-max> <wc-min> <wc-max>
       morse [-p <pitch>] [-t <filename>] <wc-min> <wc-max> [<offset>]

Options:
    -p, --pitch <pitch>     Pitch in Hz [default: 440]
    -m, --mode <mode>       Mode, denermines character set. [default: abc]
                            Valid values: abc, all, num
    -d, --dict              Use real words from dictionary file
    -c, --chars <chars>     Require every word to have all the listed characters
    -t, --text <filename>   Use a text source!
";

#[derive(Deserialize, Debug)]
struct Args {
    flag_pitch: u32,
    flag_mode: Mode,
    flag_dict: bool,
    flag_chars: String,
    flag_text: Option<String>,
    arg_wl_min: usize,
    arg_wl_max: usize,
    arg_wc_min: usize,
    arg_wc_max: usize,
    arg_offset: Option<usize>,
}

#[derive(Deserialize, Debug)]
enum Mode { Abc, All, Num }

const DICT_FILENAME: &str = "/usr/share/dict/words";

//TODO store as set
fn char_set(m: &Mode) -> Vec<char> {
    match *m {
      Mode::Abc => dvorak::minimal(),
      Mode::All => dvorak::all(),
      Mode::Num => vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
    }
}

fn quiz(message: &Vec<String>, stdin: &std::io::Stdin, pitch: u32) -> usize {
    let mut n_correct = None; //TODO do I have to initialize before loop?
    println!("Check: {}", message.len()); // convention from radiogram preamble

    let msg_str = message.join(" ");
    let elements = encoding::encode(&msg_str);

    loop {
        audio::play(&elements, pitch).output().unwrap();
        let answer = stdin.lock().lines().next().unwrap().unwrap().clone();

        let ans_words = answer.trim().split(' ');
        let pairs = message.iter().zip(ans_words);
        let m = pairs.filter(|&(a, b)| a == b).count();

        if n_correct == None {
             n_correct = Some(m);
        }

        if answer.trim() == msg_str {
            println!("--------");
            break
        } else {
            audio::bzzt().output().unwrap();
            println!("{}_ was the correct answer. You got {} of {} words. Press ENTER to try again.",
                     msg_str, m, message.len());
            stdin.lock().lines().next();
        }
    }

    n_correct.unwrap()
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(env::args()).deserialize())
        .unwrap_or_else(|e| e.exit());

    let pitch = args.flag_pitch;

    let max_words: usize;
    let offset = args.arg_offset.unwrap_or(0);

    let mut rng = rand::thread_rng();
    let mut word_gen = match args.flag_text {
        Some(ref text_filename) => {
            max_words = 1000000;
            WordGenerator::text_reader(text_filename, dvorak::all())
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

        let n_correct = quiz(&message, &stdin, pitch);
        words_correct += n_correct;
        if n_correct == n { messages_correct += 1; }
    }

    let message_percentage = messages_correct * 100 / messages_answered;
    let word_percentage = words_correct * 100 / words_answered;
    println!("You've correctly copied {} of {} messages, or {}%.", messages_correct, messages_answered, message_percentage);
    println!("You've correctly copied {} of {} words, or {}%.", words_correct, words_answered, word_percentage);

    if word_percentage >= 90 {
        println!("Good work. Time to add a new letter!");
    } else if word_percentage > 70 {
        println!("Getting there...");
    } else {
        println!("I think you need to take a break.");
    }
}
