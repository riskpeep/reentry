//
// Reentry Library
//
// A library to support the creation of a text adventure game
// by Riskpeep
use std::fmt;
use std::io::{self, Write};

pub enum Command {
    Ask(String),
    Drop(String),
    Get(String),
    Give(String),
    Go(String),
    Inventory,
    Look(String),
    Quit,
    Unknown(String),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Ask(_) => write!(f, "ask"),
            Command::Drop(_) => write!(f, "drop"),
            Command::Get(_) => write!(f, "get"),
            Command::Give(_) => write!(f, "give"),
            Command::Go(_) => write!(f, "go"),
            Command::Inventory => write!(f, "inventory"),
            Command::Look(_) => write!(f, "look"),
            Command::Quit => write!(f, "quit"),
            Command::Unknown(_) => write!(f, "unknown"),
        }
    }
}

//#[derive(Debug)]
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

    pub fn get_possession(
        &mut self,
        from: Option<usize>,
        command: Command,
        noun: &String,
    ) -> (String, Option<usize>) {
        let object_idx = self.get_object_index(noun);
        let object_loc = object_idx.and_then(|a| self.objects[a].location);

        match (from, object_idx, object_loc) {
            (None, _, _) => (
                format!("I don't understand what you want to {}.\n", command),
                None,
            ),
            (Some(_), None, _) => (
                format!("I don't understand what you want to {}.\n", command),
                None,
            ),
            (Some(from_idx), Some(object_idx), _) if object_idx == from_idx => (
                format!(
                    "You should not be doing that to {}.\n",
                    self.objects[object_idx].name
                ),
                None,
            ),
            (Some(_), Some(object_idx), None) => (
                format!("You can't do that to {}.\n", self.objects[object_idx].name),
                None,
            ),
            (Some(from_idx), Some(object_idx), Some(object_loc_idx))
                if object_loc_idx != from_idx =>
            {
                if from_idx == LOC_PLAYER {
                    (
                        format!(
                            "You are not holding any {}.\n",
                            self.objects[object_idx].name
                        ),
                        None,
                    )
                } else {
                    (
                        format!(
                            "There appears to be no {} you can get from {}.\n",
                            noun, self.objects[from_idx].name
                        ),
                        None,
                    )
                }
            }
            _ => ("".to_string(), object_idx),
        }
    }

    pub fn actor_here(&self) -> Option<usize> {
        let mut actor_loc: Option<usize> = None;

        for (pos, object) in self.objects.iter().enumerate() {
            match (pos, object.location) {
                (_, obj_loc)
                    if (obj_loc == self.objects[LOC_PLAYER].location) && (pos == LOC_COPILOT) =>
                {
                    actor_loc = Some(pos);
                    break;
                }
                _ => continue,
            }
        }

        actor_loc
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

    pub fn describe_move(&self, obj_opt: Option<usize>, to: Option<usize>) -> String {
        let obj_loc = obj_opt.and_then(|a| self.objects[a].location);
        let player_loc = self.objects[LOC_PLAYER].location;

        match (obj_opt, obj_loc, to, player_loc) {
            (Some(obj_opt_idx), _, Some(to_idx), Some(player_loc_idx))
                if to_idx == player_loc_idx =>
            {
                format!("You drop {}.\n", self.objects[obj_opt_idx].name)
            }
            (Some(obj_opt_idx), _, Some(to_idx), _) if to_idx != LOC_PLAYER => {
                if to_idx == LOC_COPILOT {
                    format!(
                        "You give {} to {}.\n",
                        self.objects[obj_opt_idx].name, self.objects[to_idx].name
                    )
                } else {
                    format!(
                        "You put {} in {}.\n",
                        self.objects[obj_opt_idx].name, self.objects[to_idx].name
                    )
                }
            }
            (Some(obj_opt_idx), Some(obj_loc_idx), _, Some(player_loc_idx))
                if obj_loc_idx == player_loc_idx =>
            {
                format!("You pick up {}.\n", self.objects[obj_opt_idx].name)
            }
            (Some(obj_opt_idx), Some(obj_loc_idx), _, _) => format!(
                "You get {} from {}.\n",
                self.objects[obj_opt_idx].name, self.objects[obj_loc_idx].name
            ),
            // This arm should never get hit.
            (None, _, _, _) | (_, None, _, _) => format!("How can you drop nothing?.\n"),
        }
    }

    pub fn move_object(&mut self, obj_opt: Option<usize>, to: Option<usize>) -> String {
        let obj_loc = obj_opt.and_then(|a| self.objects[a].location);

        match (obj_opt, obj_loc, to) {
            (None, _, _) => format!(""),
            (Some(_), _, None) => format!("There is nobody to give that to.\n"),
            (Some(_), None, Some(_)) => format!("That is way too heavy.\n"),
            (Some(obj_idx), Some(_), Some(to_idx)) => {
                let output = self.describe_move(obj_opt, to);
                self.objects[obj_idx].location = Some(to_idx);
                output
            }
        }
    }

    pub fn update_state(&mut self, command: &Command) -> String {
        match command {
            Command::Ask(noun) => self.do_ask(noun),
            Command::Drop(noun) => self.do_drop(noun),
            Command::Get(noun) => self.do_get(noun),
            Command::Give(noun) => self.do_give(noun),
            Command::Go(noun) => self.do_go(noun),
            Command::Inventory => self.do_inventory(),
            Command::Look(noun) => self.do_look(noun),
            Command::Quit => format!("Quitting.\nThank you for playing!"),
            Command::Unknown(input_str) => format!("I don't know how to '{}'.", input_str),
        }
    }

    pub fn do_ask(&mut self, noun: &String) -> String {
        let actor_loc = self.actor_here();
        let (output, object_idx) =
            self.get_possession(actor_loc, Command::Ask("ask".to_string()), noun);

        output + self.move_object(object_idx, Some(LOC_PLAYER)).as_str()
    }

    pub fn do_drop(&mut self, noun: &String) -> String {
        let (output, object_idx) =
            self.get_possession(Some(LOC_PLAYER), Command::Drop("drop".to_string()), noun);
        let player_loc = self.objects[LOC_PLAYER].location;

        output + self.move_object(object_idx, player_loc).as_str()
    }

    pub fn do_get(&mut self, noun: &String) -> String {
        let (output_vis, obj_opt) = self.get_visible("where you want to go", noun);

        let obj_loc = obj_opt.and_then(|a| self.objects[a].location);

        match (obj_opt, obj_loc) {
            (None, _) => output_vis,
            (Some(object_idx), _) if object_idx == LOC_PLAYER => {
                output_vis + &format!("You should not be doing that to yourself.\n")
            }
            (Some(object_idx), Some(obj_loc)) if obj_loc == LOC_PLAYER => {
                output_vis
                    + &format!(
                        "You already have {}.\n",
                        self.objects[object_idx].description
                    )
            }
            (Some(_), Some(obj_loc)) if obj_loc == LOC_COPILOT => {
                output_vis + &format!("You should ask nicely.\n")
            }
            (obj_opt, _) => self.move_object(obj_opt, Some(LOC_PLAYER)),
        }
    }

    pub fn do_give(&mut self, noun: &String) -> String {
        let actor_loc = self.actor_here();
        let (output, object_idx) =
            self.get_possession(Some(LOC_PLAYER), Command::Give("give".to_string()), noun);

        output + self.move_object(object_idx, actor_loc).as_str()
    }

    pub fn do_inventory(&self) -> String {
        let (list_string, count) = self.list_objects_at_location(LOC_PLAYER);
        if count == 0 {
            format!("You are empty handed.\n")
        } else {
            list_string
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
        "ask" => Command::Ask(noun),
        "drop" => Command::Drop(noun),
        "get" => Command::Get(noun),
        "give" => Command::Give(noun),
        "go" => Command::Go(noun),
        "inventory" => Command::Inventory,
        "look" => Command::Look(noun),
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
