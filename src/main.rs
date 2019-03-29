#[macro_use]
extern crate clap;
extern crate mime_guess;
extern crate tempfile;
extern crate walkdir;

use std::fs::File;
use std::io::prelude::*;
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
             .help("List of paths or files to add a header to.")
             .takes_value(true)
             .multiple(true)
             .required(true))
        .arg(Arg::with_name("comment-file")
             .short("c")
             .long("comment-file")
             .help("A path to a file containing the header you would like to add.")
             .takes_value(true))
        .arg(Arg::with_name("comment-text")
             .short("t")
             .long("comment-text")
             .help("A string containing the header you would like to add. Can contain control characters that use backslashes(\\n, \\t etc).")
             .takes_value(true))
        .arg(Arg::with_name("ignore")
             .short("n")
             .long("ignore")
             .help("Paths to files you would like to exclude from being modified")
             .takes_value(true)
             .multiple(true))
        .arg(Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .help("Show verbose information about the operation"))
        .get_matches();

    let mut data: String = String::new();

    if matches.is_present("comment-file") {
        let mut comment_file = File::open(matches.value_of("comment-file").unwrap()).expect("Unable to open the file");
        comment_file.read_to_string(&mut data).expect("Unable to read the file");
    } else if matches.is_present("comment-text") {
        data = String::from(matches.value_of("comment-text").unwrap());
    } else {
        println!("Please specify either a comment-file or comment-text argument.");
        std::process::exit(128);
    }

    let mut ignore_list: Vec<&str> = vec!();
    if matches.is_present("ignore") {
        ignore_list = matches.values_of("ignore").unwrap().collect();
    }

    let file_paths: Vec<&str> = matches.values_of("input").unwrap().collect();
    if matches.is_present("recurse") {
        for path in file_paths {
            let path: &Path = Path::new(path);
            if path.is_dir() {
                for entry in WalkDir::new(path) {
                    let entry = entry.unwrap();
                    let path_as_str: &str = entry.path().to_str().unwrap();
                    if matches.is_present("verbose") {
                        println!("{}", entry.path().to_str().unwrap());
                    }
                    if !ignore_list.contains(&path_as_str) {
                        let metadata = entry.metadata().expect("metadata call failed");
                        if !metadata.is_dir() {
                            let guess = mime_guess::guess_mime_type(entry.path());
                            if format!("{}", guess).contains("text") {
                                prepend_file(data.as_bytes(), &entry.path()).unwrap();
                            }
                        }
                    }
                }
            } else {
                if matches.is_present("verbose") {
                    println!("{} is a not directory. Ignoring...", path.to_str().unwrap());
                }
            }
        }
    } else if matches.is_present("input") {
        for entry in file_paths {
            let entry: &Path = Path::new(entry);
            if matches.is_present("verbose") {
                println!("{}", entry.to_str().unwrap());
            }
            let guess = mime_guess::guess_mime_type(entry);
            let metadata = entry.metadata().expect("metadata call failed");
            let path_as_str: &str = entry.to_str().unwrap();
            if !ignore_list.contains(&path_as_str) {
                if !metadata.is_dir() {
                    if format!("{}", guess).contains("text") {
                        prepend_file(data.as_bytes(), &entry).unwrap();
                    } else {
                        println!("Given input file was not a text file.");
                    }
                } else {
                    if matches.is_present("verbose") {
                        println!("{} is a directory. Ignoring...", entry.to_str().unwrap());
                    }
                }
            }
        }
    } else {
        println!("Please specify an input or use the recurse flag.");
        std::process::exit(128);
    }
}
