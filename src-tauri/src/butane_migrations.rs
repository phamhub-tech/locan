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
  "latest": "20250213_205624544_init"
}"#;
    MemMigrations::from_json(json)
}
