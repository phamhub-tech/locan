DROP TABLE ScanFile;
DROP TABLE ScanFileT;
DROP TABLE ScanFileTraCompletionMarker;
DROP TABLE ScanFileTy;
DROP TABLE ScanFileTyp;
DROP TABLE ScanFileType;
DROP TABLE ScanFileTyperaCompletionMarker;
DROP TABLE ScanFileTypraCompletionMarker;
DROP TABLE ScanFileTyraCompletionMarker;
DROP TABLE ScanResult;
DROP TABLE ScanResults;
ALTER TABLE Project ADD COLUMN id INTEGER NOT NULL PRIMARY KEY DEFAULT 0;
CREATE TABLE Project__butane_tmp (
"uuid" TEXT NOT NULL PRIMARY KEY UNIQUE,
"name" TEXT NOT NULL UNIQUE,
root_dir TEXT NOT NULL,
loc INTEGER,
"files" INTEGER,
scans INTEGER,
created_at TEXT NOT NULL,
updated_at TEXT NOT NULL,
id INTEGER NOT NULL PRIMARY KEY
);
INSERT INTO Project__butane_tmp SELECT "uuid", "name", root_dir, loc, "files", scans, created_at, updated_at, id FROM Project;
DROP TABLE Project;
ALTER TABLE Project__butane_tmp RENAME TO Project;
CREATE TABLE Project__butane_tmp (
"uuid" TEXT NOT NULL PRIMARY KEY UNIQUE,
"name" TEXT NOT NULL UNIQUE,
root_dir TEXT NOT NULL,
loc INTEGER,
"files" INTEGER,
created_at TEXT NOT NULL,
updated_at TEXT NOT NULL,
id INTEGER NOT NULL PRIMARY KEY
);
INSERT INTO Project__butane_tmp SELECT "uuid", "name", root_dir, loc, "files", created_at, updated_at, id FROM Project;
DROP TABLE Project;
ALTER TABLE Project__butane_tmp RENAME TO Project;
CREATE TABLE Project__butane_tmp (
"uuid" TEXT NOT NULL UNIQUE,
"name" TEXT NOT NULL UNIQUE,
root_dir TEXT NOT NULL,
loc INTEGER,
"files" INTEGER,
created_at TEXT NOT NULL,
updated_at TEXT NOT NULL,
id INTEGER NOT NULL PRIMARY KEY
);
INSERT INTO Project__butane_tmp SELECT "uuid", "name", root_dir, loc, "files", created_at, updated_at, id FROM Project;
DROP TABLE Project;
ALTER TABLE Project__butane_tmp RENAME TO Project;
