use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use super::models::{ScanFile, ScanResult};
use super::types::LocAnalysis;
use super::types::{ScanResponse, ScansResponse};
use super::utils::{file_type_from_file_name, get_extension_from_path};

use butane::{colname, query, AutoPk, DataObject, Error, ForeignKey};
use chrono::Utc;
use file_format::FileFormat;
use ignore::{overrides::OverrideBuilder, WalkBuilder};
use tauri::State;

use crate::{
    api::{ApiError, ApiResponse},
    db::DBConnection,
    projects::models::Project,
    settings::models::AppSettingsManager,
};
use crate::{api_error, api_response};

#[tauri::command]
pub fn get_project_scans(
    db: State<DBConnection>,
    uuid: String,
) -> Result<ApiResponse<ScansResponse>, ApiError> {
    let conn_guard = db.conn.lock().map_err(|e| api_error!(e.to_string()))?;
    let conn = &*conn_guard;

    let scans = query!(ScanResult, project.matches(uuid == { uuid }))
        .order_asc(colname!(ScanResult, scanned_at))
        .load(conn)
        .map_err(|e| api_error!(e.to_string()))?;

    if let Some(latest_scan) = scans.last() {
        let scan_id = latest_scan.id;
        let files = query!(ScanFile, scan.matches(id == { scan_id }))
            .order_desc(colname!(ScanFile, loc))
            .load(conn)
            .map_err(|e| api_error!(e.to_string()))?;
        return Ok(api_response!(ScansResponse { scans, files }));
    }

    Ok(api_response!(ScansResponse {
        scans,
        files: Vec::new()
    }))
}

#[tauri::command]
pub fn scan_project(
    db: State<DBConnection>,
    settings_manager: State<AppSettingsManager>,
    uuid: String,
) -> Result<ApiResponse<ScanResponse>, ApiError> {
    let conn_guard = db.conn.lock().map_err(|e| api_error!(e.to_string()))?;
    let conn = &*conn_guard;
    let mut project = Project::get(conn, uuid).map_err(|e| match e {
        Error::NoSuchObject => api_error!(String::from("Not found")),
        _ => api_error!(e.to_string()),
    })?;

    let root_dir = project.root_dir.clone();
    let mut analysis = HashMap::<String, LocAnalysis>::new();

    let scan_settings = &settings_manager
        .settings
        .lock()
        .expect("Couldn't lock settings")
        .scan;

    let mut builder = WalkBuilder::new(&root_dir);
    let mut builder = builder.standard_filters(false).hidden(true).parents(true);
    if scan_settings.use_gitignore {
        builder = builder.git_ignore(true).git_exclude(true).git_global(true);
    }

    let mut override_builder = OverrideBuilder::new(&root_dir);
    for pattern in &scan_settings.ignore_patterns {
        match override_builder.add(&format!("!{pattern}")) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("An error occured building glob: {pattern}");
                eprintln!("{:?}", e);
            }
        }
    }

    match override_builder.build() {
        Ok(overrides) => {
            builder.overrides(overrides);
        }
        Err(e) => {
            eprintln!("Could not create overrides: ${e}")
        }
    };

    let walker = builder.build();
    println!("Scanning {root_dir}");
    for result in walker {
        match result {
            Ok(entry) => {
                let entry_path = entry.path().to_string_lossy().to_string();
                if entry_path == root_dir {
                    println!("Ignoring root path: {root_dir}");
                    continue;
                }

                if entry
                    .file_type()
                    .expect("File has no path: {entry_path}")
                    .is_dir()
                {
                    continue;
                }

                let ext = get_extension_from_path(&entry_path);
                if let Some(extension) = ext {
                    match analyze_loc_for_file(&entry_path) {
                        Some(result) => match analysis.get_mut(&extension) {
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
            Err(err) => {
                eprintln!("Error ${err}")
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

    let mut scan_files = Vec::<ScanFile>::new();
    for (extension, a) in analysis {
        let mut scan_file = ScanFile {
            id: AutoPk::uninitialized(),
            scan: ForeignKey::from_pk(scan.id),
            file_type: file_type_from_file_name(&extension),
            extension,
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
    project.updated_at = Utc::now();

    scan.save(conn).map_err(|e| api_error!(e.to_string()))?;
    project.save(conn).map_err(|e| api_error!(e.to_string()))?;

    Ok(api_response!(ScanResponse {
        scan,
        files: scan_files,
    }))
}

fn read_lines<P: AsRef<Path>>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

fn analyze_loc_for_file<P: AsRef<Path> + std::fmt::Debug>(file_path: P) -> Option<LocAnalysis> {
    let fmt = match FileFormat::from_file(&file_path) {
        Err(e) => {
            println!("Could not get format: {:?} {:?}", file_path, e);
            return None;
        }
        Ok(fmt) => fmt,
    };

    let format_whitelist = vec![
        FileFormat::HypertextMarkupLanguage,
        FileFormat::PlainText,
        FileFormat::ScalableVectorGraphics,
    ];
    if !format_whitelist.contains(&fmt) {
        println!("Received unsupported file: {fmt} -> {:?}", file_path);
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
