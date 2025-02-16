CREATE TABLE ScanFileType__butane_tmp (
id INTEGER NOT NULL PRIMARY KEY,
scan INTEGER NOT NULL,
file_type TEXT NOT NULL,
loc INTEGER NOT NULL,
"files" INTEGER NOT NULL,
FOREIGN KEY (scan) REFERENCES ScanResult(id)
);
INSERT INTO ScanFileType__butane_tmp SELECT id, scan, file_type, loc, "files" FROM ScanFileType;
DROP TABLE ScanFileType;
ALTER TABLE ScanFileType__butane_tmp RENAME TO ScanFileType;
