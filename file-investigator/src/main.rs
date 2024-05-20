use std::fs;

fn main() {
    // Turn this into a function for reading dir as input
    // Can implement full logging levels with this 
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        println!("{:?}", path);
                        let metadata = fs::metadata(path);
                        match metadata {
                            Ok(metadata) => {println!("{:?}", metadata.file_type());
                                            println!("is dir {:?}", metadata.is_dir());
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
}

// Inspect a directory, if file is directory, add to vec and return vec 
pub fn inspect_dir(file_path: str) -> Vec[str] {
    let mut found_dirs: Vec<str> = vec![]
    match fs::read_dir(file_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        println!("{:?}", path);
                        let metadata = fs::metadata(path);
                        match metadata {
                            Ok(metadata) => {println!("{:?}", metadata.file_type());
                                            println!("is dir {:?}", metadata.is_dir());
                                            if metadata.is_dir() {
                                                found_dirs.append(path)
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
}