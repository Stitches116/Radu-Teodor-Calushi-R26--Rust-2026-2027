mod bookmark;
mod sqlite_bonus;

use anyhow::{Context, Result, bail};
use std::fs;

// Command trait

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]) -> Result<()>;
}

// Ping command

struct PingCommand {}

impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }

    fn exec(&mut self, _args: &[&str]) -> Result<()> {
        println!("pong!");
        Ok(())
    }
}

// Count command

struct CountCommand {}

impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }

    fn exec(&mut self, args: &[&str]) -> Result<()> {
        println!("counted {} args", args.len());
        Ok(())
    }
}

// Times command

struct TimesCommand {
    count: u32,
}

impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }

    fn exec(&mut self, _args: &[&str]) -> Result<()> {
        self.count += 1;
        println!("times called {} times", self.count);
        Ok(())
    }
}

// Echo command

struct EchoCommand {}

impl Command for EchoCommand {
    fn get_name(&self) -> &str {
        "echo"
    }

    fn exec(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            bail!("Echo needs at least one argument");
        }

        println!("{}", args.join(" "));
        Ok(())
    }
}

// Check if a command name is similar to another

fn is_similar(typed: &str, correct: &str) -> bool {
    let typed = typed.to_ascii_lowercase();
    let correct = correct.to_ascii_lowercase();

    let a = typed.as_bytes();
    let b = correct.as_bytes();

    let mut i = 0;
    let mut j = 0;
    let mut mistakes = 0;

    while i < a.len() && j < b.len() {
        if a[i] == b[j] {
            i += 1;
            j += 1;
        } else {
            mistakes += 1;

            if mistakes > 1 {
                return false;
            }

            if a.len() > b.len() {
                i += 1;
            } else if b.len() > a.len() {
                j += 1;
            } else {
                i += 1;
                j += 1;
            }
        }
    }

    mistakes += a.len() - i + b.len() - j;
    mistakes <= 1
}

// Terminal

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Self {
        Terminal {
            commands: Vec::new(),
        }
    }

    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    fn run(&mut self) -> Result<()> {
        let text = fs::read_to_string("commands.txt").context("Could not read commands.txt")?;

        for (line_number, line) in text.lines().enumerate() {
            let mut parts = line.split_whitespace();

            let name = match parts.next() {
                Some(value) => value,
                None => {
                    println!("Error at line {}: empty line", line_number + 1);
                    continue;
                }
            };

            if name == "stop" {
                println!("Terminal stopped");
                break;
            }

            let args: Vec<&str> = parts.collect();
            let mut found = false;

            for command in &mut self.commands {
                if command.get_name() == name {
                    found = true;

                    if let Err(error) = command.exec(&args) {
                        println!("Error: {error}");
                    }

                    break;
                }
            }

            if !found {
                println!("Error: unknown command '{name}'");

                let mut suggested = false;

                for command in &self.commands {
                    if is_similar(name, command.get_name()) {
                        println!("Did you mean '{}'?", command.get_name());
                        suggested = true;
                        break;
                    }
                }

                if !suggested && is_similar(name, "stop") {
                    println!("Did you mean 'stop'?");
                }
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    // SQLite bonus
    if std::env::args().nth(1).as_deref() == Some("bonus") {
        return sqlite_bonus::run();
    }

    // Problem 1 and Problem 2
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(EchoCommand {}));
    terminal.register(Box::new(bookmark::BookmarkCommand {}));

    terminal.run()?;

    Ok(())
}
