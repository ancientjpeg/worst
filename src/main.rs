mod analyzer;
mod dictionary;
mod fetch;
mod gutenberg;
mod utils;

use rand::prelude::{thread_rng, Rng};

fn main() {
    let dict = match fetch::get_dictionary() {
        Ok(d) => d,
        Err(e) => {
            println!("Analysis failed due to error: {}", e);
            return;
        }
    };

    let word_rates = match analyzer::analyze(Some(&dict)) {
        Ok(val) => val,
        Err(e) => {
            println!("Analysis failed due to error: {}", e);
            return;
        }
    };

    println!("Contains explicit: {}", dict.contains("explicit"));
    println!("Contains infect: {}", dict.contains("infect"));
    println!("Contains unsuspected: {}", dict.contains("unsuspected"));
    assert!(utils::fitness::word_is_compound("infectiously", &dict, None).is_some());

    let filtered_words = word_rates.iter().filter(|&(w, prevalence)| {
        if (0.0..0.0001).contains(prevalence) && w.len() > 8 {
            return true;
        }

        if w.len() < 8 {
            return false;
        }

        match utils::fitness::word_is_compound(&w, &dict, None) {
            Some(m) => {
                println!("Word {w} is compound with match {m}");
                false
            }
            None => true,
        }
    });

    let filtered_count = filtered_words.clone().count();
    println!("Match count: {filtered_count}");
    for _ in 0..filtered_count {
        let mut r = thread_rng();
        let i: usize = r.gen_range(0..filtered_count);

        let (word, prev) = filtered_words.clone().nth(i).unwrap();
        println!(
            "======== WORD: {:<20}========\nPrevalence {:.6}%",
            word.to_uppercase(),
            *prev * 100.
        );
        let result = dictionary::get_rq(word);

        match result {
            Ok(r) => println!("Definition:\n{}\n", r.definition),
            Err(e) => println!("Definition error: {}", e),
        }
    }
}
