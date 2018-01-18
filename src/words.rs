use rand;
use rand::Rng;

pub struct WordGenerator {
    characters: Vec<char>,
    max_length: usize,
    rng: rand::ThreadRng
}

impl Iterator for WordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let length = self.rng.gen_range(2, self.max_length + 1);
        let word = (0..length)
            .map(|_| { get_item(&self.characters, &mut self.rng) })
            .collect::<String>();
        Some(word)
    }
}

impl WordGenerator {
    pub fn get_n_words(&mut self, n: usize) -> String {
        self.take(n).collect::<Vec<String>>().join(" ")
    }

    pub fn new(characters: &[char], max_length: usize) -> WordGenerator {
        WordGenerator {
            characters: characters.into_iter().map(|x| *x).collect(),
            max_length: max_length,
            rng: rand::thread_rng()
        }
    }
}

//TODO do I actually need to hold an rng for this?
//TODO also seems like slices should be fine instead of Vec if I just knew how to make lifetimes
//work
fn get_item<T: Clone>(vals: &Vec<T>, rng: &mut rand::ThreadRng) -> T {
    rng.choose(vals).unwrap().clone()
}

/* Choose word from dictionary instead of creating one randomly */

//TODO trait for behavior shared with WordGenerator?

pub struct WordPicker {
    dict: Vec<String>,
    rng: rand::ThreadRng
}

impl Iterator for WordPicker {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        Some(get_item(&self.dict, &mut self.rng))
    }
}

impl WordPicker {
    pub fn get_n_words(&mut self, n: usize) -> String {
        self.take(n).collect::<Vec<String>>().join(" ")
    }

    //TODO filter somehow?
    //eg max/min lengths or character set
    pub fn new(dict: Vec<String>) -> WordPicker {
        WordPicker { dict: dict, rng: rand::thread_rng() }
    }
}
