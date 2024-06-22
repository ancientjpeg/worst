pub fn get() -> std::path::PathBuf {
    let mut odir = std::env::temp_dir();
    odir.push("worst_rs");
    let _ = std::fs::create_dir_all(&odir);
    odir
}

pub fn get_child(child_name: &str) -> std::path::PathBuf {
    let mut out = get();
    out.push(child_name);
    out
}
