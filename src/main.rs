use crate::utils::{display_word, get_def};
use std::process::exit;

mod utils;

fn main() {
    let mut args = std::env::args();
    args.next();
    let term = match args.next() {
        None => {
            eprintln!("Please supply a term to define");
            exit(-1)
        }
        Some(term) => term,
    };
    let json = crate::utils::get(&format!(
        "https://api.wordnik.com/v4/word.json/{}/definitions",
        term
    ));
    let words = match json.as_array() {
        None => {
            eprintln!("word '{}' not found!", term);
            exit(404);
        }
        Some(words) => words,
    };
    for word in words {
        if let Ok(def) = get_def(word) {
            display_word(&def.0, &def.1);
            break;
        }
    }
}
