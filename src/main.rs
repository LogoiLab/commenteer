#[macro_use]
extern crate clap;
extern crate mime_guess;
extern crate tempfile;
extern crate walkdir;

use std::fs::{File};
use std::io::Write;
use std::path::Path;

use clap::{App, Arg};
use walkdir::WalkDir;

fn prepend_file<P: AsRef<Path>>(data: &[u8], file_path: &P) -> std::io::Result<()> {
    let tmp_file = tempfile::NamedTempFile::new().unwrap();
    let tmp_path = tmp_file.into_temp_path();;
    let mut tmp = File::create(&tmp_path).unwrap();
    let mut src = File::open(&file_path).unwrap();
    tmp.write_all(&data).unwrap();
    std::io::copy(&mut src, &mut tmp).unwrap();
    std::fs::remove_file(&file_path).unwrap();
    let _ = File::create(&file_path).unwrap();
    std::fs::copy(&tmp_path, &file_path).unwrap();
    Ok(())
}

fn main() {
    let matches = App::new("Commenteer")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Add comment headers to code.")
        .arg(Arg::with_name("recurse")
             .short("r")
             .long("recurse")
             .help("Add header to every text file in the specified path.")
             .takes_value(false))
        .arg(Arg::with_name("input")
             .short("i")
             .long("input")
             .help("List of paths or files to add a header to")
             .takes_value(true)
             .multiple(false)
             .required(true))
        .arg(Arg::with_name("comment")
             .short("c")
             .long("comment")
             .help("A path to a file containing the header you would like to add.")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .help("Show verbose information about the operation"))
        .get_matches();

    let data = "/*╭──────────────────────╮\n  │ Author: Chad Baxter  │\n  │ Date:                │\n  │ For:                 │\n  ╰──────────────────────╯*/\n";

    if matches.is_present("recurse") {
        for entry in WalkDir::new(matches.value_of("input").unwrap()) {
            let entry = entry.unwrap();
            let metadata = entry.metadata().expect("metadata call failed");
            if !metadata.is_dir() {
                let guess = mime_guess::guess_mime_type(entry.path());
                if format!("{}", guess).contains("text") {
                    prepend_file(data.as_bytes(), &entry.path()).unwrap();
                }
            }
        }
    } else {
        let file_path = matches.value_of("input").unwrap();
        let guess = mime_guess::guess_mime_type(file_path);
        if format!("{}", guess).contains("text") {
            prepend_file(data.as_bytes(), &file_path).unwrap();
        } else {
            println!("Given input file was not a text file.");
        }
    }
}
