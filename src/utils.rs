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
