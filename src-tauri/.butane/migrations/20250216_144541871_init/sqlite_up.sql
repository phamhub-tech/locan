CREATE TABLE Project (
"uuid" TEXT NOT NULL PRIMARY KEY UNIQUE,
"name" TEXT NOT NULL UNIQUE,
root_dir TEXT NOT NULL,
loc INTEGER,
"files" INTEGER,
scans INTEGER,
last_scan TEXT,
created_at TEXT NOT NULL,
updated_at TEXT NOT NULL
);
CREATE TABLE ScanFileType (
id INTEGER NOT NULL PRIMARY KEY,
scan INTEGER NOT NULL,
file_type TEXT NOT NULL,
loc INTEGER NOT NULL,
"files" INTEGER NOT NULL,
created_at TEXT NOT NULL,
FOREIGN KEY (scan) REFERENCES ScanResult(id)
);
CREATE TABLE ScanResult (
id INTEGER NOT NULL PRIMARY KEY,
project TEXT NOT NULL,
loc INTEGER NOT NULL,
"files" INTEGER NOT NULL,
scanned_at TEXT NOT NULL,
FOREIGN KEY (project) REFERENCES Project("uuid")
);
CREATE TABLE IF NOT EXISTS butane_migrations (
"name" TEXT NOT NULL PRIMARY KEY
);
