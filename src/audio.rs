use std::process::Command;

use encoding::Element;

// "Farnsworth timing" is technique of adding extra space between characters or words beyond the
// defaults of 3 and 7 dot units respectively, while transmitting each individual character at
// the normal rate.
const CHAR_SPACE: u32 = 3;
const WORD_SPACE: u32 = 7;


struct Tone {
    audible: bool,
    length:  u32
}

impl Tone {
    pub fn from_element(element: &Element) -> Tone {
        use self::Element::*;
        match *element {
            Dot       => Tone { audible: true,  length: 1 },
            Dash      => Tone { audible: true,  length: 3 },
            CharSpace => Tone { audible: false, length: CHAR_SPACE },
            WordSpace => Tone { audible: false, length: WORD_SPACE },
        }
    }
}

pub fn play(elements: &Vec<Element>, pitch: u32, dot_length: u32) -> Command {
    // create beep command with a single inaudibly low tone
    let mut cmd = Command::new("beep");
    cmd.arg("-f 1");
    cmd.arg("-D 1000");

    for elt in elements {
        let tone = Tone::from_element(elt);
        play_tone(&mut cmd, tone, pitch, dot_length);
    }

    cmd
}

pub fn bzzt() -> Command {
    let mut cmd = Command::new("beep");
    cmd.arg("-f 100");
    cmd.arg("-l 500");

    cmd
}

fn play_tone(mut cmd: &mut Command, tone: Tone, pitch: u32, dot_length: u32) {
    if tone.audible {
        add_beep(&mut cmd, pitch, tone.length * dot_length);
        add_beep(&mut cmd, 1, dot_length);
    } else {
        add_beep(&mut cmd, 1, (tone.length - 1) * dot_length);
    }
}

fn add_beep(cmd: &mut Command, pitch: u32, length: u32) {
    cmd.arg("-n")
       .arg(format!("-f {}", pitch))
       .arg(format!("-l {}", length));
}
