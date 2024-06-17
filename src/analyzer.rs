use std::io::Write;

use crate::fetch;
use crate::gutenberg;

fn print_status_bar(proportion: f32) {
    const WIDTH: usize = 78;

    let progress_chars = ((WIDTH as f32) * proportion).round() as usize;
    let remaining_chars = WIDTH - progress_chars;

    let prog = "=".repeat(progress_chars);
    let rem = " ".repeat(remaining_chars);
    let bar = format!("[{prog}{rem}]");

    let pct_string = format!("\r{:0>5.2}% complete.", proportion * 100.);

    print!("{pct_string} {bar}");
    std::io::stdout().flush().unwrap();
}

pub fn analyze() -> Option<fetch::WordMap> {
    let mut word_map = fetch::get_words().ok()?;
    let word_data = gutenberg::get_gutenberg_data().ok()?;

    let word_data_split = word_data.split_whitespace().take(100_000_000);

    println!("Begin wordcount.");
    let wordcount: f32 = word_data_split.clone().count() as f32;

    println!("Wordcount: {}", wordcount);

    println!("Begin analysis.");
    for (i, word) in word_data_split.enumerate() {
        if word_map.contains_key(word) {
            let val = word_map.get_mut(word).unwrap();
            *val += 1usize;
            // } else {
            //     println!("Missed a word! {:20}", word);
        }

        if i % 10_000 == 0 && i > 0 {
            let complete_pct = i as f32 / wordcount;
            print_status_bar(complete_pct)
        }
    }

    println!("");

    let mut values: Vec<(&String, &usize)> = word_map.iter().map(|(k, v)| (k, v)).collect();
    values.sort_by(|(_, a), (_, b)| b.cmp(a));

    for (k, v) in values.iter().take(200) {
        let prevalence = **v as f32 / wordcount * 100.;
        println!("word: {:<15} prevalence: {}%", k, prevalence);
    }

    Some(word_map)
}
