use rand;
use rand::Rng;

struct CharGenerator {
    pub characters: Vec<char>,
    pub rng: rand::ThreadRng
}

impl Iterator for CharGenerator {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        self.rng.choose(&self.characters).map(|x| x.clone())
    }
}

impl CharGenerator {
    pub fn get_n_chars(&mut self, n: usize) -> String {
        self.take(n).collect()
    }
}

pub struct WordGenerator {
    char_gen: CharGenerator,
    length_distribution: Vec<usize>
}

impl Iterator for WordGenerator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let n = *self.char_gen.rng.choose(&self.length_distribution).unwrap();
        Some(self.char_gen.get_n_chars(n))
    }
}

impl WordGenerator {
    pub fn get_n_words(&mut self, n: usize) -> String {
        self.take(n).collect::<Vec<String>>().join(" ")
    }

    pub fn new(characters: Vec<char>, lengths: Vec<usize>) -> WordGenerator {
        let char_gen = CharGenerator { characters: characters, rng: rand::thread_rng() };
        WordGenerator {
            char_gen: char_gen,
            length_distribution: lengths
        }
    }
}
