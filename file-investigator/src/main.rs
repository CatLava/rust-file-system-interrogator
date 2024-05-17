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
