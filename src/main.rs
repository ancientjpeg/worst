mod fetch;
mod parse;
extern crate std;

fn main() -> std::io::Result<()> {
    use std::fs;
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        match path.to_str() {
            Some(p) => println!("{p}"),
            None => (),
        }
    }
    println!("Hello world!");

    Ok(())
}
