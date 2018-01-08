use rand;
use rand::Rng;

pub struct WordGenerator {
    characters: Vec<char>,
    length_distribution: Vec<usize>,
    rng: rand::ThreadRng
}

impl Iterator for WordGenerator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let word = (0..get_item(&self.length_distribution, &mut self.rng))
            .map(|_| { get_item(&self.characters, &mut self.rng) })
            .collect::<String>();
        Some(word)
    }
}

impl WordGenerator {
    pub fn get_n_words(&mut self, n: usize) -> String {
        self.take(n).collect::<Vec<String>>().join(" ")
    }

    pub fn new(characters: &[char], lengths: &[usize]) -> WordGenerator {
        WordGenerator {
            characters: characters.into_iter().map(|x| *x).collect(),
            rng: rand::thread_rng(),
            length_distribution: lengths.into_iter().map(|x| *x).collect()
        }
    }
}

//TODO do I actually need to hole an rng for this?
//TODO also seems like slices should be fine instead of Vec if I just knew how to make lifetimes
//work
fn get_item<T: Clone>(vals: &Vec<T>, rng: &mut rand::ThreadRng) -> T {
    rng.choose(vals).unwrap().clone()
}
