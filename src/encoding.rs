use std::collections::HashMap;

#[derive(Clone)]
pub enum Element {
    Dot,
    Dash,
    CharSpace,
    WordSpace
}
use self::Element::*;

pub struct Character {
    pub elements: Vec<Element>,
    pub name: char,
    pub phonetic: Option<&'static str> // will not exist for prosigns, punctuation, etc
}

impl Character {
    pub fn new(elements: Vec<Element>, name: char, phonetic: &'static str) -> Character {
        Character { elements: elements, name: name, phonetic: Some(phonetic) }
    }
}

pub fn encode(words: &str) -> Vec<Element> {
    let mut characters = HashMap::new();
    characters.insert('a', Character::new(vec![Dot,  Dash], 'a', "Alpha"));
    characters.insert('b', Character::new(vec![Dash, Dot,  Dot, Dot], 'b', "Bravo"));
    characters.insert('c', Character::new(vec![Dash, Dot,  Dash, Dot], 'c', "Charlie"));
    characters.insert('d', Character::new(vec![Dash, Dot,  Dot], 'd', "Delta"));
    characters.insert('e', Character::new(vec![Dot], 'e', "Echo"));
    characters.insert('f', Character::new(vec![Dot,  Dot,  Dash, Dot], 'f', "Foxtrot"));
    characters.insert('g', Character::new(vec![Dash, Dash, Dot], 'g', "Golf"));
    characters.insert('h', Character::new(vec![Dot,  Dot,  Dot,  Dot], 'h', "Hotel"));
    characters.insert('i', Character::new(vec![Dot,  Dot], 'i', "India"));
    characters.insert('j', Character::new(vec![Dot,  Dash, Dash, Dash], 'j', "Juliett"));
    characters.insert('k', Character::new(vec![Dash, Dot,  Dash], 'k', "Kilo"));
    characters.insert('l', Character::new(vec![Dot,  Dash, Dot,  Dot], 'l', "Lima"));
    characters.insert('m', Character::new(vec![Dash, Dash], 'm', "Mike"));
    characters.insert('n', Character::new(vec![Dash, Dot], 'n', "November"));
    characters.insert('o', Character::new(vec![Dash, Dash, Dash], 'o', "Oscar"));
    characters.insert('p', Character::new(vec![Dot,  Dash, Dash, Dot], 'p', "Papa"));
    characters.insert('q', Character::new(vec![Dash, Dash, Dot,  Dash], 'q', "Quebec"));
    characters.insert('r', Character::new(vec![Dot,  Dash, Dot], 'r', "Romeo"));
    characters.insert('s', Character::new(vec![Dot,  Dot,  Dot], 's', "Sierra"));
    characters.insert('t', Character::new(vec![Dash], 't', "Tango"));
    characters.insert('u', Character::new(vec![Dot,  Dot,  Dash], 'u', "Uniform"));
    characters.insert('v', Character::new(vec![Dot,  Dot,  Dot, Dash], 'v', "Victor"));
    characters.insert('w', Character::new(vec![Dot,  Dash, Dash], 'w', "Whiskey"));
    characters.insert('x', Character::new(vec![Dash, Dot,  Dot,  Dash], 'x', "Xray"));
    characters.insert('y', Character::new(vec![Dash, Dot,  Dash, Dash], 'y', "Yankee"));
    characters.insert('z', Character::new(vec![Dash, Dash, Dot,  Dot], 'z', "Zulu"));
    characters.insert('!', Character::new(vec![Dash, Dot,  Dash, Dot,  Dash, Dash], '!', "Bang"));
    characters.insert(',', Character::new(vec![Dash, Dash, Dot,  Dot,  Dash, Dash], ',', "Comma"));
    characters.insert('.', Character::new(vec![Dot,  Dash, Dot,  Dash, Dot,  Dash], '.', "Period"));
    characters.insert('1', Character::new(vec![Dot,  Dash, Dash, Dash, Dash], '1', "One"));
    characters.insert('2', Character::new(vec![Dot,  Dot,  Dash, Dash, Dash], '2', "Two"));
    characters.insert('3', Character::new(vec![Dot,  Dot,  Dot,  Dash, Dash], '3', "Three"));
    characters.insert('4', Character::new(vec![Dot,  Dot,  Dot,  Dot,  Dash], '4', "Four"));
    characters.insert('5', Character::new(vec![Dot,  Dot,  Dot,  Dot,  Dot], '5', "Five"));
    characters.insert('6', Character::new(vec![Dash, Dot,  Dot,  Dot,  Dot], '6', "Six"));
    characters.insert('7', Character::new(vec![Dash, Dash, Dot,  Dot,  Dot], '7', "Seven"));
    characters.insert('8', Character::new(vec![Dash, Dash, Dash, Dot,  Dot], '8', "Eight"));
    characters.insert('9', Character::new(vec![Dash, Dash, Dash, Dash, Dot], '9', "Nine"));
    characters.insert('0', Character::new(vec![Dash, Dash, Dash, Dash, Dash], '0', "Zero"));
    characters.insert('\'', Character::new(vec![Dot, Dash, Dash, Dash, Dash, Dot], '\'', "Apostrophe"));
    characters.insert(' ', Character::new(vec![WordSpace], ' ', ""));

    words.chars().into_iter()
        .filter_map(|c| characters.get(&c))
        .flat_map(|c| {
            let mut elts = c.elements.clone();
            elts.push(CharSpace);
            elts.into_iter()
        })
        .collect()
}
