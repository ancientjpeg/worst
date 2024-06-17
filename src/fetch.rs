use crate::utils;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Read;
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

pub fn get_words() -> io::Result<HashMap<String, usize>> {
    let cachefile = utils::get_app_tempdir_child("words.txt");
    create_words(&cachefile)?;

    let mut words: String = Default::default();
    let mut handle = fs::File::open(&cachefile)?;

    handle.read_to_string(&mut words)?;

    let mut map = HashMap::new();

    map.insert("".to_string(), 0usize);

    Ok(map)
}
