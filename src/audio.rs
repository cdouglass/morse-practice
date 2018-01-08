use std::process::Command;

use encoding::Element;

// 20 wpm <-> dot length 60ms
const PITCH: u32 = 220; // A below middle C
const TINY: u32 = 2;

const DOT_LENGTH: u32 = 80;
const LONG: u32 = 3 * DOT_LENGTH;

// "Farnsworth timing" is technique of adding extra space between words and characters, while
// transmitting each individual character at normal rate
// Spaces will actually all be one dot length longer than stated due to delay added after audible tones
const FARNSWORTH_LONG: u32 = 5 * DOT_LENGTH; // Space between characters; normally 3 * DOT_LENGTH
const FARNSWORTH_VERY_LONG: u32 = 7 * LONG; // Space between words; normally 7 * DOT_LENGTH


struct Tone {
    audible: bool,
    length:  u32
}

impl Tone {
    pub fn from_element(element: &Element) -> Tone {
        use self::Element::*;
        match *element {
            Dot       => Tone { audible: true,  length: DOT_LENGTH },
            Dash      => Tone { audible: true,  length: LONG },
            CharSpace => Tone { audible: false, length: FARNSWORTH_LONG },
            WordSpace => Tone { audible: false, length: FARNSWORTH_VERY_LONG },
        }
    }
}

pub fn play(elements: &Vec<Element>) -> Command {
    // create beep command with a single inaudibly low tone
    let mut cmd = Command::new("beep");
    cmd.arg("-f 1");

    for elt in elements {
        let tone = Tone::from_element(elt);
        play_tone(&mut cmd, tone);
    }

    cmd
}

fn play_tone(mut cmd: &mut Command, tone: Tone) {
    if tone.audible {
        add_beep(&mut cmd, PITCH, tone.length, 0);
        add_beep(&mut cmd, PITCH * 3 / 4, TINY, DOT_LENGTH); // click to make clear tone has ended
    } else {
        add_beep(&mut cmd, 1, tone.length, 0);
    }
}

fn add_beep(cmd: &mut Command, pitch: u32, length: u32, delay: u32) {
    cmd.arg("-n")
       .arg(format!("-f {}", pitch))
       .arg(format!("-D {}", delay))
       .arg(format!("-l {}", length));
}
