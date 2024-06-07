mod fetch;
mod parse;

fn main() {
    let words = fetch::get_words().unwrap();
    let _print_words: Vec<&str> = words.lines().take(100).collect();
    let gdata = parse::parse_gutenburg_data().unwrap();
    println!("{gdata}");
    println!("{}", gdata.len());
}
