//
// Reentry Library
//
// A library to support the creation of a text adventure game
// by Riskpeep
use serde::de::{self, Deserializer, Error, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::error;
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
    pub prospect: Option<usize>,
    pub details: String,
    pub contents: String,
    pub text_go: String,
    pub weight: isize,
    pub capacity: isize,
    pub health: isize,
}

const DEF_PROSPECT: &str = "";
const DEF_DETAILS: &str = "You see nothing special.";
const DEF_CONTENTS: &str = "You see";
const DEF_TEXT_GO: &str = "You can't get much closer than this.";
const DEF_WEIGHT: isize = 99;
const DEF_CAPACITY: isize = 0;
const DEF_HEALTH: isize = 0;

pub fn default_prospect() -> String {
    DEF_PROSPECT.into()
}

pub fn is_default_prospect(value: &str) -> bool {
    value == DEF_PROSPECT
}

pub fn default_details() -> String {
    DEF_DETAILS.into()
}

pub fn is_default_details(value: &str) -> bool {
    value == DEF_DETAILS
}

pub fn default_contents() -> String {
    DEF_CONTENTS.into()
}

pub fn is_default_contents(value: &str) -> bool {
    value == DEF_CONTENTS
}

pub fn default_text_go() -> String {
    DEF_TEXT_GO.into()
}

pub fn is_default_text_go(value: &str) -> bool {
    value == DEF_TEXT_GO
}

pub fn default_weight() -> isize {
    DEF_WEIGHT
}

pub fn is_default_weight(value: &isize) -> bool {
    *value == DEF_WEIGHT
}

pub fn default_capacity() -> isize {
    DEF_CAPACITY
}

pub fn is_default_capacity(value: &isize) -> bool {
    *value == DEF_CAPACITY
}

pub fn default_health() -> isize {
    DEF_HEALTH
}

pub fn is_default_health(value: &isize) -> bool {
    *value == DEF_HEALTH
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

const LOC_PLAYER: usize = 0;
// const LOC_BRIDGE: usize = 1;
// const LOC_GALLEY: usize = 2;
// const LOC_CRYOCHAMBER: usize = 3;
// const LOC_OUTSIDE: usize = 4;
// const LOC_GLOSSY_PHOTO: usize = 5;
// const LOC_TABLE: usize = 6;
// const LOC_CRYOSUIT: usize = 7;
// const LOC_WRINKLED_PHOTO: usize = 8;
// const LOC_COPILOT: usize = 9;
// const LOC_PEN: usize = 10;
// const AFT_TO_GALLEY: usize = 11;
// const FWD_TO_BRIDGE: usize = 12;
// const PORT_TO_CRYOCHAMBER: usize = 13;
// const STBD_TO_GALLEY: usize = 14;
// const WALL_BRIDGE: usize = 15;
// const WALL_GALLEY: usize = 16;
// const WALL_CRYOCHAMBER: usize = 17;

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Object>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SavedObject {
    pub labels: Vec<String>,
    pub description: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub location: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub destination: String,
    #[serde(
        default = "default_prospect",
        skip_serializing_if = "is_default_prospect"
    )]
    pub prospect: String,
    #[serde(
        default = "default_details",
        skip_serializing_if = "is_default_details"
    )]
    pub details: String,
    #[serde(
        default = "default_contents",
        skip_serializing_if = "is_default_contents"
    )]
    pub contents: String,
    #[serde(
        default = "default_text_go",
        skip_serializing_if = "is_default_text_go"
    )]
    pub text_go: String,
    #[serde(default = "default_weight", skip_serializing_if = "is_default_weight")]
    pub weight: isize,
    #[serde(
        default = "default_capacity",
        skip_serializing_if = "is_default_capacity"
    )]
    pub capacity: isize,
    #[serde(default = "default_health", skip_serializing_if = "is_default_health")]
    pub health: isize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SavedWorld {
    pub objects: Vec<SavedObject>,
}

#[derive(Debug)]
pub enum ParseError {
    UnknownName(String),
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnknownName(message) => write!(f, "{}", message),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        World { objects: vec![] }
    }

    pub fn read_from_file(game_file: &str) -> Result<World, std::io::Error> {
        let game_file_path = Path::new(game_file);
        let game_file_data_res = read_to_string(game_file_path);

        match game_file_data_res {
            Ok(game_file_data) => {
                /*// Create a new World struct
                let new_world = World::new();

                // Write (serialize) the struct to a string using Serde
                let serialized_ron = ron::to_string(&new_world).unwrap();

                // Write the serialized string to the console
                println!("serialized = {}", serialized_ron);

                Ok(new_world)
                */

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
                if self.is_holding(from_opt, Some(pos)) && object.prospect == to_opt {
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

        for (pos, object) in self.objects.iter().enumerate() {
            if self.is_holding(self.objects[LOC_PLAYER].location, Some(pos))
                && pos == LOC_PLAYER
                && object.health > 0
            {
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
                    output = output + &format!("{}:\n", self.objects[location].contents);
                }
                count += 1;
                output = output + &format!("{}\n", object.description);
            }
        }
        (output, count)
    }

    fn weight_of_contents(&self, container: usize) -> isize {
        let mut sum: isize = 0;
        for (pos, object) in self.objects.iter().enumerate() {
            if self.is_holding(Some(container), Some(pos)) {
                sum += object.weight;
            }
        }
        sum
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
                if self.objects[to_idx].health > 0 {
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
            (Some(obj_idx), Some(_), Some(to_idx))
                if self.objects[obj_idx].weight > self.objects[to_idx].capacity =>
            {
                "That is way too heavy.\n".to_string()
            }
            (Some(obj_idx), Some(_), Some(to_idx))
                if self.objects[obj_idx].weight + self.weight_of_contents(to_idx)
                    > self.objects[to_idx].capacity =>
            {
                "That would become to heavy.\n".to_string()
            }
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

                if obj_loc.is_some() && self.objects[obj_loc.unwrap()].health > 0 {
                    output_vis
                        + &format!(
                            "You should ask {} nicely.\n",
                            self.objects[obj_loc.unwrap()].labels[0]
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
            _ => {
                let (output_vis, obj_opt) = self.get_visible("what you want to look at", noun);
                let player_to_obj = self.get_distance(Some(LOC_PLAYER), obj_opt);

                match (player_to_obj, obj_opt) {
                    (Distance::HereContained, _) => {
                        output_vis + "Hard to see, you should try to get it first.\n"
                    }
                    (Distance::OverThere, _) => output_vis + "Too far away, move closer please.\n",
                    (Distance::NotHere, _) => {
                        output_vis + &format!("You don't see any {} here.\n", noun)
                    }
                    (Distance::UnknownObject, _) => output_vis,
                    (Distance::Location, Some(obj_idx)) => {
                        let (list_string, _) = self
                            .list_objects_at_location(self.objects[LOC_PLAYER].location.unwrap());
                        output_vis
                            + &format!("{}\n{}\n", self.objects[obj_idx].details, list_string)
                    }
                    (_, Some(obj_idx)) => {
                        let (list_string, _) =
                            self.list_objects_at_location(self.objects[obj_idx].location.unwrap());
                        output_vis
                            + &format!("{}\n{}\n", self.objects[obj_idx].details, list_string)
                    }
                    (_, None) => {
                        // Should never be here
                        output_vis + "How can you look at nothing?.\n"
                    }
                }
            }
        }
    }

    fn move_player(&mut self, obj_opt: Option<usize>) -> String {
        let go_string = format!("{}\n", self.objects[obj_opt.unwrap()].text_go);
        let obj_dst = obj_opt.and_then(|a| self.objects[a].destination);
        if obj_dst != None {
            self.objects[LOC_PLAYER].location = obj_dst;
            go_string + "\n" + &self.do_look("around")
        } else {
            go_string
        }
    }

    pub fn do_go(&mut self, noun: &str) -> String {
        let (output_vis, obj_opt) = self.get_visible("where you want to go", noun);

        match self.get_distance(Some(LOC_PLAYER), obj_opt) {
            Distance::OverThere => self.move_player(obj_opt),
            Distance::NotHere => {
                format!("You don't see any {} here.\n", noun)
            }
            Distance::UnknownObject => output_vis,
            _ => self.move_player(obj_opt),
        }
    }
}

impl Object {
    fn new(
        new_labels: Vec<String>,
        new_description: String,
        new_location: Option<usize>,
        new_destination: Option<usize>,
        new_prospect: Option<usize>,
        new_details: String,
        new_contents: String,
        new_text_go: String,
        new_weight: isize,
        new_capacity: isize,
        new_health: isize,
    ) -> Object {
        Object {
            labels: new_labels,
            description: new_description,
            location: new_location,
            destination: new_destination,
            prospect: new_prospect,
            details: new_details,
            contents: new_contents,
            text_go: new_text_go,
            weight: new_weight,
            capacity: new_capacity,
            health: new_health,
        }
    }
}

impl SavedWorld {
    fn new(new_objects: Vec<SavedObject>) -> SavedWorld {
        SavedWorld {
            objects: new_objects,
        }
    }
}

impl From<&World> for SavedWorld {
    fn from(value: &World) -> Self {
        let mut new_vec_of_objects: Vec<SavedObject> = Vec::new();

        for item in &value.objects {
            new_vec_of_objects.push(SavedObject {
                labels: item.labels.clone(),
                description: item.description.to_string(),
                location: match item.location {
                    Some(location) => value.objects[location].labels[0].to_string(),
                    None => "".to_string(),
                },
                destination: match item.destination {
                    Some(destination) => value.objects[destination].labels[0].to_string(),
                    None => "".to_string(),
                },
                prospect: match item.prospect {
                    Some(prospect) => value.objects[prospect].labels[0].to_string(),
                    None => "".to_string(),
                },
                details: item.details.to_string(),
                contents: item.contents.to_string(),
                text_go: item.text_go.to_string(),
                weight: item.weight,
                capacity: item.capacity,
                health: item.health,
            });
        }

        SavedWorld {
            objects: new_vec_of_objects,
        }
    }
}

impl TryInto<World> for SavedWorld {
    type Error = ParseError;

    fn try_into(self) -> Result<World, Self::Error> {
        let mut new_vec_of_objects: Vec<Object> = Vec::new();

        'items: for item in &self.objects {
            let mut new_object = Object::new(
                item.labels.clone(),
                item.description.to_string(),
                None,
                None,
                None,
                item.details.to_string(),
                item.contents.to_string(),
                item.text_go.to_string(),
                item.weight,
                item.capacity,
                item.health,
            );

            let mut found_location: bool = item.location.is_empty();
            let mut found_destination: bool = item.destination.is_empty();
            let mut found_prospect: bool = item.prospect.is_empty();

            for (pos, internal_item) in self.objects.iter().enumerate() {
                if item.location == internal_item.labels[0] {
                    new_object.location = Some(pos);
                    found_location = true;
                }
                if item.destination == internal_item.labels[0] {
                    new_object.destination = Some(pos);
                    found_destination = true;

                    // If no prospect is given then use the destination
                    if item.prospect.len() == 0 {
                        new_object.prospect = Some(pos);
                        found_prospect = true;
                    }
                }
                if item.prospect == internal_item.labels[0] {
                    new_object.prospect = Some(pos);
                    found_prospect = true;
                }
                if found_location && found_destination && found_prospect {
                    new_vec_of_objects.push(new_object);
                    continue 'items;
                }
            }

            if !found_location {
                return Err(ParseError::UnknownName(format!(
                    "Unknown location '{}'",
                    item.location
                )));
            }

            if !found_destination {
                return Err(ParseError::UnknownName(format!(
                    "Unknown destination '{}'",
                    item.destination
                )));
            }

            if !found_prospect {
                return Err(ParseError::UnknownName(format!(
                    "Unknown prospect '{}'",
                    item.prospect
                )));
            }

            new_vec_of_objects.push(new_object);
            return Err(ParseError::UnknownName("How are we here?".into()));
        }

        let result_world = World {
            objects: new_vec_of_objects,
        };

        Ok(result_world)
    }
}

impl Serialize for World {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serializeable_struct: SavedWorld = SavedWorld::from(self);

        // 1 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("World", 1)?;
        state.serialize_field("objects", &serializeable_struct.objects)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for World {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Objects,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`objects`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "objects" => Ok(Field::Objects),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct SavedWorldVisitor;

        impl<'de> Visitor<'de> for SavedWorldVisitor {
            type Value = SavedWorld;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct SavedWorld")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<SavedWorld, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let objects = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(SavedWorld::new(objects))
            }
            fn visit_map<V>(self, mut map: V) -> Result<SavedWorld, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut objects = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Objects => {
                            if objects.is_some() {
                                return Err(de::Error::duplicate_field("objects"));
                            }
                            objects = Some(map.next_value()?);
                        }
                    }
                }
                let objects = objects.ok_or_else(|| de::Error::missing_field("objects"))?;
                Ok(SavedWorld::new(objects))
            }
        }

        const FIELDS: &[&str] = &["objects"];
        let internal_extract = deserializer.deserialize_struct("World", FIELDS, SavedWorldVisitor);
        match internal_extract {
            Ok(extracted_val) => {
                let external_val = extracted_val.try_into();
                match external_val {
                    Ok(result_val) => Ok(result_val),
                    // From here: https://serde.rs/convert-error.html
                    // But that lacks context, this one is better:
                    // https://stackoverflow.com/questions/66230715/make-my-own-error-for-serde-json-deserialize
                    Err(_) => external_val.map_err(D::Error::custom),
                }
            }
            Err(err_val) => Err(err_val),
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
