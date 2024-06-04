mod fetch;
mod parse;

fn main() {
    let words = fetch::get_words().unwrap();
    println!("{}", &words[..100])
}
