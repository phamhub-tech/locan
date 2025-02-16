//! Butane migrations embedded in Rust.

use butane::migrations::MemMigrations;

/// Load the butane migrations embedded in Rust.
pub fn get_migrations() -> Result<MemMigrations, butane::Error> {
    let json = r#"{
  "migrations": {
    "20250216_144541871_init": {
      "name": "20250216_144541871_init",
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
          }
        },
        "extra_types": {}
      },
      "from": null,
      "up": {
        "sqlite": "CREATE TABLE Project (\n\"uuid\" TEXT NOT NULL PRIMARY KEY UNIQUE,\n\"name\" TEXT NOT NULL UNIQUE,\nroot_dir TEXT NOT NULL,\nloc INTEGER,\n\"files\" INTEGER,\nscans INTEGER,\nlast_scan TEXT,\ncreated_at TEXT NOT NULL,\nupdated_at TEXT NOT NULL\n);\nCREATE TABLE ScanFileType (\nid INTEGER NOT NULL PRIMARY KEY,\nscan INTEGER NOT NULL,\nfile_type TEXT NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\ncreated_at TEXT NOT NULL,\nFOREIGN KEY (scan) REFERENCES ScanResult(id)\n);\nCREATE TABLE ScanResult (\nid INTEGER NOT NULL PRIMARY KEY,\nproject TEXT NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\nscanned_at TEXT NOT NULL,\nFOREIGN KEY (project) REFERENCES Project(\"uuid\")\n);\nCREATE TABLE IF NOT EXISTS butane_migrations (\n\"name\" TEXT NOT NULL PRIMARY KEY\n);\n"
      },
      "down": {
        "sqlite": "DROP TABLE Project;\nDROP TABLE ScanFileType;\nDROP TABLE ScanResult;\n"
      }
    },
    "20250216_170947199_add_file_type": {
      "name": "20250216_170947199_add_file_type",
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
                "name": "extension",
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
          }
        },
        "extra_types": {}
      },
      "from": "20250216_144541871_init",
      "up": {
        "sqlite": "ALTER TABLE ScanFileType ADD COLUMN \"extension\" TEXT NOT NULL DEFAULT '';\n"
      },
      "down": {
        "sqlite": "CREATE TABLE ScanFileType__butane_tmp (\nid INTEGER NOT NULL PRIMARY KEY,\nscan INTEGER NOT NULL,\nfile_type TEXT NOT NULL,\nloc INTEGER NOT NULL,\n\"files\" INTEGER NOT NULL,\ncreated_at TEXT NOT NULL,\nFOREIGN KEY (scan) REFERENCES ScanResult(id)\n);\nINSERT INTO ScanFileType__butane_tmp SELECT id, scan, file_type, loc, \"files\", created_at FROM ScanFileType;\nDROP TABLE ScanFileType;\nALTER TABLE ScanFileType__butane_tmp RENAME TO ScanFileType;\n"
      }
    },
    "20250216_171747873_rename_scan_file_type": {
      "name": "20250216_171747873_rename_scan_file_type",
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
                "name": "extension",
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
                "name": "extension",
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
          }
        },
        "extra_types": {}
      },
      "from": "20250216_170947199_add_file_type",
      "up": {
        "sqlite": "ALTER TABLE ScanFileType\nRENAME TO ScanFile;\n"
      },
      "down": {
        "sqlite": "ALTER TABLE ScanFile\nRENAME TO ScanFileType;\n"
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
  "latest": "20250216_171747873_rename_scan_file_type"
}"#;
    MemMigrations::from_json(json)
}
