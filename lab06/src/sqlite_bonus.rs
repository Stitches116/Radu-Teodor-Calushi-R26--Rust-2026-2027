use anyhow::{Context, Result};
use rusqlite::{Connection, params};

struct Person {
    name: String,
    age: u8,
}

pub fn run() -> Result<()> {
    let conn = Connection::open("persons.db").context("Could not open persons database")?;

    // Create table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS persons (
            name TEXT PRIMARY KEY,
            age INTEGER NOT NULL
        )",
        [],
    )?;

    // Insert person
    let name = "sergiu";
    let age = 22;

    conn.execute(
        "INSERT OR REPLACE INTO persons (name, age)
         VALUES (?1, ?2)",
        params![name, age],
    )?;

    println!("Person saved");

    // Read persons
    let mut stmt = conn.prepare("SELECT name, age FROM persons")?;

    let persons = stmt.query_map([], |row| {
        Ok(Person {
            name: row.get("name")?,
            age: row.get("age")?,
        })
    })?;

    for person in persons {
        let person = person?;
        println!("name={}, age={}", person.name, person.age);
    }

    Ok(())
}
