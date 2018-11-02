extern crate clap;
extern crate walkdir;
extern crate colored;
extern crate crossbeam_channel;

mod matcher;
mod file_stream;
use clap::{App, Arg};
use colored::*;
use crossbeam_channel as channel;
use std::thread;
use file_stream::{Msg, FileStream};

fn main() {
    let matches = App::new("File finder")
        .arg(
            Arg::with_name("NEEDLE")
                .help("The value to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("no-color")
                .help("disable colored matches")
                .long("no-color")
                .short("c")
        )
        .arg(
            Arg::with_name("include-hidden")
                .help("include hidden matches")
                .long("include-hidden")
                .short("h")
        )
        .get_matches();

    let needle = matches.value_of("NEEDLE").expect("needle is required");
    let is_colored = !matches.is_present("no-color");
    let include_hidden = matches.is_present("include-hidden");

    let (s, r) = channel::bounded(1024);
    let handle = thread::spawn(move || {
        let mut stream = FileStream::new();

        if include_hidden { stream = stream.with_hidden(); }

        stream.stream(|msg| s.send(msg));
    });

    let mut matches = Vec::with_capacity(50);
    while let Some(Msg::File(path)) = r.recv() {
        if let Some(m) = matcher::score(needle, &path.clone()) {
            matches.push(m);
        }
    }
    matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    let _ = handle.join().expect("should work");

    for file in matches {
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
