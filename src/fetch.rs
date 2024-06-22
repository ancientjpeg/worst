use crate::utils;
use crate::utils::types;

use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;

fn create_words(path: &std::path::PathBuf) -> io::Result<()> {
    if path.exists() {
        return Ok(());
    }

    const DATA_URL: &str = "https://github.com/dwyl/english-words/raw/master/words_alpha.txt";

    let body = match rq::blocking::get(DATA_URL).and_then(|res| res.text()) {
        Ok(res) => res,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string().clone())),
    };

    let parent = path.parent().unwrap();
    std::fs::create_dir_all(parent)?;

    match fs::File::create_new(path) {
        Ok(mut f) => Ok(f.write_all(body.as_bytes()).unwrap()),
        Err(e) => Err(e),
    }
}

fn valid_word(word: &str) -> bool {
    const VALID_SINGLE_LETTER_WORDS: [&'static str; 2] = ["a", "i"];
    if word.len() == 1 {
        return VALID_SINGLE_LETTER_WORDS.contains(&word);
    }
    const INVALID_WORDS: [&'static str; 1] = ["ve"];
    return !INVALID_WORDS.contains(&word);
}

pub fn get_words() -> io::Result<types::WordCountMap> {
    let cachefile = utils::tempdir::get_child("words.txt");
    create_words(&cachefile)?;

    let handle = fs::File::open(&cachefile)?;
    let reader = io::BufReader::new(handle);

    let mut map = types::WordCountMap::new();

    for line in reader.lines() {
        let l = line?;

        if !valid_word(&l) {
            continue;
        };

        map.insert(l, 0usize);
    }

    Ok(map)
}
