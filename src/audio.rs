use std::process::Command;

// 20 wpm <-> dot length 60ms
const PITCH: u32 = 220; // A below middle C
const TINY: u32 = 2;

const SHORT: u32 = 80;
const LONG: u32 = 3 * SHORT;
const VERY_LONG: u32 = 7 * SHORT;

const FARNSWORTH_LONG: u32 = 6 * SHORT;
const FARNSWORTH_VERY_LONG: u32 = 10 * LONG;


// specific implementation of dots and dashes etc
// TODO no longer seems needed. instead, delete to_sound() method, create a Beep struct with
// duration etc, and implement a from_element function for that
pub enum Sound {
    ShortBeep,
    LongBeep,
    ShortSilence,
    LongSilence
}

fn add_beep(cmd: &mut Command, pitch: u32, length: u32, delay: u32) {
    cmd.arg("-n")
       .arg(format!("-f {}", pitch))
       .arg(format!("-D {}", delay))
       .arg(format!("-l {}", length));
}

pub fn play(sounds: &Vec<Sound>, farnsworth: bool) -> Command {
use self::Sound::*;
    // create beep command with a single zero-duration tone
    let mut cmd = Command::new("beep");
    cmd.arg("-l 0");

    for sound in sounds {
        match *sound {
            // add new tone
            ShortBeep => {
                add_beep(&mut cmd, PITCH, SHORT, 0);
                add_beep(&mut cmd, PITCH * 3 / 4, TINY, SHORT); // click makes it clearer when tone is ending
            },
            LongBeep => {
                add_beep(&mut cmd, PITCH, LONG, 0);
                add_beep(&mut cmd, PITCH * 3 / 4, TINY, SHORT);
            },
            // add new inaudibly low tone
            ShortSilence => {
                if farnsworth {
                    add_beep(&mut cmd, 1, FARNSWORTH_LONG, 0);
                } else {
                    add_beep(&mut cmd, 1, LONG, 0);
                }
            },
            LongSilence  => {
                if farnsworth {
                    add_beep(&mut cmd, 1, FARNSWORTH_VERY_LONG, 0);
                } else {
                    add_beep(&mut cmd, 1, VERY_LONG, 0);
                }
            }
        }
    }

    cmd
}
