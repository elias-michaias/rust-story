use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Status {
    Void,
    Poison,
    Paralyze,
    Sleep,
    Normal,
    Regenerate,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Statustarg {
    Void,
    Oneself,
    Enemy,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Playstateff {
    Void,
    Raise,
    Raisep,
    Lower,
    Lowerp,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Playstattarg {
    Void,
    Oneself,
    Enemy,
}   

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Element {
    Neutral,
    Fire,
    Water,
    Earth,
    Air,
    Light,
    Dark,
    Match,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Target {
    Oneself,
    Singen,
    Allen,
    Singal,
    Allal,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Costtype {
    Void,
    Health,
    Mana,
    Stamina,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Valuetarg {
    Void,
    Health,
    Mana,
    Stamina,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spell {
    pub status: Status,
    pub statustarg: Statustarg,
    pub playstat: Playstat,
    pub playstateff: Playstateff,
    pub playstattarg: Playstattarg,
    pub element: Element,
    pub target: Target,
    pub costtype: Costtype,
    pub cost: i32,
    pub value: i32,
    pub name: String,
    pub desc: String,
    pub valuetarg: Valuetarg,
    pub base: Base,
    pub invert: bool,
    pub acc: f64,
}

impl Spell {
    pub fn new (sstatus: Status, sstatustarg: Statustarg, splaystat: Playstat, splaystateff: Playstateff, splaystattarg: Playstattarg, selement: Element, starget: Target, scosttype: Costtype, scost: i32, svalue: i32, sname: &str, sdesc: &str, svaluetarg: Valuetarg, sbase: Base, sinvert: bool, sacc: f64) -> Spell {
        Spell {
            status: sstatus,
            statustarg: sstatustarg,
            playstat: splaystat,
            playstateff: splaystateff,
            playstattarg: splaystattarg,
            element: selement,
            target: starget,
            costtype: scosttype,
            cost: scost,
            value: svalue,
            name: sname.to_string(),
            desc: sdesc.to_string(),
            valuetarg: svaluetarg,
            base: sbase,
            invert: sinvert,
            acc: sacc,
        }
    }

    pub fn prntspl (&self) {
        
        let costsign: &str = match self.costtype {
            Costtype::Void => "",
            Costtype::Health => "HP", 
            Costtype::Mana => "MP",
            Costtype::Stamina => "SP",
        };

        let targetsign: &str = match self.target {
            Target::Oneself => "Self",
            Target::Singen => "Enemy",
            Target::Allen => "All Enemies",
            Target::Singal => "Single Ally",
            Target::Allal => "All Allies",
        };

        let statusname: &str = match self.status {
            Status::Void => "",
            Status::Poison => "Poisons",
            Status::Paralyze => "Paralyzes",
            Status::Sleep => "Tranquilizes",
            Status::Normal => "Restores",
            Status::Regenerate => "Bleses",
        };

        let statustargname: &str = match self.statustarg {
            Statustarg::Void => "N/A",
            Statustarg::Oneself => "self",
            Statustarg::Enemy => "enemy"
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
        
        let playstattargname: &str = match self.playstattarg {
            Playstattarg::Void => "",
            Playstattarg::Oneself => "own",
            Playstattarg::Enemy => "enemy",
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
            Element::Match => "Inherit L-Hand"
        };

        println!("\n[{} - {}]\nCost: {} {}\nPower: {}\nAccuracy: {}\nElement: {}\nBase: {}\nTarget: {}\nStatus: {} {}\nModifier: {} {} {}\n", 
                    self.name, 
                    self.desc, 
                    self.cost, costsign, 
                    self.value,
                    self.acc, 
                    elementname,
                    basename,
                    targetsign, 
                    statusname, statustargname,
                    playstateffname, playstattargname, playstatname); 
    }
}

//Save spells
pub fn save_spell (spell: &Vec <Spell>) {
    let serialized_spell = serde_json::to_string(&spell).unwrap();
    let path = Path::new("out/spells.json");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("\nCould not create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    
    file.write_all(serialized_spell.as_bytes()).unwrap(); 
}

//Load spells
pub fn load_spell (spell: &mut Vec <Spell>) -> Vec <Spell> {
    let mut deserialized_spell = String::new();
    let path = Path::new("out/spells.json");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("\nCould not open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("\nCould not read data for {}: {}", display, why.to_string()),
        Ok(_) => deserialized_spell = s, 
    };
    let load_spell = serde_json::from_str(&deserialized_spell).unwrap();
    return load_spell;
}

//Spell list

//Construct spells
//
//(Status, Statustarg, Playstat, Playstateff, Playstattarg, Element, Target, Costtype, Cost, Value, Name,
//Desc, Valuetarg, Base, Invert, Accuracy)

pub fn fireball () -> Spell { 
    let spl_fireball = Spell::new(
        Status::Void, 
        Statustarg::Void,
        Playstat::Void,
        Playstateff::Void,
        Playstattarg::Void,
        Element::Fire,
        Target::Singen,
        Costtype::Mana,
        2,
        10,
        "Fireball",
        "A basic fireball attack against a single target.",
        Valuetarg::Health,
        Base::Intellect,
        false,
        1.0);
    spl_fireball
}

pub fn diamonddust () -> Spell {
    let spl_diamonddust = Spell::new(
        Status::Sleep,
        Statustarg::Enemy,
        Playstat::Resistance,
        Playstateff::Lower,
        Playstattarg::Enemy,
        Element::Water,
        Target::Singen,
        Costtype::Mana,
        5,
        6,
        "Diamond Dust",
        "Advanced water spell to weaken a target and put it to sleep.",
        Valuetarg::Health,
        Base::Intellect,
        false,
        0.7);
    spl_diamonddust
}

pub fn soulcrush () -> Spell {
    let spl_soulcrush = Spell::new(
        Status::Void, 
        Statustarg::Void,
        Playstat::Void,
        Playstateff::Void,
        Playstattarg::Void,
        Element::Earth,
        Target::Singen,
        Costtype::Health,
        1,
        15,
        "Soul Crush",
        "Conjure astral rocks to crush opponent's stamina.",
        Valuetarg::Stamina,
        Base::Dexterity,
        false,
        0.9);
    spl_soulcrush
}

pub fn dropkick () -> Spell {
    let spl_dropkick = Spell::new(
        Status::Void, 
        Statustarg::Void,
        Playstat::Void,
        Playstateff::Void,
        Playstattarg::Void,
        Element::Neutral,
        Target::Singen,
        Costtype::Void,
        0,
        7,
        "Dropkick",
        "A kick that really puts your whole body into it.",
        Valuetarg::Health,
        Base::Strength,
        false,
        0.85);
        spl_dropkick
}

pub fn focusmind () -> Spell {
    let spl_focusmind = Spell::new(
        Status::Void,
        Statustarg::Void,
        Playstat::Intellect,
        Playstateff::Raise,
        Playstattarg::Oneself,
        Element::Neutral,
        Target::Oneself,
        Costtype::Stamina,
        2,
        6,
        "Focus Mind",
        "Focus the mind to raise intellect and heal self slightly.",
        Valuetarg::Health,
        Base::Intellect,
        true,
        1.0);
    spl_focusmind
}

pub fn focusbody () -> Spell {
    let spl_focusbody = Spell::new(
        Status::Void,
        Statustarg::Void,
        Playstat::Strength,
        Playstateff::Raise,
        Playstattarg::Oneself,
        Element::Neutral,
        Target::Oneself,
        Costtype::Stamina,
        2,
        6,
        "Focus Body",
        "Focus the body to raise strength and heal self slightly.",
        Valuetarg::Health,
        Base::Strength,
        true,
        1.0);
    spl_focusbody
}

pub fn zoom () -> Spell {
    let spl_zoom = Spell::new(
        Status::Void,
        Statustarg::Void,
        Playstat::Speed,
        Playstateff::Raisep,
        Playstattarg::Oneself,
        Element::Neutral,
        Target::Singen,
        Costtype::Stamina,
        2,
        6,
        "Zoom",
        "Charge past enemy to deal slight damage and boost speed.",
        Valuetarg::Health,
        Base::Strength,
        false,
        1.0);
    spl_zoom
}

pub fn cure () -> Spell {
    let spl_cure = Spell::new(
        Status::Normal,
        Statustarg::Oneself,
        Playstat::Void,
        Playstateff::Void,
        Playstattarg::Oneself,
        Element::Light,
        Target::Oneself,
        Costtype::Mana,
        3,
        10,
        "Cure",
        "Use holy power to lift status ailments and heal slightly.",
        Valuetarg::Health,
        Base::Faith,
        true,
        1.0);
    spl_cure
}

pub fn miasma () -> Spell {
    let spl_miasma = Spell::new(
        Status::Poison,
        Statustarg::Enemy,
        Playstat::Defense,
        Playstateff::Lower,
        Playstattarg::Enemy,
        Element::Dark,
        Target::Singen,
        Costtype::Mana,
        7,
        6,
        "Miasma",
        "Use arcane power to poison enemy and lower defense.",
        Valuetarg::Health,
        Base::Intellect,
        false,
        1.0);
    spl_miasma
}

pub fn bodyslam () -> Spell {
    let spl_bodyslam = Spell::new(
        Status::Void,
        Statustarg::Void,
        Playstat::Defense,
        Playstateff::Lowerp,
        Playstattarg::Oneself,
        Element::Neutral,
        Target::Singen,
        Costtype::Health,
        4,
        25,
        "Body Slam",
        "Sacrifice your defense for a devastating blow.",
        Valuetarg::Health,
        Base::Strength,
        false,
        0.8);
    spl_bodyslam
}

pub fn fierceslash () -> Spell {
    let spl_fierceslash = Spell::new(
        Status::Void,
        Statustarg::Void,
        Playstat::Void,
        Playstateff::Void,
        Playstattarg::Void,
        Element::Match,
        Target::Singen,
        Costtype::Mana,
        2,
        10,
        "Fierce Slash",
        "Cut enemy with overwhelming force.",
        Valuetarg::Health,
        Base::Strength,
        false,
        0.95);   
    spl_fierceslash
}

pub fn punch () -> Spell {
    let spl_en_punch = Spell::new(
        Status::Void, 
        Statustarg::Void,
        Playstat::Void,
        Playstateff::Void,
        Playstattarg::Void,
        Element::Neutral,
        Target::Singen,
        Costtype::Void,
        0,
        6,
        "Punch",
        "A meaty slug where it hurts.",
        Valuetarg::Health,
        Base::Strength,
        false,
        1.0);
    spl_en_punch
}

pub fn bash () -> Spell {
    let spl_en_bash = Spell::new(
        Status::Paralyze, 
        Statustarg::Enemy,
        Playstat::Void,
        Playstateff::Void,
        Playstattarg::Void,
        Element::Neutral,
        Target::Singen,
        Costtype::Mana,
        6,
        12,
        "Bash",
        "Paralyze an opponent with full force.",
        Valuetarg::Health,
        Base::Strength,
        false,
        0.8);
    spl_en_bash
}

