use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Element {
    Neutral,
    Fire,
    Water,
    Earth,
    Air,
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Eqtype {
    OHand,
    THand,
    Armor,
    Helm,
    Accessory,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Equipment {
    pub name: String,
    pub desc: String,
    pub health: i32,
    pub mana: i32,
    pub stamina: i32,
    pub strength: i32,
    pub defense: i32,
    pub intellect: i32,
    pub resistance: i32,
    pub faith: i32,
    pub charisma: i32,
    pub dexterity: i32,
    pub speed: i32,
    pub skill: i32,
    pub luck: i32,
    pub gold: i32,
    pub element: Element,
    pub eqtype: Eqtype,
}

impl Equipment {
    pub fn new (sname: &str, sdesc: &str, shealth: i32, smana: i32, sstamina: i32, sstrength: i32, sdefense: i32, sintellect: i32, sresistance: i32, sfaith: i32, scharisma: i32, sdexterity: i32, sspeed: i32, sskill: i32,  sluck: i32, sgold: i32, selement: Element, stype: Eqtype) -> Equipment {
        Equipment {
            name: sname.to_string(),
            desc: sdesc.to_string(),
            health: shealth,
            mana: smana,
            stamina: sstamina,
            strength: sstrength,
            defense: sdefense,
            intellect: sintellect,
            resistance: sresistance,
            faith: sfaith,
            charisma: scharisma,
            dexterity: sdexterity,
            speed: sspeed,
            skill: sskill,
            luck: sluck,
            gold: sgold,
            element: selement,
            eqtype: stype,
        }
    }
    

    pub fn prnteq (&self) {
        
        let elmstring = match self.element {
            Element::Neutral => "Neutral",
            Element::Fire => "Fire",
            Element::Water => "Water",
            Element::Air => "Air",
            Element::Earth => "Earth",
            Element::Light => "Light",
            Element::Dark => "Dark",
        };

        let typestring = match self.eqtype {
            Eqtype::OHand => "One-Handed",
            Eqtype::THand => "Two-Handed",
            Eqtype::Armor => "Armor",
            Eqtype::Helm => "Helm",
            Eqtype::Accessory => "Accessory",
        };

        println!("\n[{} - {}]\nHP: {}   MP: {}\nSP: {}   TYP: {}\nSTR: {}   DEF: {}\nINT: {}   RES: {}\nFAI: {}   CHA: {}\nDEX: {}   SPD: {}\nSKL: {}   LCK: {}\nCST: {}   ELM: {}\n", 
        self.name, self.desc, 
        self.health, 
        self.mana,
        self.stamina,
        typestring,
        self.strength,
        self.defense,  
        self.intellect,  
        self.resistance,
        self.faith, 
        self.charisma,  
        self.dexterity, 
        self.speed,
        self.skill, 
        self.luck, 
        self.gold, elmstring);
    }
}

//Save equipment
pub fn save_eq (eq: &Vec <Equipment>) {
    let serialized_eq = serde_json::to_string(&eq).unwrap();
    
    let path = Path::new("out/equipment.json");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("\nCould not create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    
    file.write_all(serialized_eq.as_bytes()).unwrap(); 
}

//Load equipment
pub fn load_eq (eq: &mut Vec<Equipment>) -> Vec<Equipment> {
    let mut deserialized_eq = String::new();
    let path = Path::new("out/equipment.json");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("\nCould not open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("\nCould not read data for {}: {}", display, why.to_string()),
        Ok(_) => deserialized_eq = s, 
    };
    let load_eq = serde_json::from_str(&deserialized_eq).unwrap();
    return load_eq;
}

//Save currently equipped
pub fn save_ceq (eq: &Equipment, sub: &str) {
    let serialized_eq = serde_json::to_string(&eq).unwrap();
    let mut pathvar: String = "out/".to_owned();
    pathvar.push_str(sub);
    pathvar.push_str(".json");
    let path = Path::new(&pathvar);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("\nCould not create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    
    file.write_all(serialized_eq.as_bytes()).unwrap();
}

//Load currently equipped
pub fn load_ceq (eq: &mut Equipment, sub: &str) -> Equipment {
    let mut deserialized_eq = String::new();
    let mut pathvar: String = "out/".to_owned();
    pathvar.push_str(sub);
    pathvar.push_str(".json");
    let path = Path::new(&pathvar);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("\nCould not open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("\nCould not read data for {}: {}", display, why.to_string()),
        Ok(_) => deserialized_eq = s, 
    };
    let load_eq = serde_json::from_str(&deserialized_eq).unwrap();
    return load_eq;
}

//Empty equipment
pub fn empeq () -> Equipment {
    Equipment::new(
        "Empty",
        "Nothing equipped.",
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        Element::Neutral,
        Eqtype::OHand,
    )
}

//Equipment list

//Construct equipment
//
//(NAME, DESC, HP, MP, SP, STR, DEF, INT, RES, FAI, CHA, DEX, SPD, SKL, LCK, CST, ELM, TYP)

pub fn buckler () -> Equipment {
    let eq_buckler = Equipment::new(
        "Buckler",
        "A rudimentary shield with modest protection.",
        0,
        0,
        0,
        0,
        5,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        Element::Neutral,
        Eqtype::OHand,
    );
    eq_buckler
}

pub fn flametongue () -> Equipment {
    let eq_flametongue = Equipment::new(
        "Flametongue",
        "A blade pulsating with live flame.",
        0,
        0,
        0,
        5,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        Element::Fire,
        Eqtype::OHand,
    );
    eq_flametongue
}

pub fn zweihander () -> Equipment {
    let eq_zweihander = Equipment::new(
        "Zweihander",
        "A hefty two-handed blade with a big swing.",
        0,
        0,
        0,
        10,
        0,
        0,
        0,
        0,
        0,
        0,
        -2,
        0,
        0,
        0,
        Element::Neutral,
        Eqtype::THand,
    );
    eq_zweihander
}

pub fn chainmail () -> Equipment {
    let eq_chainmail = Equipment::new(
        "Chainmail",
        "A basic, decent, chainmail armor.",
        0,
        0,
        0,
        0,
        7,
        0,
        0,
        0,
        0,
        -1,
        0,
        0,
        0,
        0,
        Element::Neutral,
        Eqtype::Armor,
    );
    eq_chainmail
}

pub fn pot () -> Equipment {
    let eq_pot = Equipment::new(
        "Pot",
        "It still smells like beef stew.",
        0,
        0,
        0,
        0,
        2,
        0,
        0,
        0,
        1,
        -1,
        0,
        0,
        0,
        0,
        Element::Neutral,
        Eqtype::Helm,
    );
    eq_pot
}

pub fn irongauntlet () -> Equipment {
    let eq_irongauntlet = Equipment::new(
        "Iron Gauntlet",
        "A sturdy armlet with some wear and tear.",
        0,
        0,
        0,
        1,
        1,
        0,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
        0,
        Element::Neutral,
        Eqtype::Accessory,
    );
    eq_irongauntlet
}

pub fn blessedring () -> Equipment {
    let eq_blessedring = Equipment::new(
        "Blessed Ring",
        "A ring that radiates divine protection.",
        0,
        0,
        0,
        0,
        0,
        0,
        3,
        3,
        0,
        0,
        0,
        0,
        0,
        0,
        Element::Neutral,
        Eqtype::Accessory,
    );
    eq_blessedring
}