use std::io;
use std::io::Write;

pub mod fitness;
pub mod tempdir;
pub mod types;

pub fn print_status_bar(proportion: f32) {
    const WIDTH: usize = 60;

    let progress_chars = ((WIDTH as f32) * proportion).round() as usize;
    let remaining_chars = WIDTH - progress_chars;

    let prog = "=".repeat(progress_chars);
    let rem = " ".repeat(remaining_chars);
    let bar = format!("[{prog}{rem}]");

    let pct_string = format!("{:0>5.2}% complete.", proportion * 100.);

    print!("\r{pct_string}{bar}");
    std::io::stdout().flush().unwrap();
}

pub fn make_io_error<T>(msg: T) -> io::Error
where
    T: ToString,
{
    return io::Error::new(io::ErrorKind::InvalidData, msg.to_string());
}
