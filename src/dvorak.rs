use std::iter::Extend;

//TODO instead have collection of Key structs and filter them
pub fn home() -> Vec<char> {
    let mut cs = l_index().home;
    cs.extend(r_index().home);
    cs.extend(l_middle().home);
    cs.extend(r_middle().home);
    cs.extend(l_ring().home);
    cs.extend(r_ring().home);
    cs.extend(l_pinky().home);
    cs.extend(r_pinky().home);
    cs
}

pub fn minimal() -> Vec<char> {
   let mut cs = home();
   cs.extend(vec!['p', 'y', 'f', 'g', 'c', 'r', 'l']);
   cs
}

pub struct Finger {
  digits: Vec<char>,
  upper:  Vec<char>,
  home:   Vec<char>,
  lower:  Vec<char>,
}

fn l_pinky() -> Finger {
    Finger {
        digits: vec!['1', '2'],
        upper:  vec!['\'', '"'],
        home:   vec!['a'],
        lower:  vec![':']
    }
}

fn r_pinky() -> Finger {
    Finger {
        digits: vec!['0'],
        upper:  vec!['l', '?'],
        home:   vec!['s'],
        lower:  vec!['z']
    }
}

fn l_ring() -> Finger {
    Finger {
        digits: vec!['2'],
        upper:  vec![','],
        home:   vec!['o'],
        lower:  vec!['q']
    }
}

fn r_ring() -> Finger {
    Finger {
        digits: vec!['9'],
        upper:  vec!['r'],
        home:   vec!['n'],
        lower:  vec!['v']
    }
}

fn l_middle() -> Finger {
    Finger {
        digits: vec!['3'],
        upper:  vec!['.'],
        home:   vec!['e'],
        lower:  vec!['j']
    }
}

fn r_middle() -> Finger {
    Finger {
        digits: vec!['8'],
        upper:  vec!['c'],
        home:   vec!['t'],
        lower:  vec!['w']
    }
}

fn l_index() -> Finger {
    Finger {
        digits: vec!['4', '5'],
        upper:  vec!['p', 'y'],
        home:   vec!['u', 'i'],
        lower:  vec!['k', 'x']
    }
}

fn r_index() -> Finger {
    Finger {
        digits: vec!['6', '7'],
        upper:  vec!['f', 'g'],
        home:   vec!['d', 'h'],
        lower:  vec!['b', 'm']
    }
}
