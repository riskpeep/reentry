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

pub struct Object {
    pub name: String,
    pub description: String,
    pub location: Option<usize>,
}

const LOC_BRIDGE: usize = 0;
// const LOC_GALLEY: usize = 1;
const LOC_CRYOCHAMBER: usize = 2;
const LOC_PLAYER: usize = 3;
// const LOC_PHOTO: usize = 4;
// const LOC_CRYOSUIT: usize = 5;
const LOC_COPILOT: usize = 6;
// const LOC_PEN: usize = 7;

pub struct World {
    pub objects: Vec<Object>,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: vec![
                Object {
                    name: "Bridge".to_string(),
                    description: "the bridge".to_string(),
                    location: None,
                },
                Object {
                    name: "Galley".to_string(),
                    description: "the galley".to_string(),
                    location: None,
                },
                Object {
                    name: "Cryochamber".to_string(),
                    description: "the cryochamber".to_string(),
                    location: None,
                },
                Object {
                    name: "Yourself".to_string(),
                    description: "yourself".to_string(),
                    location: Some(LOC_BRIDGE),
                },
                Object {
                    name: "Photo".to_string(),
                    description: "a photo of a family. They look familiar".to_string(),
                    location: Some(LOC_BRIDGE),
                },
                Object {
                    name: "Cryosuit".to_string(),
                    description: "a silver suit that will protect you in cryosleep".to_string(),
                    location: Some(LOC_CRYOCHAMBER),
                },
                Object {
                    name: "Copilot".to_string(),
                    description: "your copilot sleeping in his cryochamber".to_string(),
                    location: Some(LOC_CRYOCHAMBER),
                },
                Object {
                    name: "Pen".to_string(),
                    description: "a pen".to_string(),
                    location: Some(LOC_COPILOT),
                },
            ],
        }
    }

    fn object_has_name(&self, object: &Object, noun: &String) -> bool {
        *noun == object.name.to_lowercase()
    }

    fn get_object_index(&self, noun: &String) -> Option<usize> {
        let mut result: Option<usize> = None;
        for (pos, object) in self.objects.iter().enumerate() {
            if self.object_has_name(&object, noun) {
                result = Some(pos);
                break;
            }
        }
        result
    }

    fn get_visible(&self, message: &str, noun: &String) -> (String, Option<usize>) {
        let mut output = String::new();

        let obj_index = self.get_object_index(noun);
        let obj_loc = obj_index.and_then(|a| self.objects[a].location);
        let obj_container_loc = obj_index
            .and_then(|a| self.objects[a].location)
            .and_then(|b| self.objects[b].location);
        let player_loc = self.objects[LOC_PLAYER].location;

        match (obj_index, obj_loc, obj_container_loc, player_loc) {
            // Is this even an object?  If not, print a message
            (None, _, _, _) => {
                output = format!("I don't understand {}.\n", message);
                (output, None)
            }
            //
            // For all the below cases, we've found an object, but should the player know that?
            //
            // Is this object the player?
            (Some(obj_index), _, _, _) if obj_index == LOC_PLAYER => (output, Some(obj_index)),
            //
            // Is this object in the same location as the player?
            (Some(obj_index), _, _, Some(player_loc)) if obj_index == player_loc => {
                (output, Some(obj_index))
            }
            //
            // Is this object being held by the player (i.e. 'in' the player)?
            (Some(obj_index), Some(obj_loc), _, _) if obj_loc == LOC_PLAYER => {
                (output, Some(obj_index))
            }
            //
            // Is this object at the same location as the player?
            (Some(obj_index), Some(obj_loc), _, Some(player_loc)) if obj_loc == player_loc => {
                (output, Some(obj_index))
            }
            //
            // Is this object any location?
            (Some(obj_index), obj_loc, _, _) if obj_loc == None => (output, Some(obj_index)),
            //
            // Is this object contained by any object held by the player
            (Some(obj_index), Some(_), Some(obj_container_loc), _)
                if obj_container_loc == LOC_PLAYER =>
            {
                (output, Some(obj_index))
            }
            //
            // Is this object contained by any object at the player's location?
            (Some(obj_index), Some(_), Some(obj_container_loc), Some(player_loc))
                if obj_container_loc == player_loc =>
            {
                (output, Some(obj_index))
            }
            //
            // If none of the above, then we don't know what the noun is.
            _ => {
                output = format!("You don't see any '{}' here.\n", noun);
                (output, None)
            }
        }
    }

    pub fn list_objects_at_location(&self, location: usize) -> (String, i32) {
        let mut output = String::new();
        let mut count: i32 = 0;
        for (pos, object) in self.objects.iter().enumerate() {
            match (pos, object.location) {
                (pos, _) if pos == LOC_PLAYER => continue,
                (_, Some(obj_location)) if obj_location == location => {
                    if count == 0 {
                        output = output + &format!("You see:\n");
                    }
                    count += 1;
                    output = output + &format!("{}\n", object.description);
                }
                _ => continue,
            }
        }
        (output, count)
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
            "around" | "" => {
                let (list_string, _) =
                    self.list_objects_at_location(self.objects[LOC_PLAYER].location.unwrap());
                format!(
                    "{}\nYou are in {}.\n",
                    self.objects[self.objects[LOC_PLAYER].location.unwrap()].name,
                    self.objects[self.objects[LOC_PLAYER].location.unwrap()].description
                ) + list_string.as_str()
            }
            _ => format!("I don't understand what you want to see.\n"),
        }
    }

    pub fn do_go(&mut self, noun: &String) -> String {
        let (output_vis, obj_opt) = self.get_visible("where you want to go", noun);

        let player_loc = self.objects[LOC_PLAYER].location;
        match (obj_opt, player_loc) {
            (None, _) => output_vis,
            (Some(obj_loc), Some(player_loc)) if obj_loc == player_loc => {
                format!("Wherever you go, there you are.\n")
            }
            (Some(obj_loc), _) => {
                self.objects[LOC_PLAYER].location = Some(obj_loc);
                format!("OK.\n\n") + &self.do_look(&"around".to_string())
            }
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
