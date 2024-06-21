use std::fs;
use std::io::BufWriter;
use std::io::Write;

use crate::fetch;
use crate::gutenberg;
use crate::utils;

#[allow(dead_code)] // TODO remove
pub fn analyze() -> Option<fetch::WordMap> {
    let mut word_map = fetch::get_words().ok()?;
    let word_data = gutenberg::get_gutenberg_data().ok()?;

    let word_data_split = word_data.split_whitespace();

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
            crate::utils::print_status_bar(complete_pct)
        }
    }

    println!("");

    let mut values: Vec<(&String, &usize)> = word_map.iter().map(|(k, v)| (k, v)).collect();
    values.sort_by(|(a, _), (b, _)| a.len().cmp(&b.len()));
    values.sort_by(|(_, a), (_, b)| b.cmp(a));

    let ofile = utils::get_app_tempdir_child("output.txt");

    let handle = fs::File::create(ofile).unwrap();

    let mut writer = BufWriter::new(handle);

    for (k, v) in values.iter() {
        if **v == 0 {
            continue;
        };
        let prevalence = **v as f32 / wordcount * 100.;
        let line = format!("word: {:<25} prevalence: {:.6}%\n", k, prevalence);
        writer.write(line.as_bytes()).unwrap();
    }

    Some(word_map)
}
