//! Butane migrations embedded in Rust.

use butane::migrations::MemMigrations;

/// Load the butane migrations embedded in Rust.
pub fn get_migrations() -> Result<MemMigrations, butane::Error> {
    let json = r#"{
  "migrations": {
    "20250213_205624544_init": {
      "name": "20250213_205624544_init",
      "db": {
        "tables": {
          "Project": {
            "name": "Project",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "uuid",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": true,
                "default": null
              },
              {
                "name": "name",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": true,
                "default": null
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "created_at",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "updated_at",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          }
        },
        "extra_types": {}
      },
      "from": null,
      "up": {
        "sqlite": "CREATE TABLE Project (\nid INTEGER NOT NULL PRIMARY KEY,\n\"uuid\" TEXT NOT NULL UNIQUE,\n\"name\" TEXT NOT NULL UNIQUE,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\ncreated_at TEXT NOT NULL,\nupdated_at TEXT NOT NULL\n);\nCREATE TABLE IF NOT EXISTS butane_migrations (\n\"name\" TEXT NOT NULL PRIMARY KEY\n);\n"
      },
      "down": {
        "sqlite": "DROP TABLE Project;\n"
      }
    },
    "20250213_232206584_root_dir": {
      "name": "20250213_232206584_root_dir",
      "db": {
        "tables": {
          "Project": {
            "name": "Project",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "uuid",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": true,
                "default": null
              },
              {
                "name": "name",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": true,
                "default": null
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "root_dir",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "created_at",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "updated_at",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          }
        },
        "extra_types": {}
      },
      "from": "20250213_205624544_init",
      "up": {
        "sqlite": "ALTER TABLE Project ADD COLUMN root_dir TEXT NOT NULL DEFAULT '';\nCREATE TABLE Project__butane_tmp (\nid INTEGER NOT NULL PRIMARY KEY,\n\"uuid\" TEXT NOT NULL UNIQUE,\n\"name\" TEXT NOT NULL UNIQUE,\nloc INTEGER NOT NULL,\n\"files\" INTEGER,\ncreated_at TEXT NOT NULL,\nupdated_at TEXT NOT NULL,\nroot_dir TEXT NOT NULL\n);\nINSERT INTO Project__butane_tmp SELECT id, \"uuid\", \"name\", loc, \"files\", created_at, updated_at, root_dir FROM Project;\nDROP TABLE Project;\nALTER TABLE Project__butane_tmp RENAME TO Project;\nCREATE TABLE Project__butane_tmp (\nid INTEGER NOT NULL PRIMARY KEY,\n\"uuid\" TEXT NOT NULL UNIQUE,\n\"name\" TEXT NOT NULL UNIQUE,\nloc INTEGER,\n\"files\" INTEGER,\ncreated_at TEXT NOT NULL,\nupdated_at TEXT NOT NULL,\nroot_dir TEXT NOT NULL\n);\nINSERT INTO Project__butane_tmp SELECT id, \"uuid\", \"name\", loc, \"files\", created_at, updated_at, root_dir FROM Project;\nDROP TABLE Project;\nALTER TABLE Project__butane_tmp RENAME TO Project;\n"
      },
      "down": {
        "sqlite": "CREATE TABLE Project__butane_tmp (\nid INTEGER NOT NULL PRIMARY KEY,\n\"uuid\" TEXT NOT NULL UNIQUE,\n\"name\" TEXT NOT NULL UNIQUE,\nloc INTEGER,\n\"files\" INTEGER,\ncreated_at TEXT NOT NULL,\nupdated_at TEXT NOT NULL\n);\nINSERT INTO Project__butane_tmp SELECT id, \"uuid\", \"name\", loc, \"files\", created_at, updated_at FROM Project;\nDROP TABLE Project;\nALTER TABLE Project__butane_tmp RENAME TO Project;\nCREATE TABLE Project__butane_tmp (\nid INTEGER NOT NULL PRIMARY KEY,\n\"uuid\" TEXT NOT NULL UNIQUE,\n\"name\" TEXT NOT NULL UNIQUE,\nloc INTEGER,\n\"files\" INTEGER NOT NULL,\ncreated_at TEXT NOT NULL,\nupdated_at TEXT NOT NULL\n);\nINSERT INTO Project__butane_tmp SELECT id, \"uuid\", \"name\", loc, \"files\", created_at, updated_at FROM Project;\nDROP TABLE Project;\nALTER TABLE Project__butane_tmp RENAME TO Project;\nCREATE TABLE Project__butane_tmp (\nid INTEGER NOT NULL PRIMARY KEY,\n\"uuid\" TEXT NOT NULL UNIQUE,\n\"name\" TEXT NOT NULL UNIQUE,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\ncreated_at TEXT NOT NULL,\nupdated_at TEXT NOT NULL\n);\nINSERT INTO Project__butane_tmp SELECT id, \"uuid\", \"name\", loc, \"files\", created_at, updated_at FROM Project;\nDROP TABLE Project;\nALTER TABLE Project__butane_tmp RENAME TO Project;\n"
      }
    },
    "20250216_120509395_scan_results": {
      "name": "20250216_120509395_scan_results",
      "db": {
        "tables": {
          "Project": {
            "name": "Project",
            "columns": [
              {
                "name": "uuid",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": false,
                "unique": true,
                "default": null
              },
              {
                "name": "name",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": true,
                "default": null
              },
              {
                "name": "root_dir",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "scans",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "last_scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "created_at",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "updated_at",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFile": {
            "name": "ScanFile",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileT": {
            "name": "ScanFileT",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileTraCompletionMarker": {
            "name": "ScanFileTraCompletionMarker",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileTy": {
            "name": "ScanFileTy",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileTyp": {
            "name": "ScanFileTyp",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileType": {
            "name": "ScanFileType",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileTyperaCompletionMarker": {
            "name": "ScanFileTyperaCompletionMarker",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileTypraCompletionMarker": {
            "name": "ScanFileTypraCompletionMarker",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileTyraCompletionMarker": {
            "name": "ScanFileTyraCompletionMarker",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanResult": {
            "name": "ScanResult",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "project",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "Project",
                    "column_name": "uuid"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "scanned_at",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanResults": {
            "name": "ScanResults",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "project",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "Project",
                    "column_name": "uuid"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "scanned_at",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          }
        },
        "extra_types": {}
      },
      "from": "20250213_232206584_root_dir",
      "up": {
        "sqlite": "CREATE TABLE ScanFile (\nid INTEGER NOT NULL PRIMARY KEY,\nscan INTEGER NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\nfile_type TEXT NOT NULL,\nFOREIGN KEY (scan) REFERENCES ScanResult(id)\n);\nCREATE TABLE ScanFileT (\nid INTEGER NOT NULL PRIMARY KEY,\nscan INTEGER NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\nfile_type TEXT NOT NULL,\nFOREIGN KEY (scan) REFERENCES ScanResult(id)\n);\nCREATE TABLE ScanFileTraCompletionMarker (\nid INTEGER NOT NULL PRIMARY KEY,\nscan INTEGER NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\nfile_type TEXT NOT NULL,\nFOREIGN KEY (scan) REFERENCES ScanResult(id)\n);\nCREATE TABLE ScanFileTy (\nid INTEGER NOT NULL PRIMARY KEY,\nscan INTEGER NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\nfile_type TEXT NOT NULL,\nFOREIGN KEY (scan) REFERENCES ScanResult(id)\n);\nCREATE TABLE ScanFileTyp (\nid INTEGER NOT NULL PRIMARY KEY,\nscan INTEGER NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\nfile_type TEXT NOT NULL,\nFOREIGN KEY (scan) REFERENCES ScanResult(id)\n);\nCREATE TABLE ScanFileType (\nid INTEGER NOT NULL PRIMARY KEY,\nscan INTEGER NOT NULL,\nfile_type TEXT NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\nFOREIGN KEY (scan) REFERENCES ScanResult(id)\n);\nCREATE TABLE ScanFileTyperaCompletionMarker (\nid INTEGER NOT NULL PRIMARY KEY,\nscan INTEGER NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\nfile_type TEXT NOT NULL,\nFOREIGN KEY (scan) REFERENCES ScanResult(id)\n);\nCREATE TABLE ScanFileTypraCompletionMarker (\nid INTEGER NOT NULL PRIMARY KEY,\nscan INTEGER NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\nfile_type TEXT NOT NULL,\nFOREIGN KEY (scan) REFERENCES ScanResult(id)\n);\nCREATE TABLE ScanFileTyraCompletionMarker (\nid INTEGER NOT NULL PRIMARY KEY,\nscan INTEGER NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\nfile_type TEXT NOT NULL,\nFOREIGN KEY (scan) REFERENCES ScanResult(id)\n);\nCREATE TABLE ScanResult (\nid INTEGER NOT NULL PRIMARY KEY,\nproject TEXT NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\nscanned_at TEXT NOT NULL,\nFOREIGN KEY (project) REFERENCES Project(\"uuid\")\n);\nCREATE TABLE ScanResults (\nid INTEGER NOT NULL PRIMARY KEY,\nproject TEXT NOT NULL,\nloc INTEGER,\n\"files\" INTEGER,\nscanned_at TEXT NOT NULL,\nFOREIGN KEY (project) REFERENCES Project(\"uuid\")\n);\nALTER TABLE Project ADD COLUMN last_scan TEXT DEFAULT NULL;\nALTER TABLE Project ADD COLUMN scans INTEGER DEFAULT NULL;\nCREATE TABLE Project__butane_tmp (\n\"uuid\" TEXT NOT NULL UNIQUE,\n\"name\" TEXT NOT NULL UNIQUE,\nloc INTEGER,\n\"files\" INTEGER,\nroot_dir TEXT NOT NULL,\ncreated_at TEXT NOT NULL,\nupdated_at TEXT NOT NULL,\nlast_scan TEXT,\nscans INTEGER\n);\nINSERT INTO Project__butane_tmp SELECT \"uuid\", \"name\", loc, \"files\", root_dir, created_at, updated_at, last_scan, scans FROM Project;\nDROP TABLE Project;\nALTER TABLE Project__butane_tmp RENAME TO Project;\nCREATE TABLE Project__butane_tmp (\n\"uuid\" TEXT NOT NULL PRIMARY KEY UNIQUE,\n\"name\" TEXT NOT NULL UNIQUE,\nloc INTEGER,\n\"files\" INTEGER,\nroot_dir TEXT NOT NULL,\ncreated_at TEXT NOT NULL,\nupdated_at TEXT NOT NULL,\nlast_scan TEXT,\nscans INTEGER\n);\nINSERT INTO Project__butane_tmp SELECT \"uuid\", \"name\", loc, \"files\", root_dir, created_at, updated_at, last_scan, scans FROM Project;\nDROP TABLE Project;\nALTER TABLE Project__butane_tmp RENAME TO Project;\n"
      },
      "down": {
        "sqlite": "DROP TABLE ScanFile;\nDROP TABLE ScanFileT;\nDROP TABLE ScanFileTraCompletionMarker;\nDROP TABLE ScanFileTy;\nDROP TABLE ScanFileTyp;\nDROP TABLE ScanFileType;\nDROP TABLE ScanFileTyperaCompletionMarker;\nDROP TABLE ScanFileTypraCompletionMarker;\nDROP TABLE ScanFileTyraCompletionMarker;\nDROP TABLE ScanResult;\nDROP TABLE ScanResults;\nALTER TABLE Project ADD COLUMN id INTEGER NOT NULL PRIMARY KEY DEFAULT 0;\nCREATE TABLE Project__butane_tmp (\n\"uuid\" TEXT NOT NULL PRIMARY KEY UNIQUE,\n\"name\" TEXT NOT NULL UNIQUE,\nroot_dir TEXT NOT NULL,\nloc INTEGER,\n\"files\" INTEGER,\nscans INTEGER,\ncreated_at TEXT NOT NULL,\nupdated_at TEXT NOT NULL,\nid INTEGER NOT NULL PRIMARY KEY\n);\nINSERT INTO Project__butane_tmp SELECT \"uuid\", \"name\", root_dir, loc, \"files\", scans, created_at, updated_at, id FROM Project;\nDROP TABLE Project;\nALTER TABLE Project__butane_tmp RENAME TO Project;\nCREATE TABLE Project__butane_tmp (\n\"uuid\" TEXT NOT NULL PRIMARY KEY UNIQUE,\n\"name\" TEXT NOT NULL UNIQUE,\nroot_dir TEXT NOT NULL,\nloc INTEGER,\n\"files\" INTEGER,\ncreated_at TEXT NOT NULL,\nupdated_at TEXT NOT NULL,\nid INTEGER NOT NULL PRIMARY KEY\n);\nINSERT INTO Project__butane_tmp SELECT \"uuid\", \"name\", root_dir, loc, \"files\", created_at, updated_at, id FROM Project;\nDROP TABLE Project;\nALTER TABLE Project__butane_tmp RENAME TO Project;\nCREATE TABLE Project__butane_tmp (\n\"uuid\" TEXT NOT NULL UNIQUE,\n\"name\" TEXT NOT NULL UNIQUE,\nroot_dir TEXT NOT NULL,\nloc INTEGER,\n\"files\" INTEGER,\ncreated_at TEXT NOT NULL,\nupdated_at TEXT NOT NULL,\nid INTEGER NOT NULL PRIMARY KEY\n);\nINSERT INTO Project__butane_tmp SELECT \"uuid\", \"name\", root_dir, loc, \"files\", created_at, updated_at, id FROM Project;\nDROP TABLE Project;\nALTER TABLE Project__butane_tmp RENAME TO Project;\n"
      }
    },
    "20250216_124825489_add_scan_file_created_at": {
      "name": "20250216_124825489_add_scan_file_created_at",
      "db": {
        "tables": {
          "Project": {
            "name": "Project",
            "columns": [
              {
                "name": "uuid",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": false,
                "unique": true,
                "default": null
              },
              {
                "name": "name",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": true,
                "default": null
              },
              {
                "name": "root_dir",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "scans",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "last_scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "created_at",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "updated_at",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFile": {
            "name": "ScanFile",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileT": {
            "name": "ScanFileT",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileTraCompletionMarker": {
            "name": "ScanFileTraCompletionMarker",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileTy": {
            "name": "ScanFileTy",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileTyp": {
            "name": "ScanFileTyp",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileType": {
            "name": "ScanFileType",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "created_at",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileTyperaCompletionMarker": {
            "name": "ScanFileTyperaCompletionMarker",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileTypraCompletionMarker": {
            "name": "ScanFileTypraCompletionMarker",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanFileTyraCompletionMarker": {
            "name": "ScanFileTyraCompletionMarker",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "scan",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "ScanResult",
                    "column_name": "id"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "file_type",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanResult": {
            "name": "ScanResult",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "project",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "Project",
                    "column_name": "uuid"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "scanned_at",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "ScanResults": {
            "name": "ScanResults",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "project",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Text"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null,
                "reference": {
                  "Literal": {
                    "table_name": "Project",
                    "column_name": "uuid"
                  }
                }
              },
              {
                "name": "loc",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "files",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Int"
                  }
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "scanned_at",
                "sqltype": {
                  "KnownId": {
                    "Ty": "Timestamp"
                  }
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          }
        },
        "extra_types": {}
      },
      "from": "20250216_120509395_scan_results",
      "up": {
        "sqlite": "ALTER TABLE ScanFileType ADD COLUMN created_at TEXT NOT NULL DEFAULT '1970-01-01T00:00:00';\n"
      },
      "down": {
        "sqlite": "CREATE TABLE ScanFileType__butane_tmp (\nid INTEGER NOT NULL PRIMARY KEY,\nscan INTEGER NOT NULL,\nfile_type TEXT NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\nFOREIGN KEY (scan) REFERENCES ScanResult(id)\n);\nINSERT INTO ScanFileType__butane_tmp SELECT id, scan, file_type, loc, \"files\" FROM ScanFileType;\nDROP TABLE ScanFileType;\nALTER TABLE ScanFileType__butane_tmp RENAME TO ScanFileType;\n"
      }
    }
  },
  "current": {
    "name": "current",
    "db": {
      "tables": {},
      "extra_types": {}
    },
    "from": null,
    "up": {},
    "down": {}
  },
  "latest": "20250216_124825489_add_scan_file_created_at"
}"#;
    MemMigrations::from_json(json)
}
