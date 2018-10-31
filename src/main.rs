extern crate clap;
extern crate walkdir;
extern crate rayon;
extern crate colored;

mod matcher;
use clap::{App, Arg};
use walkdir::WalkDir;
use colored::*;

fn main() {
    let matches = App::new("File finder")
        .arg(
            Arg::with_name("NEEDLE")
                .help("The value to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("color")
                .help("whether to display colored matches")
                .long("color")
                .short("c")
        )
        .get_matches();

    let needle = matches.value_of("NEEDLE").expect("needle is required");
    let is_colored = matches.is_present("color");

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
        if is_colored {
            let mut buf = String::new();
            for (index, c) in file.string.char_indices() {
                if file.matches.contains(&index) {
                    print!("{}", buf);
                    print!("{}", c.to_string().black().on_cyan());
                    buf.clear();
                } else {
                    buf.push(c);
                }
            }
            println!("{}", buf);
        } else {
            println!("{}", file.string);
        }
    }
}
