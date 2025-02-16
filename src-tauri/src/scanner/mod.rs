mod models;
mod types;
mod utils;

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use butane::{AutoPk, DataObject, Error, ForeignKey};
use chrono::Utc;
use file_format::FileFormat;
use models::{ScanFileType, ScanResult};
use tauri::State;
use types::LocAnalysis;
use walkdir::WalkDir;

use utils::get_entry_path;

use crate::{
    api::{ApiError, ApiResponse},
    db::DBConnection,
    projects::models::Project,
};
use crate::{api_error, api_response};

use self::types::ScanResponse;

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

        return Some(LocAnalysis {
            loc,
            blanks,
            files: 1,
        });
    }

    None
}

#[tauri::command]
pub fn scan_project(
    db: State<DBConnection>,
    uuid: String,
) -> Result<ApiResponse<ScanResponse>, ApiError> {
    let conn_guard = db.conn.lock().map_err(|e| api_error!(e.to_string()))?;
    let conn = &*conn_guard;

    let mut project = Project::get(conn, uuid).map_err(|e| match e {
        Error::NoSuchObject => api_error!(String::from("Not found")),
        _ => api_error!(e.to_string()),
    })?;

    let root_dir = project.root_dir.clone();
    let mut walker = WalkDir::new(&root_dir).into_iter();
    let mut analysis = HashMap::<String, LocAnalysis>::new();

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
            println!("Skipping directory: {entry_path}");
            continue;
        }

        let ext = entry_path.split(".").last();
        if let Some(extension) = ext {
            match analyze_loc_for_file(&entry_path) {
                Some(result) => match analysis.get_mut(extension) {
                    Some(a) => {
                        a.files += result.files;
                        a.loc += result.loc;
                        a.blanks += result.blanks;
                    }
                    None => {
                        analysis.insert(extension.to_string(), result);
                    }
                },
                None => continue,
            }
        }
    }

    println!("Scan completed");
    let mut scan = ScanResult {
        id: AutoPk::uninitialized(),
        project: ForeignKey::from_pk(project.uuid.clone()),
        loc: 0,
        files: 0,
        scanned_at: Utc::now(),
    };
    // Initialize the scan id
    scan.save(conn).map_err(|e| api_error!(e.to_string()))?;

    let mut scan_files = Vec::<ScanFileType>::new();
    for (extension, a) in analysis {
        let mut scan_file = ScanFileType {
            id: AutoPk::uninitialized(),
            scan: ForeignKey::from_pk(scan.id),
            file_type: extension,
            loc: a.loc,
            files: a.files as i16,
            created_at: scan.scanned_at,
        };

        scan.loc += &scan_file.loc;
        scan.files += &scan_file.files;

        if let Err(e) = scan_file.save(conn) {
            eprintln!("Error: {}", e);
            eprintln!("Could not save scan_file: {:?}", scan_file);
            continue;
        }

        scan_files.push(scan_file);
    }

    project.last_scan = Some(scan.scanned_at);
    project.scans = Some(project.scans.unwrap_or(0) + 1);
    project.loc = Some(scan.loc);
    project.files = Some(scan.files);

    scan.save(conn).map_err(|e| api_error!(e.to_string()))?;
    project.save(conn).map_err(|e| api_error!(e.to_string()))?;

    Ok(api_response!(ScanResponse {
        scan,
        files: scan_files,
    }))
}
