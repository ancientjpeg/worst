use regex::Regex;
use std::{fs, io, path::PathBuf};

fn clean_ebook(mut ebook_txt: String) -> io::Result<String> {
    let begin_str = r"(?mR)^.*(START|END).*PROJECT GUTENBERG.*";

    let re = Regex::new(begin_str).unwrap();

    let proper_start: usize;
    let proper_end: usize;
    {
        let line_iter: Vec<regex::Match> = re.find_iter(&ebook_txt).collect();
        if line_iter.len() > 2 || line_iter.is_empty() {
            eprintln!("bad file len: {}", line_iter.len());
            for l in line_iter {
                eprintln!("{}", l.as_str());
            }

            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Found invalid number of start/end matches",
            ));
        }

        proper_start = line_iter[0].end();
        proper_end = if line_iter.len() == 1 {
            ebook_txt.len()
        } else {
            line_iter[1].start()
        };
    }

    ebook_txt.truncate(proper_end);
    ebook_txt.replace_range(..proper_start, "");

    Ok(ebook_txt)
}

fn get_ebook(path: PathBuf) -> io::Result<String> {
    let full_string = fs::read(&path).and_then(|data| {
        String::from_utf8(data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
    })?;

    let ret_val = clean_ebook(full_string);

    if ret_val.is_err() {
        eprintln!("bad file path: {}", path.to_str().unwrap())
    }

    ret_val
}

pub fn parse_gutenburg_data() -> io::Result<String> {
    let file = PathBuf::from("./gutenberg/data/raw");

    if !file.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Data directory does not exist",
        ));
    }

    let ebooks: Vec<String> = match fs::read_dir(&file) {
        Ok(dir_iter) => dir_iter
            .take(20) // TODO remove once we optimize the concatenator
            .filter_map(|f| f.and_then(|f0| get_ebook(f0.path())).ok())
            .collect(),
        Err(e) => return Err(e),
    };

    Ok(ebooks.iter().flat_map(|s| s.chars()).collect())
}
