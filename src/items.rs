use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Void,
    Poison,
    Paralyze,
    Sleep,
    Normal,
    Regenerate
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Statustarg {
    Void,
    Oneself,
    Enemy,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Playstat {
    Void,
    Maxhealth,
    Maxmana,
    Maxstamina,
    Strength,
    Dexterity,
    Intellect,
    Faith,
    Charisma,
    Defense,
    Resistance,
    Speed,
    Skill,
    Luck,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Playstateff {
    Void,
    Raise,
    Raisep,
    Lower,
    Lowerp,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Playstattarg {
    Void,
    Oneself,
    Enemy,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Element {
    Neutral,
    Fire,
    Water,
    Earth,
    Air,
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Target {
    Oneself,
    Singen,
    Allen,
    Singal,
    Allal,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Valuetarg {
    Void,
    Health,
    Mana,
    Stamina,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Base {
    Void,
    Health,
    Mana,
    Stamina,
    Strength,
    Defense,
    Intellect,
    Resistance,
    Faith,
    Charisma,
    Speed,
    Dexterity,
    Skill,
    Luck,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub status: Status,
    pub statustarg: Statustarg,
    pub playstat: Playstat,
    pub playstateff: Playstateff,
    pub playstattarg: Playstattarg,
    pub element: Element,
    pub target: Target,
    pub amount: i32,
    pub value: i32,
    pub name: String,
    pub desc: String,
    pub valuetarg: Valuetarg,
    pub base: Base,
    pub invert: bool,
}

impl Item {
    pub fn new (sstatus: Status, sstatustarg: Statustarg, splaystat: Playstat, splaystateff: Playstateff, splaystattarg: Playstattarg, selement: Element, starget: Target, samount: i32, svalue: i32, sname: &str, sdesc: &str, svaluetarg: Valuetarg, sbase: Base, sinvert: bool) -> Item {
        Item {
            status: sstatus,
            statustarg: sstatustarg,
            playstat: splaystat,
            playstateff: splaystateff,
            playstattarg: splaystattarg,
            element: selement,
            target: starget,
            amount: samount,
            value: svalue,
            name: sname.to_string(),
            desc: sdesc.to_string(),
            valuetarg: svaluetarg,
            base: sbase,
            invert: sinvert,
        }
    }
    
    pub fn up_amount (&mut self, val: i32) {
        self.amount += val;
    }

    pub fn prntitem (&self) {

        let targetsign: &str = match self.target {
            Target::Oneself => "Self",
            Target::Singen => "Enemy",
            Target::Allen => "All Enemies",
            Target::Singal => "Single Ally",
            Target::Allal => "All Allies",
        };

        let statuseffname: &str = match self.status {
            Status::Void => "",
            Status::Poison => "Poisons",
            Status::Paralyze => "Paralyzes",
            Status::Sleep => "Tranquilizes",
            Status::Normal => "Restores",
            Status::Regenerate => "Blesses",
        };

        let playstatname: &str = match self.playstat {
            Playstat::Void => "",
            Playstat::Maxhealth => "maximum health",
            Playstat::Maxmana => "maximum mana",
            Playstat::Maxstamina => "maximum stamina",
            Playstat::Strength => "strength",
            Playstat::Intellect => "intellect",
            Playstat::Faith => "faith",
            Playstat::Charisma => "charisma",
            Playstat::Dexterity => "dexterity",
            Playstat::Defense => "defense",
            Playstat::Resistance => "resistance",
            Playstat::Speed => "speed",
            Playstat::Skill => "skill",
            Playstat::Luck => "luck",
        };

        let playstattargname: &str = match self.playstattarg {
            Playstattarg::Void => "",
            Playstattarg::Oneself => "own",
            Playstattarg::Enemy => "enemy",
        };

        let basename: &str = match self.base {
            Base::Void => "N/A",
            Base::Health => "Health",
            Base::Mana => "Mana",
            Base::Stamina => "Stamina",
            Base::Strength => "Strength",
            Base::Intellect => "Intellect",
            Base::Faith => "Faith",
            Base::Charisma => "Charisma",
            Base::Dexterity => "Dexterity",
            Base::Defense => "Defense",
            Base::Resistance => "Resistance",
            Base::Speed => "Speed",
            Base::Skill => "Skill",
            Base::Luck => "Luck",
        };


        let playstateffname: &str = match self.playstateff {
            Playstateff::Void => "N/A",
            Playstateff::Raise => "Raises",
            Playstateff::Raisep => "Greatly raises",
            Playstateff::Lower => "Lowers",
            Playstateff::Lowerp => "Greatly lowers",
        };

        let elementname: &str = match self.element {
            Element::Neutral => "Neutral",
            Element::Fire => "Fire",
            Element::Water => "Water",
            Element::Air => "Air",
            Element::Earth => "Earth",
            Element::Light => "Light",
            Element::Dark => "Dark",
        };

        let statustargname: &str = match self.statustarg {
            Statustarg::Void => "N/A",
            Statustarg::Oneself => "self",
            Statustarg::Enemy => "enemy"
        };

        println!("\n[{} - {}]\nAmount: {}\nPower: {}\nElement: {}\nBase: {}\nTarget: {}\nStatus: {} {}\nModifier: {} {} {}\n", 
                    self.name, 
                    self.desc, 
                    self.amount, 
                    self.value, 
                    elementname,
                    basename,
                    targetsign, 
                    statuseffname, statustargname, 
                    playstateffname, playstattargname, playstatname); 
    }
}

//Save items
pub fn save_item (item: &Vec <Item>) {
    let serialized_item = serde_json::to_string(&item).unwrap();
    let path = Path::new("out/items.json");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("\nCould not create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    
    file.write_all(serialized_item.as_bytes()).unwrap(); 
}

//Load items
pub fn load_item (item: &mut Vec <Item>) -> Vec <Item> {
    let mut deserialized_item = String::new();
    let path = Path::new("out/items.json");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("\nCould not open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("\nCould not read data for {}: {}", display, why.to_string()),
        Ok(_) => deserialized_item = s, 
    };
    let load_item = serde_json::from_str(&deserialized_item).unwrap();
    return load_item;
}

//Item list

//Construct items
//
//(Status, Statustarg, Playstat, Playstateff, Playstattarg, Element, Target, Amount, Value, Name,
//Desc, Valuetarg, Base, Invert)

pub fn smoothstone (Amount: i32) -> Item {    
    let item = Item::new(
        Status::Void,
        Statustarg::Void,
        Playstat::Void,
        Playstateff::Void,
        Playstattarg::Void,
        Element::Earth,
        Target::Singen,
        Amount,
        12,
        "Smooth Stone",
        "A small, smoothed rock. Great for throwing.",
        Valuetarg::Health,
        Base::Strength,
        false);
    item
}

pub fn medicinal_leaf (Amount: i32) -> Item {
    let item = Item::new(    
        Status::Void,
        Statustarg::Void,
        Playstat::Void,
        Playstateff::Void,
        Playstattarg::Void,
        Element::Neutral,
        Target::Oneself,
        Amount,
        12,
        "Medicinal Leaf",
        "A soothing leaf that can mend injury.",
        Valuetarg::Health,
        Base::Faith,
        true);
    item
}