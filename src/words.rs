use rand;
use rand::Rng;

pub struct WordGenerator {
    characters: Vec<char>,
    min_length: usize,
    max_length: usize,
    dict: Option<Vec<String>>,
    rng: rand::ThreadRng
}

impl Iterator for WordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.dict {
            Some(ref d) => {
                //TODO filter somehow?
                //eg max/min lengths or character set
                Some(get_item(d, &mut self.rng))
            },
            None => {
                let length = self.rng.gen_range(self.min_length, self.max_length + 1);
                let word = (0..length)
                    .map(|_| { get_item(&self.characters, &mut self.rng) })
                    .collect::<String>();
                Some(word)
            }
        }
    }
}

impl WordGenerator {
    pub fn get_n_words(&mut self, n: usize) -> String {
        self.take(n).collect::<Vec<String>>().join(" ")
    }

    pub fn new(characters: &[char], min_length: usize, max_length: usize) -> WordGenerator {
        WordGenerator {
            characters: characters.into_iter().map(|x| *x).collect(),
            min_length: min_length,
            max_length: max_length,
            dict: None,
            rng: rand::thread_rng()
        }
    }

    pub fn new_with_dict(dict: Vec<String>) -> WordGenerator {
        WordGenerator {
            characters: vec![],
            min_length: 0,
            max_length: 0,
            dict: Some(dict),
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
