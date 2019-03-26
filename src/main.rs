extern crate clap;
extern crate mime_guess;
extern crate tempfile;
extern crate walkdir;

use std::fs::{File};
use std::io::Write;
use std::path::Path;

use walkdir::WalkDir;

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
    let _ = File::create(&file_path).unwrap();
    println!("1");
    std::fs::copy(&tmp_path, &file_path).unwrap();
    Ok(())
}

fn main() {
    for entry in WalkDir::new("./") {
        let entry = entry.unwrap();
        let metadata = entry.metadata().expect("metadata call failed");
        if !metadata.is_dir() {
            let guess = mime_guess::guess_mime_type(entry.path());
            if format!("{}", guess).contains("text") {
                println!("{:?}", entry);
            }
        }
    }
    let file_path = Path::new("./file.txt");
    let data = "Data to add to the beginning of the file\n";
    prepend_file(data.as_bytes(), &file_path).unwrap();
}
