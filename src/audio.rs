use std::process::Command;

// 20 wpm <-> dot length 60ms
const PITCH: u32 = 220; // A below middle C
const SHORT: u32 = 100;
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

pub fn play_sounds<I>(sounds: I) -> Command
where I: Iterator<Item=Sound> {
    // create beep command with no tones
    let mut cmd = Command::new("beep");
    cmd.arg("-l 0");

    for sound in sounds {
        match sound {
            // add new tone
            ShortBeep => {
                cmd.arg("-n")
                   .arg(format!("-D {}", SHORT))
                   .arg(format!("-f {}", PITCH))
                   .arg(format!("-l {}", SHORT));
            },
            LongBeep => {
                cmd.arg("-n")
                   .arg(format!("-D {}", SHORT))
                   .arg(format!("-f {}", PITCH))
                   .arg(format!("-l {}", LONG));
            },
            // change delay following last tone added
            ShortSilence => { cmd.arg(format!("-D {}", LONG)); },
            LongSilence  => { cmd.arg(format!("-D {}", VERY_LONG)); }
        }
    }

    cmd
}
