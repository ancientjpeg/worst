mod fetch;
mod gutenburg;

fn main() {
    let words = fetch::get_words().unwrap();
    let _print_words: Vec<&str> = words.lines().take(100).collect();
    let gdata = gutenburg::get_gutenburg_data().unwrap();
    println!("{}", gdata.len());
}
