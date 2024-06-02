use std::io::Write;

fn create_words(path: std::path::PathBuf) {
    if path.exists() {
        return;
    }

    let url = "https://github.com/dwyl/english-words/raw/master/words_alpha.zip";

    let response = match rq::blocking::get(url) {
        Ok(res) => res,
        Err(_) => unimplemented!(),
    };

    let body = match response.text() {
        Ok(b) => b,
        Err(_) => unimplemented!(),
    };

    std::fs::create_dir("./data").unwrap();

    std::fs::create_dir_all(path.parent().unwrap()).unwrap();

    match std::fs::File::create_new(path) {
        Ok(mut f) => f.write_all(body.as_bytes()).unwrap(),
        Err(_) => (),
    }
}

pub fn get_words() {
    let file: std::path::PathBuf = ["data", "words.txt"].iter().collect();
    create_words(file);
}
