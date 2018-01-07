use std::process::Command;

fn chain<I>(message: I) -> Command
where I: Iterator<Item=bool> {
    let mut cmd = Command::new("beep");
    for symbol in message {
        if symbol {
            cmd.arg("-f 220").arg("-l 100").arg("-D 100").arg("-n");
        } else {
            cmd.arg("-f 220").arg("-l 300").arg("-D 100").arg("-n");
        }
    }
    cmd.arg("-l 0"); // suppress beep from final "-n"
    cmd
}

fn main() {
    println!("._.");
    let message = vec![true, false, true];
    chain(message.into_iter()).spawn().unwrap();
}
