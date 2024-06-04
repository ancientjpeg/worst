use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

fn create_words(path: &std::path::PathBuf) -> io::Result<()> {
    if path.exists() {
        return Ok(());
    }

    const DATA_URL: &str = "https://github.com/dwyl/english-words/raw/master/words_alpha.zip";

    let body = match rq::blocking::get(DATA_URL).and_then(|res| res.text()) {
        Ok(res) => res,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string().clone())),
    };

    fs::create_dir(path).unwrap();
    path.parent().map(|p| std::fs::create_dir_all(p).unwrap());

    match fs::File::create_new(path) {
        Ok(mut f) => Ok(f.write_all(body.as_bytes()).unwrap()),
        Err(e) => Err(e),
    }
}

pub fn get_words() -> io::Result<String> {
    let file: std::path::PathBuf = ["data", "words.txt"].iter().collect();
    create_words(&file)?;

    let mut words: String = Default::default();
    let mut handle = fs::File::open(&file)?;

    return handle.read_to_string(&mut words).map(|_| words);
}
