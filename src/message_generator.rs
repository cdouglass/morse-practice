use rand::Rng;

pub fn make_word(chars: &Vec<char>) -> String {
    let mut rng = rand::thread_rng();
    let lengths = [1, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5];
    let mut n = *rng.choose(&lengths).unwrap();

    let mut word = String::new();

    while n > 0 {
        let ch = rng.choose(chars).unwrap();
        word.push(*ch);
        n -= 1;
    }

    word
}

//TODO dry up (this basically repeats make_word)
pub fn make_message(chars: &Vec<char>) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let lengths = [1, 2, 1];
    let mut n = *rng.choose(&lengths).unwrap();

    let mut words = vec![];

    while n > 0 {
        words.push(make_word(chars));
        n -= 1;
    }

    words
}

