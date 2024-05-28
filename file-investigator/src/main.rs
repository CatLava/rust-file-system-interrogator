use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Turn this into a function for reading dir as input
    // Can implement full logging levels with this 
    let inspection = inspect_dir(".");
    println!("found dirs {:?}", inspection);
    // Get hashes of files MD5 and SHA 256 

    // recursively inspect each directory found 
}

// Inspect a directory, if file is directory, add to vec and return vec 
// To do make return object a result 
pub fn inspect_dir(file_path: &str) -> Vec<String> {
    let mut found_dirs: Vec<String> = vec![];
    match fs::read_dir(file_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        println!("{:?}", path);
                        let metadata = fs::metadata(path.clone());
                        match metadata {
                            Ok(metadata) => {println!("{:?}", metadata.file_type());
                                            println!("is dir {:?}", metadata.is_dir());
                                            if metadata.is_dir() {
                                                found_dirs.push(path.display().to_string())
                                            } else {
                                                println!("size of file is {:?}", metadata.len());
                                                println!("Printing lines of file");
                                                // TODO gate this with cli input 
                                                read_file_by_line(&path.display().to_string(), &"test".to_string())
                                            }
                                            },
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

    return found_dirs
}

pub fn read_file_by_line(file_path: &str, search_term: &str) {
    if let Ok(file) = File::open(file_path) {
        let mut buf_reader = BufReader::new(file);

        for line in buf_reader.lines() {
            if let Ok(line_str) = line {
                if line_str.contains(search_term){
                    println!("{}", line_str)
                }
            }
        }
    }

}