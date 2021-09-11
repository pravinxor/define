mod utils;
mod word;

fn main() {
    let mut args = std::env::args();
    args.next();
    let term = match args.next() {
        None => {
            eprintln!("Please supply a term to define\nExiting...");
            std::process::exit(-1)
        }
        Some(term) => term,
    };
    let json = crate::utils::get(&format!(
        "https://api.wordnik.com/v4/word.json/{}/definitions",
        term
    ));
    println!("{}", json);
}
