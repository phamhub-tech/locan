CREATE TABLE Project__butane_tmp (
id INTEGER NOT NULL PRIMARY KEY,
"uuid" TEXT NOT NULL UNIQUE,
"name" TEXT NOT NULL UNIQUE,
loc INTEGER,
"files" INTEGER,
created_at TEXT NOT NULL,
updated_at TEXT NOT NULL
);
INSERT INTO Project__butane_tmp SELECT id, "uuid", "name", loc, "files", created_at, updated_at FROM Project;
DROP TABLE Project;
ALTER TABLE Project__butane_tmp RENAME TO Project;
CREATE TABLE Project__butane_tmp (
id INTEGER NOT NULL PRIMARY KEY,
"uuid" TEXT NOT NULL UNIQUE,
"name" TEXT NOT NULL UNIQUE,
loc INTEGER,
"files" INTEGER NOT NULL,
created_at TEXT NOT NULL,
updated_at TEXT NOT NULL
);
INSERT INTO Project__butane_tmp SELECT id, "uuid", "name", loc, "files", created_at, updated_at FROM Project;
DROP TABLE Project;
ALTER TABLE Project__butane_tmp RENAME TO Project;
CREATE TABLE Project__butane_tmp (
id INTEGER NOT NULL PRIMARY KEY,
"uuid" TEXT NOT NULL UNIQUE,
"name" TEXT NOT NULL UNIQUE,
loc INTEGER NOT NULL,
"files" INTEGER NOT NULL,
created_at TEXT NOT NULL,
updated_at TEXT NOT NULL
);
INSERT INTO Project__butane_tmp SELECT id, "uuid", "name", loc, "files", created_at, updated_at FROM Project;
DROP TABLE Project;
ALTER TABLE Project__butane_tmp RENAME TO Project;
