CREATE TABLE ScanFile (
id INTEGER NOT NULL PRIMARY KEY,
scan INTEGER NOT NULL,
loc INTEGER NOT NULL,
"files" INTEGER NOT NULL,
file_type TEXT NOT NULL,
FOREIGN KEY (scan) REFERENCES ScanResult(id)
);
CREATE TABLE ScanFileT (
id INTEGER NOT NULL PRIMARY KEY,
scan INTEGER NOT NULL,
loc INTEGER NOT NULL,
"files" INTEGER NOT NULL,
file_type TEXT NOT NULL,
FOREIGN KEY (scan) REFERENCES ScanResult(id)
);
CREATE TABLE ScanFileTraCompletionMarker (
id INTEGER NOT NULL PRIMARY KEY,
scan INTEGER NOT NULL,
loc INTEGER NOT NULL,
"files" INTEGER NOT NULL,
file_type TEXT NOT NULL,
FOREIGN KEY (scan) REFERENCES ScanResult(id)
);
CREATE TABLE ScanFileTy (
id INTEGER NOT NULL PRIMARY KEY,
scan INTEGER NOT NULL,
loc INTEGER NOT NULL,
"files" INTEGER NOT NULL,
file_type TEXT NOT NULL,
FOREIGN KEY (scan) REFERENCES ScanResult(id)
);
CREATE TABLE ScanFileTyp (
id INTEGER NOT NULL PRIMARY KEY,
scan INTEGER NOT NULL,
loc INTEGER NOT NULL,
"files" INTEGER NOT NULL,
file_type TEXT NOT NULL,
FOREIGN KEY (scan) REFERENCES ScanResult(id)
);
CREATE TABLE ScanFileType (
id INTEGER NOT NULL PRIMARY KEY,
scan INTEGER NOT NULL,
file_type TEXT NOT NULL,
loc INTEGER NOT NULL,
"files" INTEGER NOT NULL,
FOREIGN KEY (scan) REFERENCES ScanResult(id)
);
CREATE TABLE ScanFileTyperaCompletionMarker (
id INTEGER NOT NULL PRIMARY KEY,
scan INTEGER NOT NULL,
loc INTEGER NOT NULL,
"files" INTEGER NOT NULL,
file_type TEXT NOT NULL,
FOREIGN KEY (scan) REFERENCES ScanResult(id)
);
CREATE TABLE ScanFileTypraCompletionMarker (
id INTEGER NOT NULL PRIMARY KEY,
scan INTEGER NOT NULL,
loc INTEGER NOT NULL,
"files" INTEGER NOT NULL,
file_type TEXT NOT NULL,
FOREIGN KEY (scan) REFERENCES ScanResult(id)
);
CREATE TABLE ScanFileTyraCompletionMarker (
id INTEGER NOT NULL PRIMARY KEY,
scan INTEGER NOT NULL,
loc INTEGER NOT NULL,
"files" INTEGER NOT NULL,
file_type TEXT NOT NULL,
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
CREATE TABLE ScanResults (
id INTEGER NOT NULL PRIMARY KEY,
project TEXT NOT NULL,
loc INTEGER,
"files" INTEGER,
scanned_at TEXT NOT NULL,
FOREIGN KEY (project) REFERENCES Project("uuid")
);
ALTER TABLE Project ADD COLUMN last_scan TEXT DEFAULT NULL;
ALTER TABLE Project ADD COLUMN scans INTEGER DEFAULT NULL;
CREATE TABLE Project__butane_tmp (
"uuid" TEXT NOT NULL UNIQUE,
"name" TEXT NOT NULL UNIQUE,
loc INTEGER,
"files" INTEGER,
root_dir TEXT NOT NULL,
created_at TEXT NOT NULL,
updated_at TEXT NOT NULL,
last_scan TEXT,
scans INTEGER
);
INSERT INTO Project__butane_tmp SELECT "uuid", "name", loc, "files", root_dir, created_at, updated_at, last_scan, scans FROM Project;
DROP TABLE Project;
ALTER TABLE Project__butane_tmp RENAME TO Project;
CREATE TABLE Project__butane_tmp (
"uuid" TEXT NOT NULL PRIMARY KEY UNIQUE,
"name" TEXT NOT NULL UNIQUE,
loc INTEGER,
"files" INTEGER,
root_dir TEXT NOT NULL,
created_at TEXT NOT NULL,
updated_at TEXT NOT NULL,
last_scan TEXT,
scans INTEGER
);
INSERT INTO Project__butane_tmp SELECT "uuid", "name", loc, "files", root_dir, created_at, updated_at, last_scan, scans FROM Project;
DROP TABLE Project;
ALTER TABLE Project__butane_tmp RENAME TO Project;
