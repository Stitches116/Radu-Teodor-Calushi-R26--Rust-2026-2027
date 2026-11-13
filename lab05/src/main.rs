mod p2;

use anyhow::{Context, Result, bail};
use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn print_oldest_and_youngest(students: &[Student]) -> Result<()> {
    if students.is_empty() {
        bail!("No students found");
    }

    let mut oldest = &students[0];
    let mut youngest = &students[0];

    for student in students {
        if student.age > oldest.age {
            oldest = student;
        }

        if student.age < youngest.age {
            youngest = student;
        }
    }

    println!(
        "Oldest student: {} - {} years, phone: {}",
        oldest.name, oldest.age, oldest.phone
    );

    println!(
        "Youngest student: {} - {} years, phone: {}",
        youngest.name, youngest.age, youngest.phone
    );

    Ok(())
}

// Problem 1

fn problem1() -> Result<()> {
    let text = fs::read_to_string("students.txt").context("Could not read students.txt")?;

    let mut students = Vec::new();

    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut parts = line.split(',');

        let name = match parts.next() {
            Some(value) => value.trim(),
            None => bail!("Missing name"),
        };

        let phone = match parts.next() {
            Some(value) => value.trim(),
            None => bail!("Missing phone"),
        };

        let age_text = match parts.next() {
            Some(value) => value.trim(),
            None => bail!("Missing age"),
        };

        if name.is_empty() || phone.is_empty() || age_text.is_empty() || parts.next().is_some() {
            bail!("Invalid student line: {line}");
        }

        let age = age_text.parse::<u32>().context("Invalid student age")?;

        let student = Student {
            name: name.to_string(),
            phone: phone.to_string(),
            age,
        };

        students.push(student);
    }

    print_oldest_and_youngest(&students)?;

    Ok(())
}

// Problem 3

fn problem3() -> Result<()> {
    let text = fs::read_to_string("students.jsonl").context("Could not read students.jsonl")?;

    let mut students = Vec::new();

    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let student: Student = serde_json::from_str(line).context("Invalid JSON student")?;
        students.push(student);
    }

    print_oldest_and_youngest(&students)?;

    Ok(())
}

fn main() -> Result<()> {
    println!(" Problem 1 ");
    problem1()?;

    println!("\n Problem 2 ");
    p2::run();

    println!("\n Problem 3 ");
    problem3()?;

    Ok(())
}
