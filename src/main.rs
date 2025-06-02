use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rcat <file_path>");
        return;
    }

    let input_path = &args[1];

    let mut file_name: PathBuf = PathBuf::new();

    match std::env::current_dir() {
        Ok(path) => {
            file_name = path;
            file_name.push(input_path);
        }
        Err(err) => eprint!("{}", err),
    }

    let handle_file = std::fs::read(file_name);

    match handle_file {
        Ok(body) => {
            if let Err(err) = io::stdout().write_all(&body) {
                eprintln!("{}", err);
            }
        }
        Err(err) => {
            eprintln!("Cant open file: {}", err);
        }
    }
}
