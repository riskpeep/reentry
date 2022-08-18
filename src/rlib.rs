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

pub struct Location {
    pub name: String,
    pub description: String,
}

pub struct World {
    pub player_location: usize,
    pub locations: Vec<Location>,
}

impl World {
    pub fn new() -> Self {
        World {
            player_location: 0,
            locations: vec![
                Location {
                    name: "Bridge".to_string(),
                    description: "the bridge".to_string(),
                },
                Location {
                    name: "Galley".to_string(),
                    description: "the galley".to_string(),
                },
                Location {
                    name: "Cryochamber".to_string(),
                    description: "the cryochamber".to_string(),
                },
            ],
        }
    }

    pub fn update_state(&mut self, command: &Command) -> String {
        match command {
            Command::Look(noun) => self.do_look(noun),
            Command::Go(noun) => self.do_go(noun),
            Command::Quit => format!("Quitting.\nThank you for playing!"),
            Command::Unknown(input_str) => format!("I don't know how to '{}'.", input_str),
        }
    }

    pub fn do_look(&self, noun: &String) -> String {
        match noun.as_str() {
            "around" | "" => format!(
                "{}\nYou are in {}.\n",
                self.locations[self.player_location].name,
                self.locations[self.player_location].description
            ),
            _ => format!("I don't understand what you want to see.\n"),
        }
    }

    pub fn do_go(&mut self, noun: &String) -> String {
        let mut output = String::new();

        for (pos, location) in self.locations.iter().enumerate() {
            if *noun == location.name.to_lowercase() {
                if pos == self.player_location {
                    output = output + &format!("Wherever you go, there you are.\n");
                } else {
                    self.player_location = pos;
                    output = output + &format!("OK.\n\n") + &self.do_look(&"around".to_string());
                }
                break;
            }
        }

        if output.len() == 0 {
            format!("I don't understand where you want to go.")
        } else {
            output
        }
    }
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

pub fn update_screen(output: String) {
    println!("{}", output);
}
