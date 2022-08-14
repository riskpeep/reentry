//
// Reentry Library
//
// A library to support the creation of a text adventure game
// by Riskpeep
use std::io::{self, Write};

pub enum Command {
    Look(String),
    Go(String),
    Quit,
    Unknown(String),
}

pub fn parse(input_str: String) -> Command {
    let lc_input_str = input_str.to_lowercase();
    let mut split_input_iter = lc_input_str.trim().split_whitespace();

    let verb = split_input_iter.next().unwrap_or_default().to_string();
    let noun = split_input_iter.next().unwrap_or_default().to_string();

    match verb.as_str() {
        "look" => Command::Look(noun),
        "go" => Command::Go(noun),
        "quit" => Command::Quit,
        _ => Command::Unknown(input_str.trim().to_string()),
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

    // Parse & Return
    parse(input_str)
}

pub fn update_state(command: &Command) -> String {
    let output: String;

    match command {
        Command::Look(_) => {
            output = format!("It is very dark, you can see nothing but the flashing light.")
        }
        Command::Go(_) => output = format!("It is too dark to move."),
        Command::Quit => output = format!("Quitting.\nThank you for playing!"),
        Command::Unknown(input_str) => output = format!("I don't know how to '{}'.", input_str),
    }

    // Return
    output
}

pub fn update_screen(output: String) {
    println!("{}", output);
}
