use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use md5;

mod cli_options;

fn main() {
    // Turn this into a function for reading dir as input
    // Can implement full logging levels with this
    let file_args: cli_options::FileArgs = cli_options::FileArgs::parse();

    let dir_inpection = file_args.start_file_path;
    let inspection = inspect_dir(&dir_inpection);
    println!("found dirs {:?}", inspection);
    // recursion should occur out here instead of inside the function itself, this will allow for master tracking 
}

#[derive(Debug)]
pub struct FileCrawlStats{
    file_path: String,
    number_of_files: u16,
    directory_list: Vec<String>
}
// Inspect a directory, if file is directory, add to vec and return vec
// To do make return object a result
pub fn inspect_dir(file_path: &str) -> FileCrawlStats {
    let mut found_dirs: Vec<String> = vec![];
    let mut files_inspected = 0;
    match fs::read_dir(file_path) {
        Ok(entries) => {
            // TODO determine if rayon can paralllize this task and compare in a result
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        println!("{:?}", path);
                        let metadata = fs::metadata(path.clone());
                        match metadata {
                            Ok(metadata) => {
                                println!("{:?}", metadata.file_type());
                                println!("is dir {:?}", metadata.is_dir());
                                if metadata.is_dir() {
                                    found_dirs.push(path.display().to_string())
                                } else {
                                    files_inspected += 1;
                                    println!("size of file is {:?}", metadata.len());
                                    println!("Printing lines of file");
                                    // TODO gate this with cli input
                                    // read_file_by_line(
                                    //     &path.display().to_string(),
                                    //     &"test".to_string(),
                                    // );
                                    compute_file_hash(&path.display().to_string());
                                }
                            }
                            Err(err) => {
                                eprintln!("Error: {}", err);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
    println!("found dirs {:?}", found_dirs);
    // If recursively called flag initiated
    let mut mut_full_list = vec![];
    for directory in found_dirs.iter() {
        println!("{:?}", directory);
        mut_full_list.push(inspect_dir(&directory));
    }
    println!("total files inspected {:?}", files_inspected);
    let file_stats = FileCrawlStats {
        file_path : file_path.to_string(),
        number_of_files : files_inspected,
        directory_list: found_dirs
    };
    println!("file stats {:?}", file_stats);
    println!("full stats {:?}", mut_full_list);

    return file_stats;
}

pub fn read_file_by_line(file_path: &str, search_term: &str) {
    if let Ok(file) = File::open(file_path) {
        let mut buf_reader = BufReader::new(file);

        for line in buf_reader.lines() {
            if let Ok(line_str) = line {
                if line_str.contains(search_term) {
                    println!("{}", line_str)
                }
            }
        }
    }
}

pub fn compute_file_hash(file_path: &str) {
    let file_contents = fs::read(file_path).expect("Error reading file");
    let hash = md5::compute(&file_contents);
    println!("MD5 hash of {} is: {:?}", file_path, hash);
}
