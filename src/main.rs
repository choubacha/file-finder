extern crate clap;
extern crate colored;
extern crate crossbeam_channel;
extern crate walkdir;

mod file_stream;
mod score;
use clap::{App, Arg};
use colored::*;
use crossbeam_channel as channel;
use file_stream::{FileStream, Msg};
use std::thread;

fn main() {
    let matches = App::new("File finder")
        .arg(
            Arg::with_name("NEEDLE")
                .help("The value to search for")
                .takes_value(true)
                .required(true),
        ).arg(
            Arg::with_name("PATH")
                .help("The path to start searching at. Defaults to current working directory")
                .takes_value(true),
        ).arg(
            Arg::with_name("no-color")
                .help("disable colored matches")
                .long("no-color")
                .short("c"),
        ).arg(
            Arg::with_name("include-hidden")
                .help("include hidden matches")
                .long("include-hidden")
                .short("h"),
        ).arg(
            Arg::with_name("number")
                .help("the number of matches to return")
                .default_value("10")
                .long("number")
                .short("n")
                .takes_value(true),
        ).get_matches();

    let needle = matches.value_of("NEEDLE").expect("needle is required");
    let is_colored = !matches.is_present("no-color");
    let include_hidden = matches.is_present("include-hidden");
    let number_to_return: usize = matches
        .value_of("number")
        .unwrap_or("10")
        .parse()
        .unwrap_or(10);
    let cwd = std::env::current_dir().expect("Failed to load cwd");
    let root = matches
        .value_of("PATH")
        .unwrap_or(cwd.to_str().expect("should be a string"))
        .to_string();

    let (s, r) = channel::bounded(1024);
    let handle = thread::spawn(move || {
        let mut stream = FileStream::new().start_at(root.to_string());

        if include_hidden {
            stream = stream.with_hidden();
        }

        stream.stream(|msg| s.send(msg));
    });

    let mut matches = Vec::with_capacity(number_to_return);
    while let Some(Msg::File(path)) = r.recv() {
        if let Some(m) = score::calc(needle, &path.clone()) {
            matches.push(m);
        }
    }
    matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    let _ = handle.join().expect("should work");

    for file in matches.iter().take(number_to_return) {
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
