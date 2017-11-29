use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};

const FIG4: &'static [&'static str] = &[
    "atque", "quoque", "neque", "itaque", "absque", "apsque", "abusque", "adaeque", "adusque", "denique",
    "deque", "susque", "oblique", "peraeque", "plenisque", "quandoque", "quisque", "quaeque",
    "cuiusque", "cuique", "quemque", "quamque", "quaque", "quique", "quorumque", "quarumque",
    "quibusque", "quosque", "quasque", "quotusquisque", "quousque", "ubique", "undique", "usque",
    "uterque", "utique", "utroque", "utribique", "torque", "coque", "concoque", "contorque",
    "detorque", "decoque", "excoque", "extorque", "obtorque", "optorque", "retorque", "recoque",
    "attorque", "incoque", "intorque", "praetorque"
];

const FIG6A: &'static [&'static str] = &[
    "ibus", "ius", "ae", "am", "as", "em", "es", "ia",
    "is", "nt", "os", "ud", "um", "us", "a", "e",
    "i", "o", "u"
];

const FIG6B: &'static [&'static str] = &[
    "iuntur", "beris", "erunt", "untur", "iunt", "mini", "ntur", "stis",
    "bor", "ero", "mur", "mus", "ris", "sti", "tis", "tur",
    "unt", "bo", "ns", "nt", "ri", "m", "r", "s", "t"
];

const FIG6B_I: &'static [&'static str] = &[
    "iuntur", "erunt", "untur", "iunt", "unt"
];

const FIG6B_BI: &'static [&'static str] = &[
    "beris", "bor", "bo"
];


fn main() {

    let input = env::args().nth(1).unwrap_or("-".to_string());
    let f: Box<io::Read> = match &*input {
        "-" => Box::new(io::stdin()),
        _   => Box::new(fs::File::open(input).unwrap())
    };

    let file = BufReader::new(f);
    for line in file.lines() {
        let word = line.unwrap();
        let (noun, verb) = schinke(&word);
        println!("{:30}{:25}{}", word, noun, verb);
    }      
}

fn schinke(s: &String) -> (String, String) {

    // Following the Schinke Latin stemming algorithm described in:
    // http://snowball.tartarus.org/otherapps/schinke/intro.html

    // Rule 2
    let s1: String = s.chars().map(|x| match x {
        'j' => 'i',
        'v' => 'u',
        _   => x
    }).collect();

    // Rule 3
    let s3: &str;
    if s1.ends_with("que") {
        if FIG4.contains(&&*s1) {
            return (s1.clone(), s1);
        } else {
            s3 = &s1[..s1.len() - 3];
        }
    } else {
       s3 = &s1;
    }
    
    // Rule 4
    let mut s4: &str = s3;
    for suffix in FIG6A {
        if s3.ends_with(suffix) {
           s4 = &s3[..s3.len() - suffix.len()];
           break
        }
    }

    let mut noun = "".to_string();
    let mut verb = "".to_string();

    // Rule 5
    if s4.len() >= 2 {
        noun += s4;
    } else {
        noun += s3;  // not in algorithm, but matches expected result
    }

    // Rule 6
    for suffix in FIG6B {
        if s3.ends_with(suffix) {
           verb += &s3[..s3.len() - suffix.len()];
           if suffix == &"ero" {
               verb += "eri";
               break
           } else if FIG6B_BI.contains(suffix) {
               verb += "bi";
               break
           } else if FIG6B_I.contains(suffix) {
               verb += "i";
               break
           }
           break;
        }
    }

    if verb.len() < 2 {
        verb.clear();
    }

    if verb.len() == 0 {
        verb += s3;
    }

    (noun, verb)
}
