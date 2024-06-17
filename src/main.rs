mod fetch;
mod gutenburg;

fn main() {
    let gdata = gutenburg::get_gutenburg_data().unwrap();
    println!("{}", gdata.len());
}
