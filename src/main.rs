mod analyzer;
mod dictionary;
mod fetch;
mod gutenberg;
mod utils;

use rand::prelude::thread_rng;

fn main() {
    let word_rates = match analyzer::analyze() {
        Ok(val) => val,
        Err(e) => {
            println!("Analysis failed due to error: {}", e);
            return;
        }
    };

    let filtered_words = word_rates.iter().filter(|(k, v)| {
        let occ = **v as f32 / word_rates.len() as f32;
        if k.ends_with("ed") {
            return false;
        }
        if k.ends_with("ly") {
            return false;
        }
        if k.ends_with("s") {
            return false;
        }
        return (0.0..0.000001).contains(&occ) && k.len() > 8;
    });

    let filtered_count = filtered_words.clone().count();

    println!("Match count: {filtered_count}");
    for _ in 0..filtered_count {
        let mut r = thread_rng();
        let i: usize = r.gen_range(0..filtered_count);

        let (word, prev) = filtered_words.clone().nth(i).unwrap();
        println!("Chose word {word} with prevalence {:.6}%", *prev * 100.);
        let result = dictionary::get_rq(word);

        match result {
            Ok(r) => println!("Definition of {}:\n{}\n", word, r.definition),
            Err(e) => println!("Definition error: {}", e),
        }
    }
}
