use std::io;

use serialization::ToFile;

use crate::fetch;
use crate::gutenberg;
use crate::utils;
use crate::utils::types;

mod serialization;

fn word_counts_to_prevalence(
    counts: &types::WordCountMap,
    total_words: usize,
) -> types::WordPrevalenceMap {
    let total = total_words as f32;
    let mut ret = types::WordPrevalenceMap::new();

    for (k, v) in counts.iter() {
        let prevalence = *v as f32 / total;
        ret.insert(k.to_string(), prevalence);
    }

    ret
}

#[allow(dead_code)] // TODO remove
pub fn analyze(dictionary: Option<&types::Dict>) -> io::Result<types::WordPrevalenceMap> {
    let ofile = utils::tempdir::get_child("output.txt");

    // prefetch if ofile exists
    if ofile.exists() {
        return serialization::FromFile::from_file(&ofile);
    }

    // let mut dict = fetch::get_dictionary()?;
    let dict = dictionary.unwrap();
    let mut count_map = types::WordCountMap::new();
    let word_data = gutenberg::get_gutenberg_data()?;

    let word_data_split = word_data.split_whitespace();

    println!("Begin wordcount.");
    let wordcount = word_data_split.clone().count();

    println!("Wordcount: {}", wordcount);

    println!("Begin analysis.");
    for (i, word) in word_data_split.enumerate() {
        if dict.contains(word) {
            match count_map.get_mut(word) {
                Some(r) => *r += 1,
                None => {
                    count_map.insert(word.to_string(), 0usize);
                    ()
                }
            }
        }

        if i % 10_000 == 0 && i > 0 {
            let complete_pct = i as f32 / wordcount as f32;
            crate::utils::print_status_bar(complete_pct)
        }
    }
    println!("");

    let prevalence_map = word_counts_to_prevalence(&count_map, wordcount);

    prevalence_map.to_file(&ofile)?;

    Ok(prevalence_map)
}
