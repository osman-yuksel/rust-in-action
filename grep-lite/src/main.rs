use std::{collections::HashMap, fs::DirEntry, io::BufRead};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The pattern to search for
    #[arg(short = 'p', long = "pattern")]
    pattern: String,

    /// The directory to search
    #[arg(short = 'd', long = "directory", default_value = "<cwd>")]
    directory: String,
}

fn main() {
    let cli = Cli::parse();

    let dir = match cli.directory.as_str() {
        "<cwd>" => std::env::current_dir().unwrap(),
        _ => std::path::PathBuf::from(cli.directory),
    };

    let pattern = match regex::Regex::new(&cli.pattern) {
        Ok(regex) => regex,
        Err(err) => {
            eprintln!("Error compiling regex: {:?}", err);
            std::process::exit(1);
        }
    };

    let files = populate_files(&dir);
    let mut matches = HashMap::new();

    for file in files {
        match search_file(&file, &pattern) {
            Ok(res) => {
                if res.is_empty() {
                    continue;
                }
                matches.insert(file.path(), res);
            }
            Err(_) => continue,
        };
    }

    println!("\n\nTotal {} file(s)\n", matches.len());
    for m in matches {
        println!("File: {:?}", m.0.as_path());
        for line in m.1 {
            println!("\t{}: {}", line.line, line.contents);
        }
    }
}

struct SearchMatch {
    line: usize,
    contents: String,
}

fn search_file(
    dir_entry: &std::fs::DirEntry,
    pattern: &regex::Regex,
) -> Result<Vec<SearchMatch>, std::io::Error> {
    // println!("Searching file {:?}", dir_entry.path());
    let mut matches = Vec::new();
    let file = std::fs::File::open(dir_entry.path())?;

    let reader = std::io::BufReader::new(file);

    let mut current_line = 0;
    for line in reader.lines() {
        if line.is_err() {
            return Err(line.err().unwrap());
        }

        let contents = line.unwrap();

        current_line += 1;
        if pattern.is_match(&contents) {
            matches.push(SearchMatch {
                line: current_line,
                contents: contents.trim().to_string(),
            });
        }
    }
    Ok(matches)
}

fn populate_files(dir: &std::path::PathBuf) -> Vec<DirEntry> {
    let mut files = Vec::new();

    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        match entry.file_type().unwrap().is_dir() {
            true => files.extend(populate_files(&entry.path())),
            false => files.push(entry),
        }
    }

    files
}
