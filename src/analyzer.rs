use std::io;

use serialization::ToFile;

use crate::fetch;
use crate::gutenberg;
use crate::utils;

mod serialization;

fn word_counts_to_prevalence(
    counts: &fetch::WordCountMap,
    total_words: usize,
) -> fetch::WordPrevalenceMap {
    let total = total_words as f32;
    let mut ret = fetch::WordPrevalenceMap::new();

    for (k, v) in counts.iter() {
        let prevalence = *v as f32 / total;
        ret.insert(k.to_string(), prevalence);
    }

    ret
}

#[allow(dead_code)] // TODO remove
pub fn analyze() -> io::Result<fetch::WordPrevalenceMap> {
    let ofile = utils::tempdir::get_child("output.txt");

    // prefetch if ofile exists
    if ofile.exists() {
        return serialization::FromFile::from_file(&ofile);
    }

    let mut count_map = fetch::get_words()?;
    let word_data = gutenberg::get_gutenberg_data()?;

    let word_data_split = word_data.split_whitespace();

    println!("Begin wordcount.");
    let wordcount = word_data_split.clone().count();

    println!("Wordcount: {}", wordcount);

    println!("Begin analysis.");
    for (i, word) in word_data_split.enumerate() {
        if count_map.contains_key(word) {
            let val = count_map.get_mut(word).unwrap();
            *val += 1usize;
            // } else {
            //     println!("Missed a word! {:20}", word);
        }

        if i % 10_000 == 0 && i > 0 {
            let complete_pct = i as f32 / wordcount as f32;
            crate::utils::print_status_bar(complete_pct)
        }
    }
    println!("");

    count_map.retain(|_, &mut v| v != 0);

    let prevalence_map = word_counts_to_prevalence(&count_map, wordcount);

    prevalence_map.to_file(&ofile)?;

    Ok(prevalence_map)
}
