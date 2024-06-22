use std::io;
use std::io::Write;

pub fn get_app_tempdir() -> std::path::PathBuf {
    let mut odir = std::env::temp_dir();
    odir.push("worst_rs");
    let _ = std::fs::create_dir_all(&odir);
    odir
}

pub fn get_app_tempdir_child(child_name: &str) -> std::path::PathBuf {
    let mut out = get_app_tempdir();
    out.push(child_name);
    out
}

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
