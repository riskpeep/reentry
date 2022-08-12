//
// Reentry Library
//
// A library to support the creation of a text adventure game
// by Riskpeep
use std::io::{self, Write};

pub struct Command {
    pub verb: String,
    pub noun: String,
}

impl Command {
    pub fn new() -> Command {
        Command {
            verb: String::new(),
            noun: String::new(),
        }
    }

    fn parse(&mut self, input_str: &str) {
        let mut split_input_iter = input_str.trim().split_whitespace();

        self.verb = split_input_iter.next().unwrap_or_default().to_string();
        self.noun = split_input_iter.next().unwrap_or_default().to_string();
    }
}

pub fn get_input() -> Command {
    // Prompt
    println!("");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input_str = String::new();

    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read move");
    println!("");

    // Parse
    let mut command = Command::new();
    command.parse(input_str.as_str());

    // Return
    command
}

pub fn update_state(command: &Command) -> String {
    let output: String;

    match command.verb.as_str() {
        "quit" => output = format!("Quitting.\nThank you for playing!"),
        "look" => output = format!("It is very dark, you can see nothing but the flashing light."),
        "go" => output = format!("It is too dark to move."),
        _ => output = format!("I don't know how to '{}'.", command.verb),
    }

    // Return
    output
}

pub fn update_screen(output: String) {
    println!("{}", output);
}
