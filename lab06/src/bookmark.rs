use crate::Command;
use anyhow::{Context, Result, bail};
use rusqlite::{Connection, params};

// Bookmark structure

struct Bookmark {
    name: String,
    url: String,
}

// Bookmark command

pub struct BookmarkCommand {}

impl Command for BookmarkCommand {
    fn get_name(&self) -> &str {
        "bk"
    }

    fn exec(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            bail!("Use: bk add <name> <url> or bk search <name>");
        }

        let conn = Connection::open("bookmarks.db").context("Could not open bookmarks database")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS bookmarks (
                name TEXT PRIMARY KEY,
                url TEXT NOT NULL
            )",
            [],
        )?;

        match args[0] {
            "add" => {
                if args.len() != 3 {
                    bail!("Use: bk add <name> <url>");
                }

                conn.execute(
                    "INSERT OR REPLACE INTO bookmarks (name, url)
                     VALUES (?1, ?2)",
                    params![args[1], args[2]],
                )?;

                println!("Saved bookmark: {}", args[1]);
            }

            "search" => {
                if args.len() != 2 {
                    bail!("Use: bk search <name>");
                }

                let mut stmt = conn.prepare(
                    "SELECT name, url FROM bookmarks
                     WHERE name LIKE ?1
                     ORDER BY name",
                )?;

                let search_text = format!("%{}%", args[1]);

                let bookmarks = stmt.query_map(params![search_text], |row| {
                    Ok(Bookmark {
                        name: row.get("name")?,
                        url: row.get("url")?,
                    })
                })?;

                let mut found = false;

                for bookmark in bookmarks {
                    let bookmark = bookmark?;

                    println!("{} => {}", bookmark.name, bookmark.url);
                    found = true;
                }

                if !found {
                    println!("No bookmarks found");
                }
            }

            _ => {
                bail!("Unknown bookmark action: {}", args[0]);
            }
        }

        Ok(())
    }
}
