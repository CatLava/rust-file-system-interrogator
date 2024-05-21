use std::fs;

fn main() {
    // Turn this into a function for reading dir as input
    // Can implement full logging levels with this 
    let inspection = inspect_dir(".");
    println!("found dirs {:?}", inspection);
    // Options read contents of file, grep for key phrase
    // Get hashes of files

    // DONE Get file sizes
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
                                                println!("size of file is {:?}", metadata.len())
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