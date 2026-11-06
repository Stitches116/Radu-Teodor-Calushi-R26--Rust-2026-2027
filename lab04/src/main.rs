use anyhow::{Context, Result, bail};
use std::fs;

// Problem 1

fn longest_lines() -> Result<()> {
    let text = fs::read_to_string("input1.txt").context("Could not read input1.txt")?;

    let mut longest_bytes = "";
    let mut longest_chars = "";

    for line in text.lines() {
        if line.len() > longest_bytes.len() {
            longest_bytes = line;
        }

        if line.chars().count() > longest_chars.chars().count() {
            longest_chars = line;
        }
    }

    println!("Longest line by bytes: {longest_bytes}");
    println!("Bytes: {}", longest_bytes.len());

    println!("Longest line by characters: {longest_chars}");
    println!("Characters: {}", longest_chars.chars().count());

    Ok(())
}

// Problem 2

fn rot13(text: &str) -> Result<String> {
    let mut result = String::new();

    for c in text.chars() {
        if !c.is_ascii() {
            bail!("Non-ASCII character found: {c}");
        }

        let new_char = if c.is_ascii_lowercase() {
            ((c as u8 - b'a' + 13) % 26 + b'a') as char
        } else if c.is_ascii_uppercase() {
            ((c as u8 - b'A' + 13) % 26 + b'A') as char
        } else {
            c
        };

        result.push(new_char);
    }

    Ok(result)
}

fn encrypt_file() -> Result<()> {
    let text = fs::read_to_string("input2.txt").context("Could not read input2.txt")?;

    let encrypted = rot13(&text)?;

    fs::write("rot13_output.txt", &encrypted).context("Could not write rot13_output.txt")?;

    println!("Original: {}", text.trim_end());
    println!("ROT13: {}", encrypted.trim_end());

    Ok(())
}

// Problem 3

fn replace_abbreviations(text: &str) -> String {
    let mut result = String::new();

    for (index, word) in text.split(' ').enumerate() {
        if index > 0 {
            result.push(' ');
        }

        let full_word = match word {
            "pt" | "ptr" => "pentru",
            "dl" => "domnul",
            "dna" => "doamna",
            _ => word,
        };

        result.push_str(full_word);
    }

    result
}

fn correct_text() -> Result<()> {
    let text = fs::read_to_string("input3.txt").context("Could not read input3.txt")?;

    let corrected = replace_abbreviations(text.trim_end());

    println!("Original: {}", text.trim_end());
    println!("Corrected: {corrected}");

    Ok(())
}

// Problem 4

fn show_hosts() -> Result<()> {
    let path = if cfg!(windows) {
        r"C:\Windows\System32\drivers\etc\hosts"
    } else {
        "/etc/hosts"
    };

    let text = fs::read_to_string(path).context("Could not read the hosts file")?;

    let mut found = false;

    for line in text.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut columns = line.split_whitespace();

        if let (Some(ip), Some(host)) = (columns.next(), columns.next()) {
            println!("{host} => {ip}");
            found = true;
        }
    }

    if !found {
        println!("No active entries found in the hosts file");
    }

    Ok(())
}

fn main() -> Result<()> {
    println!(" Problem 1 ");
    longest_lines()?;

    println!("\n Problem 2 ");
    encrypt_file()?;

    println!("\n Problem 3 ");
    correct_text()?;

    println!("\n Problem 4 ");
    show_hosts()?;

    Ok(())
}
