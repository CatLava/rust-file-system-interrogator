use std::fs;

fn main() {
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        println!("{:?}", path);
                        let metadata = fs::metadata(path);
                        println!("{:?}", metadata.expect("unable to wrap").file_type())
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
