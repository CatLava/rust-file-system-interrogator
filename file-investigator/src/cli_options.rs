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
    pub grep_files: String,
}