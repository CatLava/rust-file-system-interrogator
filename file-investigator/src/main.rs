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
    let grep_files = match file_args.grep_files {
        Some(grep) => GrepOptions {grep_files: true, grep_term: Some(grep)},
        None => GrepOptions {grep_files: false, grep_term: None}
    };
    let recursion_flag = file_args.directory_recursive;
    println!("Grep files {:?}", grep_files);
    let inspection = inspect_dir(&dir_inpection);
    let mut dir_inspection_ls = inspection.directory_list;
    let mut total_cnt = 0;
    let mut total_file_ls = vec![];
    // for directory in inspection.directory_list.iter() {
    //     println!("Futther inspections");
    //     total_cnt +=1;
    //     inspect_dir(&directory);
    // }
    if recursion_flag {
        while dir_inspection_ls.len() > 0 {
            total_cnt +=1;
            let inspected_item = dir_inspection_ls.pop();
            if let Some(item) = inspected_item {
                println!("Popped item: {}", item);
                total_file_ls.push(item.clone());
                let inpection_again = inspect_dir(&item);
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
    println!("all files {:?} : ", total_file_ls)
}

#[derive(Debug)]
pub struct GrepOptions {
    grep_files: bool, // default should be false
    grep_term: Option<String>
}
 #[derive(Debug)]
pub struct FileCrawlStats{
    file_path: String,
    number_of_files: u16,
    // Total file sizes? 
    directory_list: Vec<String>
}

#[derive(Debug)]
pub struct TotalCrawlStats{
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
    // for directory in found_dirs.iter() {
    //     inspect_dir(&directory);
    // }
    println!("total files inspected {:?}", files_inspected);
    let file_stats = FileCrawlStats {
        file_path : file_path.to_string(),
        number_of_files : files_inspected,
        directory_list: found_dirs
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

pub fn compute_file_hash(file_path: &str) {
    let file_contents = fs::read(file_path).expect("Error reading file");
    let hash = md5::compute(&file_contents);
    println!("MD5 hash of {} is: {:?}", file_path, hash);
}
