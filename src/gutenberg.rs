use regex::Regex;
use std::{
    fs,
    io::{self, BufRead},
    path::PathBuf,
};
use crate::utils;

fn get_text_matcher() -> regex::Regex {
    let begin_str = r"(?mR)^.*(START|END).*PROJECT GUTENBERG.*";
    Regex::new(begin_str).unwrap()
}

fn line_filter<'a>(line: &'a str) -> impl Iterator<Item = char> + 'a {
    let cond = |c:&char| c.is_ascii_alphabetic() || c.is_ascii_whitespace(); 
    line.chars().filter(cond).map(|c| c.to_ascii_lowercase())
}

fn get_ebook(path: PathBuf, buffer: &mut String) -> io::Result<()> {
    let re = get_text_matcher();

    let handle = fs::File::open(&path)?;
    let _ = handle
        .metadata()
        .and_then(|md| Ok(buffer.reserve(buffer.len() + md.len() as usize)));

    let reader = io::BufReader::new(handle);

    let mut reading: bool = false;
    let mut match_count: i32 = 0;

    for line_res in reader.lines() {
        let line = line_res?;

        if re.is_match(&line) {
            match_count += 1;
            if reading {
                break;
            } else {
                reading = true;
                continue;
            }
        }

        if reading {
            buffer.extend(line_filter(&line));
            buffer.push_str("\n");
        }
    }

    if match_count != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Improper match count for gutenberg text.",
        ));
    }

    Ok(())
}

pub fn get_gutenberg_data() -> io::Result<String> {
    let cachefile = utils::get_app_tempdir_child("text.txt");
    if cachefile.exists() {
        println!("Cache for words already existed.");
        return fs::read_to_string(cachefile);
    }

    /* TODO: get gutenburg data procedurally */
    let file = PathBuf::from("./gutenberg/data/raw");

    if !file.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Data directory does not exist",
        ));
    }

    let mut buffer = String::new();
    let valid_files = fs::read_dir(&file)?.filter_map(|f| f.ok());

    let ext_check =
        |f: &fs::DirEntry| f.path().extension().and_then(|oss| oss.to_str()) == Some("txt");
    let txt_files = valid_files.filter(ext_check);

    for file in txt_files {
        let safe_len = buffer.len();
        let res = get_ebook(file.path(), &mut buffer);
        if res.is_err() {
            buffer.truncate(safe_len);
            eprintln!(
                "Error reading {}: {}",
                file.path().to_str().unwrap(),
                res.err().unwrap()
            );
        }
    }

    if cfg!(debug_assertions) {
        if get_text_matcher().is_match(&buffer) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Processing failed: metadata was not properly stripped.",
            ));
        }
    }


    fs::write(cachefile, buffer.as_bytes())?;
    Ok(buffer)
}
