use std::{fs::OpenOptions, io::Write, path::Path};

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    write!(file, "{}", content).unwrap();
}