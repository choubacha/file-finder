extern crate clap;
extern crate walkdir;

mod matcher;
use clap::{App, Arg};
use walkdir::WalkDir;

fn main() {
    let matches = App::new("File finder")
        .arg(
            Arg::with_name("NEEDLE")
                .help("The value to search for")
                .takes_value(true)
                .required(true),
        ).get_matches();

    let needle = matches.value_of("NEEDLE").expect("needle is required");

    let mut files: Vec<String> = Vec::with_capacity(1000);
    for entry in WalkDir::new(".") {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if let Some(path) = path.to_str() {
                    files.push(path.to_owned());
                }
            }
            Err(_) => { /* no op */ }
        }
    }

    let files = matcher::find(needle, &files);
    for file in files {
        println!("{}", file.string);
    }
}
