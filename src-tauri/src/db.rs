use std::{fs, fs::File, sync::Mutex};

use butane::db::{Connection, ConnectionSpec};
use tauri::{AppHandle, Manager};

use crate::butane_migrations;

pub struct DBConnection {
    pub conn: Mutex<Connection>,
}

pub fn establish_connection(handle: &AppHandle) -> DBConnection {
    use butane::migrations::Migrations;

    let path = handle
        .path()
        .app_data_dir()
        .expect("Could not get data directory");
    let db_path = path.join("locan.db");
    if !db_path.exists() {
        if !path.exists() {
            println!("App Data directory not found, creating...");
            fs::create_dir_all(path).expect("Could not create data directory");
            println!("App Data directory created")
        }

        File::create_new(&db_path).expect("DB file could not be created");
        println!("Created database file at {:?}", db_path);
    }

    let mut connection = butane::db::connect(
        &ConnectionSpec::new(
            String::from("sqlite"),
            db_path.into_os_string().into_string().unwrap(),
        ), //&ConnectionSpec::load(".butane/connection.json").unwrap()
    )
    .expect("Could not create database connection");
    let migrations = butane_migrations::get_migrations().unwrap();
    migrations.migrate(&mut connection).unwrap();

    DBConnection {
        conn: Mutex::new(connection),
    }
}
