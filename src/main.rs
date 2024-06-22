mod analyzer;
mod dictionary;
mod fetch;
mod gutenberg;
mod utils;

use rand::prelude::*;

fn main() {
    let word_rates = match analyzer::analyze() {
        Ok(val) => val,
        Err(_) => return,
    };

    let mut filtered_words = word_rates.iter().filter(|(_, v)| {
        let occ = **v as f32 / word_rates.len() as f32;
        return occ < 0.001 && occ > 0.0005;
    });

    let filtered_count = filtered_words.clone().count();

    let mut r = thread_rng();
    let i: usize = r.gen_range(0..filtered_count);

    for (w, v) in word_rates.iter() {
        println!("{w}: {v}");
    }

    let word = filtered_words.nth(i).unwrap().0;
    let result = dictionary::get_rq(word);

    match result {
        Ok(r) => println!("Definition of {}:\n{}", word, r.definition),
        Err(e) => println!("Definition error: {}", e),
    }
}
