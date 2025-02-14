mod types;
mod utils;

use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use file_format::FileFormat;
use types::LocAnalysis;
use walkdir::WalkDir;

use utils::get_entry_path;

use crate::api::{ApiError, ApiResponse};
use crate::api_response;

fn read_lines<P: AsRef<Path>>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

fn analyze_loc_for_file<P: AsRef<Path>>(file_path: P) -> Option<LocAnalysis> {
    let fmt = match FileFormat::from_file(&file_path) {
        Err(e) => {
            println!("Could not get format: {:?}", e);
            return None;
        }
        Ok(fmt) => fmt,
    };
    if fmt != FileFormat::PlainText {
        println!("Received unsupported file: {fmt}");
        return None;
    }

    if let Ok(lines) = read_lines(&file_path) {
        let mut loc = 0;
        let mut blanks = 0;
        let blank = String::from("");
        for line in lines.map_while(Result::ok) {
            if line.trim() == blank {
                blanks += 1;
            } else {
                loc += 1;
            }
        }

        return Some(LocAnalysis { loc, blanks });
    }

    None
}

#[tauri::command]
pub fn scan_project(root_dir: String) -> Result<ApiResponse<LocAnalysis>, ApiError> {
    let mut walker = WalkDir::new(&root_dir).into_iter();
    let mut analysis = LocAnalysis::default();

    println!("Scanning {root_dir}");
    loop {
        let entry = match walker.next() {
            None => break,
            Some(Err(_)) => continue,
            Some(Ok(e)) => e,
        };
        let entry_path = get_entry_path(&entry);
        if entry_path == root_dir {
            println!("Ignoring root path: {root_dir}");
            continue;
        }

        if entry.file_type().is_dir() {
            print!("Skipping directory: {entry_path}");
            continue;
        }

        match analyze_loc_for_file(entry_path) {
            Some(results) => {
                analysis.loc += results.loc;
                analysis.blanks += results.blanks;
            }
            None => continue,
        }
    }

    Ok(api_response!(analysis))
}
