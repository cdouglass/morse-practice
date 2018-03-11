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
Usage: morse [-p <pitch>] [-m <mode>] [-d] <wl-min> <wl-max> <wc-min> <wc-max>

Options:
    -p, --pitch <pitch>     Pitch in Hz [default: 440]
    -m, --mode <mode>       Mode, denermines character set. [default: abc]
                            Valid values: abc, all, num
    -d, --dict              Use real words from dictionary file
";

#[derive(Deserialize, Debug)]
struct Args {
    flag_pitch: u32,
    flag_mode: Mode,
    flag_dict: bool,
    arg_wl_min: usize,
    arg_wl_max: usize,
    arg_wc_min: usize,
    arg_wc_max: usize,
}

#[derive(Deserialize, Debug)]
enum Mode { Abc, All, Num }

const DICT_FILENAME: &str = "/usr/share/dict/words";

fn char_set(m: Mode) -> Vec<char> {
    match m {
      Mode::Abc => dvorak::minimal(),
      Mode::All => dvorak::all(),
      Mode::Num => vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
    }
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

    let pitch = args.flag_pitch;

    let mut rng = rand::thread_rng();
    let mut word_gen = WordGenerator::new(
        char_set(args.flag_mode),
        args.arg_wl_min,
        args.arg_wl_max,
        if args.flag_dict { Some(DICT_FILENAME) } else { None });

    let mut total_correct = 0;
    let mut total_answered = 0;

    println!("Press ENTER to start");
    let stdin = std::io::stdin();
    stdin.lock().lines().next();

    while total_answered < 25 {
        let n = rng.gen_range(args.arg_wc_min, args.arg_wc_max + 1);
        println!("Check: {}", n); // convention from radiogram preamble

        let message = word_gen.get_n_words(n);
        let correct = quiz(&message, &stdin, pitch);

        total_answered += 1;
        if correct { total_correct += 1; }
    }

    let percentage = total_correct * 4;
    println!("You've correctly copied {} of {} messages, or {}%.", total_correct, total_answered, percentage);

    if percentage >= 90 {
        println!("Good work. Time to add a new letter!");
    } else if percentage > 50 {
        println!("Getting there...");
    } else {
        println!("I think you need to take a break.");
    }
}
