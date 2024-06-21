mod analyzer;
mod dictionary;
mod fetch;
mod gutenberg;
mod utils;

fn main() {
    // analyzer::analyze();
    let word = "technical";
    let result = dictionary::get_rq(word);
    match result {
        Ok(r) => println!("Definition of {}:\n{}", word, r.definition),
        Err(e) => println!("Definition error: {}", e),
    }
}
