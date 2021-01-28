use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Condition {
    Normal,
    Sleep,
    Poison,
    Paralyze,
    Regenerate,
}

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
pub struct Entity {
    pub name: String,
    pub desc: String,
    pub health: i32,
    pub healthmod: i32,
    pub curhealth: i32,
    pub mana: i32,
    pub manamod: i32,
    pub curmana: i32,
    pub stamina: i32,
    pub staminamod: i32,
    pub curstamina: i32,
    pub strength: i32,
    pub strengthmod: i32,
    pub defense: i32,
    pub defensemod: i32,
    pub intellect: i32,
    pub intellectmod: i32,
    pub resistance: i32,
    pub resistancemod: i32,
    pub faith: i32,
    pub faithmod: i32,
    pub charisma: i32,
    pub charismamod: i32,
    pub dexterity: i32,
    pub dexteritymod: i32,
    pub speed: i32,
    pub speedmod: i32,
    pub skill: i32,
    pub skillmod: i32,
    pub luck: i32,
    pub luckmod: i32,
    pub experience: i32,
    pub gold: i32,
    pub condition: Condition,
    pub tafa: String,
    pub tasu: String,
    pub element: Element,
}

impl Entity {
    pub fn new (sname: &str, sdesc: &str, shealth: i32, shealthmod: i32, scurhealth: i32, smana: i32, smanamod: i32, scurmana: i32, sstamina: i32, sstaminamod: i32, scurstamina: i32, sstrength: i32, sstrengthmod: i32, sdefense: i32, sdefensemod: i32, sintellect: i32, sintellectmod: i32, sresistance: i32, sresistancemod: i32, sfaith: i32, sfaithmod: i32, scharisma: i32, scharismamod: i32, sdexterity: i32, sdexteritymod: i32, sspeed: i32, sspeedmod: i32, sskill: i32, sskillmod: i32, sluck: i32, sluckmod: i32, sexperience: i32, sgold: i32, scondition: Condition, stafa: &str, stasu: &str, selement: Element) -> Entity {
        Entity {
            name: sname.to_string(),
            desc: sdesc.to_string(),
            health: shealth,
            healthmod: shealthmod,
            curhealth: scurhealth,
            mana: smana,
            manamod: smanamod,
            curmana: scurmana,
            stamina: sstamina,
            staminamod: sstaminamod,
            curstamina: scurstamina,
            strength: sstrength,
            strengthmod: sstrengthmod,
            defense: sdefense,
            defensemod: sdefensemod,
            intellect: sintellect,
            intellectmod: sintellectmod,
            resistance: sresistance,
            resistancemod: sresistancemod,
            faith: sfaith,
            faithmod: sfaithmod,
            charisma: scharisma,
            charismamod: scharismamod,
            dexterity: sdexterity,
            dexteritymod: sdexteritymod,
            speed: sspeed,
            speedmod: sspeedmod,
            skill: sskill,
            skillmod: sskillmod,
            luck: sluck,
            luckmod: sluckmod,
            experience: sexperience,
            gold: sgold,
            condition: scondition,
            tafa: stafa.to_string(),
            tasu: stasu.to_string(),
            element: selement,
        }
    }
    
    pub fn _health (&mut self, val: i32) {
        self.health += val;
    }
    
    pub fn _healthmod (&mut self, val: i32) {
        self.healthmod += val;
    }
    
    pub fn _curhealth (&mut self, val: i32) {
        self.curhealth += val;
    }

    pub fn _mana (&mut self, val: i32) {
        self.mana += val;
    }
    
    pub fn _manamod (&mut self, val: i32) {
        self.manamod += val;
    }
    
    pub fn _curmana (&mut self, val: i32) {
        self.curmana += val;
    }

    pub fn _stamina (&mut self, val: i32) {
        self.stamina += val;
    }
    
    pub fn _staminamod (&mut self, val: i32) {
        self.staminamod += val;
    }
    
    pub fn _curstamina (&mut self, val: i32) {
        self.curstamina += val;
    }
    
    pub fn _strength (&mut self, val: i32) {
        self.strength += val;
    }
    
    pub fn _strengthmod (&mut self, val: i32) {
        self.strengthmod += val;
    }
    
    pub fn _defense (&mut self, val: i32) {
        self.defense += val;
    }
    
    pub fn _defensemod (&mut self, val: i32) {
        self.defensemod += val;
    }
    
    pub fn _intellect (&mut self, val: i32) {
        self.intellect += val;
    }
    
    pub fn _intellectmod (&mut self, val: i32) {
        self.intellectmod += val;
    }
    
    pub fn _resistance (&mut self, val: i32) {
        self.resistance += val;
    }
    
    pub fn _resistancemod (&mut self, val: i32) {
        self.resistancemod += val;
    }
    
    pub fn _faith (&mut self, val: i32) {
        self.faith += val;
    }

    pub fn _faithmod (&mut self, val: i32) {
        self.faithmod += val;
    }
    
    pub fn _charisma (&mut self, val: i32) {
        self.charisma += val;
    }
    
    pub fn _charismamod (&mut self, val: i32) {
        self.charismamod += val;
    }

    pub fn _speed (&mut self, val: i32) {
        self.speed += val;
    }
    
    pub fn _speedmod (&mut self, val: i32) {
        self.speedmod += val;
    }
    
    pub fn _dexterity (&mut self, val: i32) {
        self.dexterity += val;
    }
    
    pub fn _dexteritymod (&mut self, val: i32) {
        self.dexteritymod += val;
    }
    
    pub fn _skill (&mut self, val: i32) {
        self.skill += val;
    }
    
    pub fn _skillmod (&mut self, val: i32) {
        self.skillmod += val;
    }
    
    pub fn _luck (&mut self, val: i32) {
        self.luck += val;
    }
    
    pub fn _luckmod (&mut self, val: i32) {
        self.luckmod += val;
    }
    
    pub fn _experience (&mut self, val: i32) {
        self.experience += val;
    }

    pub fn _gold (&mut self, val: i32) {
        self.gold += val;
    }
        
    pub fn _condition (&mut self, scondition: Condition) {
        self.condition = scondition;
    }

    pub fn prntent (&self) {
        let mut healthmodstring = "";
        let mut manamodstring = "";
        let mut staminamodstring = "";
        let mut strengthmodstring = "";
        let mut defensemodstring = "";
        let mut intellectmodstring = "";
        let mut resistancemodstring = "";
        let mut faithmodstring = "";
        let mut charismamodstring = "";
        let mut speedmodstring = "";
        let mut dexteritymodstring = "";
        let mut skillmodstring = "";
        let mut luckmodstring = "";
        let conditionstring =match self.condition {
            Condition::Normal => "",
            Condition::Sleep => "*Asleep*",
            Condition::Poison => "*Poisoned*",
            Condition::Paralyze => "*Paralyzed*",
            Condition::Regenerate => "*Regenerating*",
        };
        if self.healthmod > 0 {
            healthmodstring = "+";          
        } else if self.healthmod < 0 {
            healthmodstring = "-";
        }
        if self.manamod > 0 {
            manamodstring = "+";          
        } else if self.manamod < 0 {
            manamodstring = "-";
        }
        if self.staminamod > 0 {
            staminamodstring = "+";         
        } else if self.staminamod < 0 {
            staminamodstring = "-";
        }
        if self.strengthmod > 0 {
            strengthmodstring = "+";          
        } else if self.strengthmod < 0 {
            strengthmodstring = "-";
        }
        if self.defensemod > 0 {
            defensemodstring = "+";          
        } else if self.defensemod < 0 {
            defensemodstring = "-";
        }
        if self.intellectmod > 0 {
            intellectmodstring = "+";          
        } else if self.intellectmod < 0 {
            intellectmodstring = "-";
        }
        if self.resistancemod > 0 {
            resistancemodstring = "+";          
        } else if self.resistancemod < 0 {
            resistancemodstring = "-";
        }
        if self.faithmod > 0 {
            faithmodstring = "+";          
        } else if self.faithmod < 0 {
            faithmodstring = "-";
        }
        if self.charismamod > 0 {
            charismamodstring = "+";          
        } else if self.charismamod < 0 {
            charismamodstring = "-";
        }
        if self.speedmod > 0 {
            speedmodstring = "+";          
        } else if self.speedmod < 0 {
            speedmodstring = "-";
        }
        if self.dexteritymod > 0 {
            dexteritymodstring = "+";          
        } else if self.dexteritymod < 0 {
            dexteritymodstring = "-";
        }
        if self.skillmod > 0 {
            skillmodstring = "+";          
        } else if self.skillmod < 0 {
            skillmodstring = "-";
        }
        if self.luckmod > 0 {
            luckmodstring = "+";          
        } else if self.luckmod < 0 {
            luckmodstring = "-";
        }

        println!("\n[{} - {}]\n[{}/{}{} HP | {}/{}{} MP | {}/{}{} SP] {}\nSTR: {}{}   DEF: {}{}\nINT: {}{}   RES: {}{}\nFAI: {}{}   CHA: {}{}\nDEX: {}{}   SPD: {}{}\nSKL: {}{}   LCK: {}{}\nEXP: {}   GLD: {}\n", 
        self.name, self.desc, 
        self.curhealth, self.health+self.healthmod, healthmodstring, 
        self.curmana, self.mana+self.manamod, manamodstring, 
        self.curstamina, self.stamina+self.staminamod, staminamodstring,
        conditionstring, 
        self.strength+self.strengthmod, strengthmodstring, 
        self.defense+self.defensemod, defensemodstring, 
        self.intellect+self.intellectmod, intellectmodstring, 
        self.resistance+self.resistancemod, resistancemodstring, 
        self.faith+self.faithmod, faithmodstring, 
        self.charisma+self.charismamod, charismamodstring, 
        self.dexterity+self.dexteritymod, dexteritymodstring, 
        self.speed+self.speedmod, speedmodstring, 
        self.skill+self.skillmod, skillmodstring, 
        self.luck+self.luckmod, luckmodstring, 
        self.experience, self.gold);
    }

    //Display out-of-battle status
    pub fn mls (&mut self) {
        let playercondition = match self.condition {
                    Condition::Poison => "*Poisoned*",
                    Condition::Sleep => "*Asleep*",
                    Condition::Paralyze => "*Paralyzed*",
                    Condition::Regenerate => "*Regenerating*",
                    _ => "",
                };
                if self.curhealth < 0 {
                    self.curhealth = 0;
                }
        println!("\n[{} XP | {} GLD]", self.experience, self.gold);
        println!("[{}/{} HP | {}/{} MP | {}/{} SP] {}", self.curhealth, self.health+self.healthmod, self.curmana, self.mana+self.manamod, self.curstamina, self.stamina+self.staminamod, playercondition);
    }

    //Display battle status
    pub fn ls (&mut self, enemy: &mut Entity) {
        let playercondition = match self.condition {
                    Condition::Poison => "*Poisoned*",
                    Condition::Sleep => "*Asleep*",
                    Condition::Paralyze => "*Paralyzed*",
                    Condition::Regenerate => "*Regenerating*",
                    _ => "",
                };
                let enemycondition = match enemy.condition {
                    Condition::Poison => "*Poisoned*",
                    Condition::Sleep => "*Asleep*",
                    Condition::Paralyze => "*Paralyzed*",
                    Condition::Regenerate => "*Regenerating*",
                    _ => "",
        };
                if self.curhealth < 0 {
                    self.curhealth = 0;
                }
                if enemy.curhealth < 0 {
                    enemy.curhealth = 0;
                }
        println!("\n[{}: {}/{} HP | {}/{} MP | {}/{} SP] {}", self.name, self.curhealth, self.health+self.healthmod, self.curmana, self.mana+self.manamod, self.curstamina, self.stamina+self.staminamod, playercondition);
        println!("[{}: {}/{} HP | {}/{} MP | {}/{} SP] {}", enemy.name, enemy.curhealth, enemy.health+enemy.healthmod, enemy.curmana, enemy.mana+enemy.manamod, enemy.curstamina, enemy.stamina+enemy.staminamod, enemycondition);
    }

    //Reset self curval
    pub fn reset_cur (&mut self) {
        self.curhealth = self.health;
        self.curmana = self.mana;
        self.curstamina = self.stamina;
        self._condition(self::Condition::Normal);
    }

    //Reset self modval
    pub fn reset_mod (&mut self) {
        self.healthmod = 0;
        self.manamod = 0;
        self.staminamod = 0;
        self.strengthmod = 0;
        self.defensemod = 0;
        self.intellectmod = 0;
        self.resistancemod = 0;
        self.faithmod = 0;
        self.charismamod = 0;
        self.speedmod = 0;
        self.dexteritymod = 0;
        self.skillmod = 0;
        self.luckmod = 0;
    }
}

//Save entity
pub fn save_entity (entity: &Entity) {
    let serialized_entity = serde_json::to_string(&entity).unwrap();
    let mut pathvar: String = "out/".to_owned();
    let lowercaseentity = &entity.name.to_ascii_lowercase();
    let dataname: &str = &lowercaseentity;
    pathvar.push_str(dataname);
    pathvar.push_str(".json");
    let path = Path::new(&pathvar);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("\nCould not create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    
    file.write_all(serialized_entity.as_bytes()).unwrap();
}

//Load entity
pub fn load_entity (entity: &mut Entity) -> Entity {
    let mut deserialized_entity = String::new();
    let mut pathvar: String = "out/".to_owned();
    let lowercaseentity = &entity.name.to_ascii_lowercase();
    let dataname: &str = &lowercaseentity;
    pathvar.push_str(dataname);   
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
        Ok(_) => deserialized_entity = s, 
    };
    let load_entity = serde_json::from_str(&deserialized_entity).unwrap();
    return load_entity;
}

//Entity list

//Construct entities
//
//(Health, Healthmod, Curhealth, Mana, Manamod, Curmana, Stamina, Staminamod, Curstamina, 
//Strength, Strengthmod, Defense, Defensemod, Intellect, Intellectmod,
//Resistance, Resistancemod, Faith, Faithmod, Charisma, Charismamod, Speed, Speedmod, Dexterity, 
//Dexteritymod, Skill, Skillmod, Luck, Luckmod, Experience, Gold, Condition, Talk Failure
//Quote, Talk Success Quote)

pub fn player () -> Entity {
    let ent_player = Entity::new(
    "You", "The player.", 10, 0, 10, 10, 0, 10, 10, 0, 10, 
    10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 
    10, 10, 
    Condition::Normal, "", "", Element::Neutral);
    ent_player
}

pub fn goblin () -> Entity {
    let ent_goblin = Entity::new(
    "Goblin", "A hideous pest - with character.", 10, 0, 10, 10, 0, 10, 10, 0, 10, 
    10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 
    10, 10, 
    Condition::Normal, "You's ugly!", "*Shriek*", Element::Water);    
    ent_goblin
}

pub fn troll () -> Entity {
    let ent_troll = Entity::new(
    "Troll", "A colorful cave-dweller with a big stick.", 10, 0, 10, 10, 0, 10, 10, 0, 10, 
    10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 
    10, 10, 
    Condition::Normal, "I don't find your language appropriate.", "Let's put the weapons down, and be civilized.", Element::Water);    
    ent_troll
}

