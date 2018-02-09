//TODO instead have collection of Key structs and filter them
pub fn home() -> Vec<char> {
    all_keys().into_iter()
        .filter(|k| {k.row == Home})
        .map(|k| {k.character})
        .collect()
}

pub fn minimal() -> Vec<char> {
    all_keys().into_iter()
        .filter(|k| {k.row == Home || k.row == Upper})
        .map(|k| {k.character})
        .collect()
}

use self::Column::*;
use self::Row::*;
use self::Side::*;
fn all_keys() -> Vec<Key> {
    let mut keys = vec![];

    keys.push(Key::new('1', Pinky,        Digits, Left));
    keys.push(Key::new('2', Ring,         Digits, Left));
    keys.push(Key::new('3', Middle,       Digits, Left));
    keys.push(Key::new('4', Index,        Digits, Left));
    keys.push(Key::new('5', IndexStretch, Digits, Left));
    keys.push(Key::new('6', IndexStretch, Digits, Right));
    keys.push(Key::new('7', Index,        Digits, Right));
    keys.push(Key::new('8', Middle,       Digits, Right));
    keys.push(Key::new('9', Ring,         Digits, Right));
    keys.push(Key::new('0', Pinky,        Digits, Right));

    // keys.push(Key::new('\'', Pinky,       Upper, Left));
    // keys.push(Key::new(',', Ring,         Upper, Left));
    // keys.push(Key::new('.', Middle,       Upper, Left));
    keys.push(Key::new('p', Index,        Upper, Left));
    keys.push(Key::new('y', IndexStretch, Upper, Left));
    keys.push(Key::new('f', IndexStretch, Upper, Right));
    keys.push(Key::new('g', Index,        Upper, Right));
    keys.push(Key::new('c', Middle,       Upper, Right));
    keys.push(Key::new('r', Ring,         Upper, Right));
    keys.push(Key::new('l', Pinky,        Upper, Right));

    keys.push(Key::new('a', Pinky,        Home, Left));
    keys.push(Key::new('o', Ring,         Home, Left));
    keys.push(Key::new('e', Middle,       Home, Left));
    keys.push(Key::new('u', Index,        Home, Left));
    keys.push(Key::new('i', IndexStretch, Home, Left));
    keys.push(Key::new('d', IndexStretch, Home, Right));
    keys.push(Key::new('h', Index,        Home, Right));
    keys.push(Key::new('t', Middle,       Home, Right));
    keys.push(Key::new('n', Ring,         Home, Right));
    keys.push(Key::new('s', Pinky,        Home, Right));

    //keys.push(Key::new(';', Pinky,        Lower, Left));
    keys.push(Key::new('q', Ring,         Lower, Left));
    keys.push(Key::new('j', Middle,       Lower, Left));
    keys.push(Key::new('k', Index,        Lower, Left));
    keys.push(Key::new('x', IndexStretch, Lower, Left));
    keys.push(Key::new('b', IndexStretch, Lower, Right));
    keys.push(Key::new('m', Index,        Lower, Right));
    keys.push(Key::new('w', Middle,       Lower, Right));
    keys.push(Key::new('v', Ring,         Lower, Right));
    keys.push(Key::new('z', Pinky,        Lower, Right));

    keys
}

struct Key {
    pub character: char,
    pub column: Column,
    pub row: Row,
    pub side: Side,
}

impl Key {
    pub fn new(x: char, c: Column, r: Row, s: Side) -> Key {
        Key {character: x, column: c, row: r, side: s}
    }
}

// TODO: Better name
#[derive(PartialEq)]
enum Column {
    Pinky,
    Ring,
    Middle,
    Index,
    IndexStretch
}

#[derive(PartialEq)]
enum Row {
    Digits,
    Upper,
    Home,
    Lower
}

#[derive(PartialEq)]
enum Side {
    Left,
    Right
}
