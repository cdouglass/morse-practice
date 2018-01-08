use std::process::Command;

// 20 wpm <-> dot length 60ms
const PITCH: u32 = 220; // A below middle C
const SHORT: u32 = 80;
const LONG: u32 = 3 * SHORT;
const VERY_LONG: u32 = 7 * SHORT;


// specific implementation of dots and dashes etc
pub enum Sound {
    ShortBeep,
    LongBeep,
    ShortSilence,
    LongSilence
}
use self::Sound::*;

// Make it easy to tell when the tone is ending
fn add_click(cmd: &mut Command) {
    cmd.arg("-n")
       .arg(format!("-f {}", PITCH * 3 / 4))
       .arg(format!("-l {}", 2));
}

pub fn play(sounds: &Vec<Sound>, farnsworth: bool) -> Command {
    // create beep command with no tones
    let mut cmd = Command::new("beep");
    cmd.arg("-l 0");

    for sound in sounds {
        match *sound {
            // add new tone
            ShortBeep => {
                cmd.arg("-n")
                   .arg(format!("-D {}", SHORT))
                   .arg(format!("-f {}", PITCH))
                   .arg(format!("-l {}", SHORT));
                add_click(&mut cmd);
            },
            LongBeep => {
                cmd.arg("-n")
                   .arg(format!("-D {}", SHORT))
                   .arg(format!("-f {}", PITCH))
                   .arg(format!("-l {}", LONG));
                add_click(&mut cmd);
            },
            // change delay following last tone added
            ShortSilence => {
                //cmd.arg(format!("-D {}", LONG));
                cmd.arg("-n")
                   .arg(format!("-f {}", 1));
                if farnsworth {
                   cmd.arg(format!("-l {}", 5 * SHORT)); // screw this so far
                   //cmd.arg(format!("-l {}", VERY_LONG));
                } else {
                   cmd.arg(format!("-l {}", LONG));
                }
            },
            LongSilence  => {
                cmd.arg("-n")
                   .arg(format!("-f {}", 1));
                if farnsworth {
                   cmd.arg(format!("-l {}", 10 * SHORT));
                } else {
                   cmd.arg(format!("-D {}", VERY_LONG));
                }
            }
        }
    }

    cmd
}
