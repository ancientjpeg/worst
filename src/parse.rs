use regex::Regex;
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

fn clean_ebook(ebook_txt: &mut String, section_start: usize, section_end: usize) -> io::Result<()> {
    let begin_str = r"(?mR)^.*(START|END).*PROJECT GUTENBERG.*";

    let re = Regex::new(begin_str).unwrap();

    let proper_start: usize;
    let proper_end: usize;
    {
        let line_iter: Vec<regex::Match> = re.find_iter(ebook_txt).collect();
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

    ebook_txt.replace_range(proper_end..section_end, "");
    ebook_txt.replace_range(section_start..proper_start, "");

    Ok(())
}

fn get_ebook(path: PathBuf, buffer: &mut String) -> io::Result<()> {
    let begin_len = buffer.len();

    let mut handle = fs::OpenOptions::new().read(true).open(&path)?;
    let _bytes_read = handle.read_to_string(buffer);

    let end_len = buffer.len();

    clean_ebook(buffer, begin_len, end_len)
}

pub fn parse_gutenburg_data() -> io::Result<String> {
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
        println!("{}", file.path().display());
        get_ebook(file.path(), &mut buffer)?;
    }

    Ok(buffer)
}
