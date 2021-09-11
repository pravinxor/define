mod utils;
mod word;

use std::env;
use std::process::exit;

fn main() {
    let mut args = env::args();
    args.next();
    let term = match args.next() {
        None => {
            eprintln!("Please supply a term to define\nExiting...");
            exit(-1);
        }
        Some(term) => term,
    };
    dbg!(term);
}
