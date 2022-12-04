//
// Reentry Library
//
// A library to support the creation of a text adventure game
// by Riskpeep
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::Path;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    pub labels: Vec<String>,
    pub description: String,
    pub location: Option<usize>,
    pub destination: Option<usize>,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum Distance {
    Me,
    Held,
    HeldContained,
    Location,
    Here,
    HereContained,
    OverThere,
    NotHere,
    UnknownObject,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum AmbiguousOption<T> {
    None,
    Some(T),
    Ambiguous,
}

const LOC_BRIDGE: usize = 0;
const LOC_GALLEY: usize = 1;
const LOC_CRYOCHAMBER: usize = 2;
const LOC_PLAYER: usize = 3;
// const LOC_GLOSSY_PHOTO: usize = 4;
// const LOC_CRYOSUIT: usize = 5;
// const LOC_WRINKLED_PHOTO: usize = 6;
const LOC_COPILOT: usize = 7;
// const LOC_PEN: usize = 8;
// const AFT_TO_GALLEY: usize = 9;
// const FWD_TO_BRIDGE: usize = 10;
// const PORT_TO_CRYOCHAMBER: usize = 11;
// const STBD_TO_GALLEY: usize = 12;
// const WALL_BRIDGE: usize = 13;
// const WALL_GALLEY: usize = 14;
// const WALL_CRYOCHAMBER: usize = 15;

#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub objects: Vec<Object>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        World {
            objects: vec![
                Object {
                    labels: vec!["Bridge".into()],
                    description: "the bridge".into(),
                    location: None,
                    destination: None,
                },
                Object {
                    labels: vec!["Galley".into()],
                    description: "the galley".into(),
                    location: None,
                    destination: None,
                },
                Object {
                    labels: vec!["Cryochamber".into()],
                    description: "the cryochamber".into(),
                    location: None,
                    destination: None,
                },
                Object {
                    labels: vec!["Yourself".into()],
                    description: "yourself".into(),
                    location: Some(LOC_BRIDGE),
                    destination: None,
                },
                Object {
                    labels: vec!["Glossy Photo".into(), "Photo".into()],
                    description: "a glossy photo of a family. They look familiar".into(),
                    location: Some(LOC_BRIDGE),
                    destination: None,
                },
                Object {
                    labels: vec!["Cryosuit".into()],
                    description: "a silver suit that will protect you in cryosleep".into(),
                    location: Some(LOC_CRYOCHAMBER),
                    destination: None,
                },
                Object {
                    labels: vec!["Wrinkled Photo".into(), "Photo".into()],
                    description: "a wrinkled photo of a woman. They woman is crying".into(),
                    location: Some(LOC_COPILOT),
                    destination: None,
                },
                Object {
                    labels: vec!["Copilot".into()],
                    description: "your copilot sleeping in his cryochamber".into(),
                    location: Some(LOC_CRYOCHAMBER),
                    destination: None,
                },
                Object {
                    labels: vec!["Pen".into()],
                    description: "a pen".into(),
                    location: Some(LOC_COPILOT),
                    destination: None,
                },
                Object {
                    labels: vec!["Aft".into()],
                    description: "a passage aft to the galley".into(),
                    location: Some(LOC_BRIDGE),
                    destination: Some(LOC_GALLEY),
                },
                Object {
                    labels: vec!["Forward".into()],
                    description: "a passage forward to the bridge".into(),
                    location: Some(LOC_GALLEY),
                    destination: Some(LOC_BRIDGE),
                },
                Object {
                    labels: vec!["Aft".into()],
                    description: "a passage aft to the cryochamber".into(),
                    location: Some(LOC_GALLEY),
                    destination: Some(LOC_CRYOCHAMBER),
                },
                Object {
                    labels: vec!["Forward".into()],
                    description: "a passage forward to the galley".into(),
                    location: Some(LOC_CRYOCHAMBER),
                    destination: Some(LOC_GALLEY),
                },
                Object {
                    labels: vec!["Forward".into(), "Port".into(), "Starboard".into()],
                    description: "a bulkhead covered in switchpanels and gauges".into(),
                    location: Some(LOC_BRIDGE),
                    destination: None,
                },
                Object {
                    labels: vec!["Port".into(), "Starboard".into()],
                    description: "a smooth bulkhead with an endless void on the other side".into(),
                    location: Some(LOC_GALLEY),
                    destination: None,
                },
                Object {
                    labels: vec!["Aft".into(), "Port".into(), "Starboard".into()],
                    description:
                        "cryochambers backed by a dense tangle of pipes, tubes, and conduits".into(),
                    location: Some(LOC_CRYOCHAMBER),
                    destination: None,
                },
            ],
        }
    }

    pub fn read_from_file(game_file: &str) -> Result<World, std::io::Error> {
        let game_file_path = Path::new(game_file);
        let game_file_data_res = read_to_string(game_file_path);

        match game_file_data_res {
            Ok(game_file_data) => {
                // Read (deserialize) a World struct from the game_file string
                let deserialized_ron_result: Result<World, ron::error::SpannedError> =
                    ron::from_str(&game_file_data);
                match deserialized_ron_result {
                    Ok(deserialized_ron) => Ok(deserialized_ron),
                    Err(de_err_str) => Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        de_err_str.to_string(),
                    )),
                }
            }
            Err(file_err) => Err(file_err),
        }
    }

    fn object_has_label(&self, object: &Object, noun: &str) -> bool {
        let mut result: bool = false;
        for (_, label) in object.labels.iter().enumerate() {
            if label.to_lowercase() == noun {
                result = true;
                break;
            }
        }
        result
    }

    fn get_object_index(
        &self,
        noun: &str,
        from: Option<usize>,
        max_distance: Distance,
    ) -> AmbiguousOption<usize> {
        let mut result: AmbiguousOption<usize> = AmbiguousOption::None;
        for (pos, object) in self.objects.iter().enumerate() {
            if self.object_has_label(object, noun)
                && self.get_distance(from, Some(pos)) <= max_distance
            {
                if result == AmbiguousOption::None {
                    result = AmbiguousOption::Some(pos);
                } else {
                    result = AmbiguousOption::Ambiguous;
                }
            }
        }
        result
    }

    pub fn is_holding(&self, container: Option<usize>, object: Option<usize>) -> bool {
        object.is_some() && (object.and_then(|a| self.objects[a].location) == container)
    }

    fn get_passage_index(&self, from_opt: Option<usize>, to_opt: Option<usize>) -> Option<usize> {
        let mut result: Option<usize> = None;

        if from_opt.is_some() && to_opt.is_some() {
            for (pos, object) in self.objects.iter().enumerate() {
                if self.is_holding(from_opt, Some(pos)) && object.destination == to_opt {
                    result = Some(pos);
                    break;
                }
            }
            result
        } else {
            result
        }
    }

    pub fn get_distance(&self, from: Option<usize>, to: Option<usize>) -> Distance {
        let from_loc = from.and_then(|a| self.objects[a].location);
        let to_loc = to.and_then(|a| self.objects[a].location);

        if to.is_none() {
            Distance::UnknownObject
        } else if to == from {
            Distance::Me
        } else if self.is_holding(from, to) {
            Distance::Held
        } else if self.is_holding(to, from) {
            Distance::Location
        } else if from_loc.is_some() && self.is_holding(from_loc, to) {
            Distance::Here
        } else if self.is_holding(from, to_loc) {
            Distance::HeldContained
        } else if self.is_holding(from_loc, to_loc) {
            Distance::HereContained
        } else if self.get_passage_index(from_loc, to).is_some() {
            Distance::OverThere
        } else {
            Distance::NotHere
        }
    }

    fn get_visible(&self, message: &str, noun: &str) -> (String, Option<usize>) {
        let obj_over_there = self.get_object_index(noun, Some(LOC_PLAYER), Distance::OverThere);
        let obj_not_here = self.get_object_index(noun, Some(LOC_PLAYER), Distance::NotHere);

        match (obj_over_there, obj_not_here) {
            (AmbiguousOption::None, AmbiguousOption::None) => {
                (format!("I don't understand {}.\n", message), None)
            }
            (AmbiguousOption::None, AmbiguousOption::Some(_)) => {
                (format!("You don't see any '{}' here.\n", noun), None)
            }
            (AmbiguousOption::Ambiguous, _)
            | (AmbiguousOption::None, AmbiguousOption::Ambiguous) => (
                format!("Please be more specific about which {} you mean.\n", noun),
                None,
            ),
            (AmbiguousOption::Some(index), _) => (String::new(), Some(index)),
        }
    }

    pub fn get_possession(
        &mut self,
        from: Option<usize>,
        command: Command,
        noun: &str,
    ) -> (String, Option<usize>) {
        let object_held = self.get_object_index(noun, from, Distance::HeldContained);
        let object_not_here = self.get_object_index(noun, from, Distance::NotHere);

        match (from, object_held, object_not_here) {
            (None, _, _) => (
                format!("I don't understand what you want to {}.\n", command),
                None,
            ),
            (Some(_), AmbiguousOption::None, AmbiguousOption::None) => (
                format!("I don't understand what you want to {}.\n", command),
                None,
            ),
            (Some(from_idx), AmbiguousOption::None, _) if from_idx == LOC_PLAYER => {
                (format!("You are not holding any {}.\n", noun), None)
            }
            (Some(from_idx), AmbiguousOption::None, _) => (
                format!(
                    "There appears to be no {} you can get from {}.\n",
                    noun, self.objects[from_idx].labels[0]
                ),
                None,
            ),
            (Some(from_idx), AmbiguousOption::Some(object_held_idx), _)
                if object_held_idx == from_idx =>
            {
                (
                    format!(
                        "You should not be doing that to {}.\n",
                        self.objects[object_held_idx].labels[0]
                    ),
                    None,
                )
            }
            (Some(_), AmbiguousOption::Ambiguous, _) => (
                format!(
                    "Please be more specific about which {} you want to {}.\n",
                    noun, command
                ),
                None,
            ),
            (Some(_), AmbiguousOption::Some(object_held_idx), _) => {
                ("".to_string(), Some(object_held_idx))
            }
        }
    }

    pub fn actor_here(&self) -> Option<usize> {
        let mut actor_loc: Option<usize> = None;

        for (pos, _) in self.objects.iter().enumerate() {
            if self.is_holding(self.objects[LOC_PLAYER].location, Some(pos)) && pos == LOC_COPILOT {
                actor_loc = Some(pos);
            }
        }
        actor_loc
    }

    pub fn list_objects_at_location(&self, location: usize) -> (String, i32) {
        let mut output = String::new();
        let mut count: i32 = 0;
        for (pos, object) in self.objects.iter().enumerate() {
            if pos != LOC_PLAYER && self.is_holding(Some(location), Some(pos)) {
                if count == 0 {
                    output += "You see:\n";
                }
                count += 1;
                output = output + &format!("{}\n", object.description);
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
                format!("You drop {}.\n", self.objects[obj_opt_idx].labels[0])
            }
            (Some(obj_opt_idx), _, Some(to_idx), _) if to_idx != LOC_PLAYER => {
                if to_idx == LOC_COPILOT {
                    format!(
                        "You give {} to {}.\n",
                        self.objects[obj_opt_idx].labels[0], self.objects[to_idx].labels[0]
                    )
                } else {
                    format!(
                        "You put {} in {}.\n",
                        self.objects[obj_opt_idx].labels[0], self.objects[to_idx].labels[0]
                    )
                }
            }
            (Some(obj_opt_idx), Some(obj_loc_idx), _, Some(player_loc_idx))
                if obj_loc_idx == player_loc_idx =>
            {
                format!("You pick up {}.\n", self.objects[obj_opt_idx].labels[0])
            }
            (Some(obj_opt_idx), Some(obj_loc_idx), _, _) => {
                format!(
                    "You get {} from {}.\n",
                    self.objects[obj_opt_idx].labels[0], self.objects[obj_loc_idx].labels[0]
                )
            }
            // This arm should never get hit.
            (None, _, _, _) | (_, None, _, _) => "How can you drop nothing?.\n".to_string(),
        }
    }

    pub fn move_object(&mut self, obj_opt: Option<usize>, to: Option<usize>) -> String {
        let obj_loc = obj_opt.and_then(|a| self.objects[a].location);

        match (obj_opt, obj_loc, to) {
            (None, _, _) => String::new(),
            (Some(_), _, None) => "There is nobody to give that to.\n".to_string(),
            (Some(_), None, Some(_)) => "That is way too heavy.\n".to_string(),
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
            Command::Quit => "Quitting.\nThank you for playing!".to_string(),
            Command::Unknown(input_str) => format!("I don't know how to '{}'.", input_str),
        }
    }

    pub fn do_ask(&mut self, noun: &str) -> String {
        let actor_loc = self.actor_here();
        let (output, object_idx) =
            self.get_possession(actor_loc, Command::Ask("ask".to_string()), noun);

        output + self.move_object(object_idx, Some(LOC_PLAYER)).as_str()
    }

    pub fn do_drop(&mut self, noun: &str) -> String {
        let (output, object_idx) =
            self.get_possession(Some(LOC_PLAYER), Command::Drop("drop".to_string()), noun);
        let player_loc = self.objects[LOC_PLAYER].location;

        output + self.move_object(object_idx, player_loc).as_str()
    }

    pub fn do_get(&mut self, noun: &str) -> String {
        let (output_vis, obj_opt) = self.get_visible("what you want to get", noun);

        let player_to_obj = self.get_distance(Some(LOC_PLAYER), obj_opt);

        match (player_to_obj, obj_opt) {
            (Distance::Me, _) => output_vis + "You should not be doing that to yourself.\n",
            (Distance::Held, Some(object_idx)) => {
                output_vis
                    + &format!(
                        "You already have {}.\n",
                        self.objects[object_idx].description
                    )
            }
            (Distance::OverThere, _) => output_vis + "Too far away, move closer please.\n",
            (Distance::UnknownObject, _) => output_vis,
            _ => {
                let obj_loc = obj_opt.and_then(|a| self.objects[a].location);

                if obj_loc == Some(LOC_COPILOT) {
                    output_vis
                        + &format!(
                            "You should ask {} nicely.\n",
                            self.objects[LOC_COPILOT].labels[0]
                        )
                } else {
                    self.move_object(obj_opt, Some(LOC_PLAYER))
                }
            }
        }
    }

    pub fn do_give(&mut self, noun: &str) -> String {
        let actor_loc = self.actor_here();
        let (output, object_idx) =
            self.get_possession(Some(LOC_PLAYER), Command::Give("give".to_string()), noun);

        output + self.move_object(object_idx, actor_loc).as_str()
    }

    pub fn do_inventory(&self) -> String {
        let (list_string, count) = self.list_objects_at_location(LOC_PLAYER);
        if count == 0 {
            "You are empty handed.\n".to_string()
        } else {
            list_string
        }
    }

    pub fn do_look(&self, noun: &str) -> String {
        match noun {
            "around" | "" => {
                let (list_string, _) =
                    self.list_objects_at_location(self.objects[LOC_PLAYER].location.unwrap());
                format!(
                    "{}\nYou are in {}.\n",
                    self.objects[self.objects[LOC_PLAYER].location.unwrap()].labels[0],
                    self.objects[self.objects[LOC_PLAYER].location.unwrap()].description
                ) + list_string.as_str()
            }
            _ => "I don't understand what you want to see.\n".to_string(),
        }
    }

    pub fn do_go(&mut self, noun: &str) -> String {
        let (output_vis, obj_opt) = self.get_visible("where you want to go", noun);

        match self.get_distance(Some(LOC_PLAYER), obj_opt) {
            Distance::OverThere => {
                self.objects[LOC_PLAYER].location = obj_opt;
                "OK.\n\n".to_string() + &self.do_look("around")
            }
            Distance::NotHere => {
                format!("You don't see any {} here.\n", noun)
            }
            Distance::UnknownObject => output_vis,
            _ => {
                let obj_dst = obj_opt.and_then(|a| self.objects[a].destination);
                if obj_dst.is_some() {
                    self.objects[LOC_PLAYER].location = obj_dst;
                    "OK.\n\n".to_string() + &self.do_look("around")
                } else {
                    "You can't get much closer than this.\n".to_string()
                }
            }
        }
    }
}

pub fn parse(input_str: String) -> Command {
    let lc_input_str = input_str.to_lowercase();
    let mut split_input_iter = lc_input_str.split_whitespace();

    let verb = split_input_iter.next().unwrap_or_default().to_string();
    let noun = split_input_iter.fold("".to_string(), |accum, item| {
        if accum.is_empty() {
            accum + item
        } else {
            accum + " " + item
        }
    });

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
    println!();

    // Parse & Return
    parse(input_str)
}

pub fn update_screen(output: String) {
    println!("{}", output);
}
