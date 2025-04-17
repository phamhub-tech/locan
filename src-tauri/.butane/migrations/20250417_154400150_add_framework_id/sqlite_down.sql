CREATE TABLE Project__butane_tmp (
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
INSERT INTO Project__butane_tmp SELECT "uuid", "name", root_dir, loc, "files", scans, last_scan, created_at, updated_at FROM Project;
DROP TABLE Project;
ALTER TABLE Project__butane_tmp RENAME TO Project;
