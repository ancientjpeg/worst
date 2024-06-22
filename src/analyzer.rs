use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

use regex::Regex;

use crate::fetch;
use crate::gutenberg;
use crate::utils;

#[allow(dead_code)] // TODO remove
pub fn analyze() -> Result<fetch::WordMap, Box<dyn std::error::Error>> {
    let ofile = utils::get_app_tempdir_child("output.txt");

    // prefetch if ofile exists
    if ofile.exists() {
        let mut ret = fetch::WordMap::new();

        let file = fs::File::open(&ofile)?;
        let reader = BufReader::new(file);
        let re = Regex::new(r"word: (\w+)\s+prevalence: (.*)%").unwrap();

        let lines: Vec<_> = reader.lines().collect();

        let wc = lines.len() as f32;

        for l in lines {
            let line = l.unwrap();
            let caps = re.captures(&line).unwrap();
            assert_eq!((&caps).len(), 3usize);

            let prevalence: f32 = caps[2].parse().unwrap();
            let count = (wc * prevalence).round() as usize;
            if count == 0 {
                continue;
            }
            ret.insert(caps[1].to_string(), count);
        }
        return Ok(ret);
    }

    let mut word_map = fetch::get_words()?;
    let word_data = gutenberg::get_gutenberg_data()?;

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

    Ok(word_map)
}
