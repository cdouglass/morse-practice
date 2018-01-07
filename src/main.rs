mod audio;
mod encoding;

fn main() {
    let sounds = encoding::encode("hello, world!")
        .into_iter()
        .map(|e| e.to_sound());
    audio::play_sounds(sounds).spawn().unwrap();
}
