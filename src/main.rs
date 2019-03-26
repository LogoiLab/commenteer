extern crate clap;
extern crate tempfile;

use std::fs::{File};
use std::io::Write;
use std::path::Path;

fn prepend_file<P: AsRef<Path>>(data: &[u8], file_path: &P) -> std::io::Result<()> {
    println!("1");
    let tmp_file = tempfile::NamedTempFile::new().unwrap();
    println!("1");
    let tmp_path = tmp_file.into_temp_path();;
    println!("1");
    let mut tmp = File::create(&tmp_path).unwrap();
    // Open source file for reading
    println!("1");
    let mut src = File::open(&file_path).unwrap();
    // Write the data to prepend
    println!("1");
    tmp.write_all(&data).unwrap();
    // Copy the rest of the source file
    println!("1");
    std::io::copy(&mut src, &mut tmp).unwrap();
    println!("1");
    std::fs::remove_file(&file_path).unwrap();
    println!("1");
    let mut new_file = File::create(&file_path).unwrap();
    println!("1");
    std::fs::copy(&tmp_path, &file_path).unwrap();
    Ok(())
}

fn main() {
    let file_path = Path::new("./file.txt");
    let data = "Data to add to the beginning of the file\n";
    prepend_file(data.as_bytes(), &file_path).unwrap();
}
