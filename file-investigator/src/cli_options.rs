// Options for file grep
// getting the hash of a file
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct FileArgs {
    // File to encrypt or decrypt, eventually make this an array
    #[arg(short, long)]
    pub start_file_path: String,

    // Password to encrypt file with
    #[arg(short, long)]
    pub grep_files: Option<String>,

    #[arg(short, long, default_value_t = false)]
    pub compute_hashes: bool,
    
    #[arg(short, long, default_value_t = false)]
    pub match_md5_hash: Option<String>

    #[arg(short, long, default_value_t = false)]
    pub directory_recursive: bool,
}
