use std::process::Command;

pub enum Element {
    Dot,
    Dash
}
use Element::Dot;
use Element::Dash;


fn chain<I>(message: I) -> Command
where I: Iterator<Item=Element> {
    // create beep command with no tones
    let mut cmd = Command::new("beep");
    cmd.arg("-l 0");

    for element in message {
        add_element(element, &mut cmd);
    }

    cmd
}

fn add_element(element: Element, cmd: &mut Command) {
    // append new tone; set frequency and following delay
    cmd.args(&["-n", "-f 220", "-D 100"]);

    // set length of tone
    match element {
        Dot  => { cmd.arg("-l 100"); },
        Dash => { cmd.arg("-l 300"); },
    }
}

fn main() {
    let message = vec![Dot, Dash, Dot];
    chain(message.into_iter()).spawn().unwrap();
}
