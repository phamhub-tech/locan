use std::{fs::File, sync::Mutex};

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
    println!("DB path: {:?}", db_path);
    if !db_path.exists() {
        File::create_new(&db_path).expect("DB file could not be created");
    }

    let mut connection = butane::db::connect(
        &ConnectionSpec::new(
            String::from("sqlite"),
            db_path.into_os_string().into_string().unwrap(),
        ), //&ConnectionSpec::load(".butane/connection.json").unwrap()
    )
    .unwrap();
    let migrations = butane_migrations::get_migrations().unwrap();
    migrations.migrate(&mut connection).unwrap();

    DBConnection {
        conn: Mutex::new(connection),
    }
}
