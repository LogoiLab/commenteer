extern crate clap;
extern crate tempfile;

use std::fs::{File};
use std::io::{Write, Read, Seek, SeekFrom};
use std::path::Path;

fn prepend_file<P: AsRef<Path>>(data: &[u8], file_path: &P) -> std::io::Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let tmp_path = dir.path().join("tmpfile");
    let mut tmp = File::create(&tmp_path).unwrap();
    // Open source file for reading
    let mut src = File::open(&file_path).unwrap();
    // Write the data to prepend
    tmp.write_all(&data).unwrap();
    // Copy the rest of the source file
    std::io::copy(&mut src, &mut tmp).unwrap();
    std::fs::remove_file(&file_path).unwrap();
    std::fs::rename(&tmp_path, &file_path).unwrap();
    Ok(())
}

fn main() {
    let file_path = Path::new("file.txt");
    let data = "Data to add to the beginning of the file\n";
    prepend_file(data.as_bytes(), &file_path).unwrap();
}
