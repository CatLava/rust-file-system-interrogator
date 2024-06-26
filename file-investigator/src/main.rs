use clap::Parser;
use md5;
use md5::Digest;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

mod cli_options;


// TODO implement log levels for this function

fn main() {
    // Turn this into a function for reading dir as input
    // Can implement full logging levels with this
    let start = Instant::now();
    let file_args: cli_options::FileArgs = cli_options::FileArgs::parse();

    let dir_inpection = file_args.start_file_path;
    let grep_files: GrepOptions = match file_args.grep_files {
        Some(grep) => {
            println!("Grep Options set and will look for term {:?}", grep);
            GrepOptions {
                grep_files: true,
                grep_term: Some(grep),
            }
        }
        None => GrepOptions {
            grep_files: false,
            grep_term: None,
        },
    };
    let display_hash = file_args.compute_hashes;
    let match_hash = match file_args.match_md5_hash {
        Some(hash) => {
            // check if valid hash input
            let valid_hash = verify_md5_hash_input(&hash);
            if valid_hash {
                HashOptions {
                    display_hash: display_hash,
                    match_hash: Some(hash),
                }
            } else {
                println!("invalid hash input for {:?}", hash);
                HashOptions {
                    display_hash: display_hash,
                    match_hash: None,
                }
            }
        }
        None => HashOptions {
            display_hash: display_hash,
            match_hash: None,
        },
    };
    let recursion_flag = file_args.directory_recursive;
    let inspection = inspect_dir(&dir_inpection, &grep_files, &match_hash);
    let mut dir_inspection_ls = inspection.directory_list;
    let mut total_cnt = 0;
    let mut total_file_ls = vec![];
    if recursion_flag {
        while dir_inspection_ls.len() > 0 {
            total_cnt += 1;
            let inspected_item = dir_inspection_ls.pop();
            if let Some(item) = inspected_item {
                println!("Popped item: {}", item);
                total_file_ls.push(item.clone());
                let inpection_again = inspect_dir(&item, &grep_files, &match_hash);
                if inpection_again.directory_list.len() > 0 {
                    dir_inspection_ls.extend(inpection_again.directory_list)
                }
            } else {
                println!("The list was empty");
            }
        }
    }
    // then do a for loop thru the dirs and gather all the information
    // add that information to master tracking list
    // recursion should occur out here instead of inside the function itself, this will allow for master tracking
    println!("total inspection {:?}", total_cnt);
    println!("all files {:?} : ", total_file_ls);
    let end = Instant::now();
    let duration = end - start;

    let milliseconds = duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000;
    println!("Execution time: {} milliseconds", milliseconds);
}

#[derive(Debug)]
pub struct GrepOptions {
    grep_files: bool, // default should be false
    grep_term: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HashOptions {
    display_hash: bool,         // default should be false
    match_hash: Option<String>, // display if there is a match on hash
}

#[derive(Debug)]
pub struct FileCrawlStats {
    file_path: String,
    number_of_files: u16,
    // Total file sizes?
    directory_list: Vec<String>,
    md5_matched_files: Vec<String>,
    //grep_term_match_file: Vec<String>,
}

#[derive(Debug)]
pub struct TotalCrawlStats {
    file_path: String,
    number_of_files: u16,
    directory_list: Vec<String>,
}
// Inspect a directory, if file is directory, add to vec and return vec
// To do make return object a result
pub fn inspect_dir(
    file_path: &str,
    grep_info: &GrepOptions,
    hash_options: &HashOptions,
) -> FileCrawlStats {
    let mut found_dirs: Vec<String> = vec![];
    let mut matched_md5_file: Vec<String> = vec![];
    let mut files_inspected = 0;
    match fs::read_dir(file_path) {
        Ok(entries) => {
            // TODO determine if rayon can paralllize this task and compare in a result
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        // println!("{:?}", path);
                        let metadata = fs::metadata(path.clone());
                        match metadata {
                            Ok(metadata) => {
                                // println!("{:?}", metadata.file_type());
                                // println!("is dir {:?}", metadata.is_dir());
                                if metadata.is_dir() {
                                    found_dirs.push(path.display().to_string())
                                } else {
                                    files_inspected += 1;
                                    println!("size of file is {:?}", metadata.len());
                                    if grep_info.grep_files {
                                        read_file_by_line(
                                            &path.display().to_string(),
                                            &grep_info
                                                .grep_term
                                                .clone()
                                                .expect("already unwraped")
                                                .to_string(),
                                        );
                                    }
                                    if hash_options.display_hash || hash_options.match_hash.is_some() {
                                        let file_hash =
                                            compute_file_hash(&path.display().to_string());
                                        // todo need matching logic
                                        let match_hash = hash_options.match_hash.clone().unwrap_or("defailt".to_string());
                                        if format!("{:?}", file_hash) == match_hash {
                                            // what to do if matched, add as return type 
                                            matched_md5_file.push(path.display().to_string());
                                        }
                                        println!("Hash of file is  {:?}", file_hash);
                                    }
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
    // for directory in found_dirs.iter() {
    //     inspect_dir(&directory);
    // }
    println!("total files inspected {:?}", files_inspected);
    let file_stats = FileCrawlStats {
        file_path: file_path.to_string(),
        number_of_files: files_inspected,
        directory_list: found_dirs,
        md5_matched_files: matched_md5_file
    };
    println!("file stats {:?}", file_stats);

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

pub fn compute_file_hash(file_path: &str) -> Digest {
    let file_contents = fs::read(file_path).expect("Error reading file");
    let hash = md5::compute(&file_contents);
    hash
}

pub fn verify_md5_hash_input(md5_hash_claim: &str) -> bool {
    md5_hash_claim.len() == 32 && md5_hash_claim.chars().all(|c| c.is_ascii_hexdigit())
}
