mod spells;
mod entity;
mod items;
mod equip;
mod general;

use rand::prelude::*;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use json_pretty::PrettyFormatter;

extern crate json_pretty;

//Title Screen
fn title (players: &mut (entity::Entity, Vec<spells::Spell>, [equip::Equipment; 6]), inventory: &mut (Vec<items::Item>, Vec<equip::Equipment>)) {    
    println!("- Rust Story -\n1) New Game\n2) Load Game");
    loop {
        let dec = general::input();
        if dec == "1" {
            break;
        } else if dec == "2" {
            *players = load_player();
            *inventory = load_inv();
            break;
        }
    }
}

//Prompt save
fn save (player: &(entity::Entity, Vec<spells::Spell>, [equip::Equipment; 6]), inventory: &(Vec<items::Item>, Vec<equip::Equipment>)) {    
println!("\nWould you like to save? [Y/N]");
    loop {
        let dec = general::input();
        if dec == "y" || dec == "Y" || dec == "yes" || dec == "Yes" || dec == "YES" {
            println!("\nSaving...");
            general::pause();
            save_player(&player);
            save_inv(&inventory);
            println!("\nSuccessfully saved data.");
            general::pause();
            break;
        } else if dec == "n" || dec == "N" || dec == "no" || dec == "No" || dec == "NO" {
            println!("\nDid not save data.");
            general::pause();
            break;
        }
    }
}

//Save player
fn save_player (players: &(entity::Entity, Vec<spells::Spell>, [equip::Equipment; 6])) {
    let serialized_entity = serde_json::to_string(&players).unwrap();
    let formatter = PrettyFormatter::from_str(&serialized_entity);
    let result = formatter.pretty();

    let path = Path::new("out/you.json");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("\nCould not create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    
    file.write_all(result.as_bytes()).unwrap();
}

//Load player
fn load_player () -> (entity::Entity, Vec<spells::Spell>, [equip::Equipment; 6]) {
    let mut deserialized_entity = String::new();
    let path = Path::new("out/you.json");
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

//Save inventory
fn save_inv (inventory: &(Vec<items::Item>, Vec<equip::Equipment>)) {
    let serialized_entity = serde_json::to_string(&inventory).unwrap();
    let formatter = PrettyFormatter::from_str(&serialized_entity);
    let result = formatter.pretty();
    let path = Path::new("out/inventory.json");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("\nCould not create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    
    file.write_all(result.as_bytes()).unwrap();
}

//Load inventory
fn load_inv () -> (Vec<items::Item>, Vec<equip::Equipment>) {
    let mut deserialized_entity = String::new();
    let path = Path::new("out/inventory.json");
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

//Equip
fn equip (eqp: &mut equip::Equipment, playeq: &mut [equip::Equipment], left: bool, player: &mut entity::Entity) {
    
    match  eqp.eqtype {
        equip::Eqtype::OHand => {
            if left {
                unequip(&mut playeq[0], player);
                playeq[0] = eqp.clone();
            } else {
                unequip(&mut playeq[1], player);
                playeq[1] = eqp.clone();
            }
        },
        equip::Eqtype::THand => {
            unequip(&mut playeq[0], player);
            unequip(&mut playeq[1], player);
            playeq[0] = eqp.clone();
        },
        equip::Eqtype::Armor => {
            unequip(&mut playeq[2], player);
            playeq[2] = eqp.clone();
        },
        equip::Eqtype::Helm => {
            unequip(&mut playeq[3], player);
            playeq[3] = eqp.clone();
        },
        equip::Eqtype::Accessory => {
            if left {
                unequip(&mut playeq[4], player);
                playeq[4] = eqp.clone();
            } else {
                unequip(&mut playeq[5], player);
                playeq[5] = eqp.clone();
            }
        },
    } 
    
    player._health(eqp.health);
    player._mana(eqp.mana);
    player._stamina(eqp.stamina);
    player._strength(eqp.strength);
    player._defense(eqp.defense);
    player._intellect(eqp.intellect);
    player._resistance(eqp.resistance);
    player._faith(eqp.faith);
    player._charisma(eqp.charisma);
    player._speed(eqp.speed);
    player._dexterity(eqp.dexterity);
    player._skill(eqp.skill);
    player._luck(eqp.luck);

}

//Unequip 
fn unequip (eqp: &mut equip::Equipment, player: &mut entity::Entity) {

    player._health(-eqp.health);
    player._mana(-eqp.mana);
    player._stamina(-eqp.stamina);
    player._strength(-eqp.strength);
    player._defense(-eqp.defense);
    player._intellect(-eqp.intellect);
    player._resistance(-eqp.resistance);
    player._faith(-eqp.faith);
    player._charisma(-eqp.charisma);
    player._speed(-eqp.speed);
    player._dexterity(-eqp.dexterity);
    player._skill(-eqp.skill);
    player._luck(-eqp.luck);

    *eqp = equip::empeq();

}
    
//Menu
fn menu (playerstatus: &mut entity::Entity, spelllist: &Vec<spells::Spell>, eqlist: &mut [equip::Equipment], itemlist: &mut Vec<items::Item>, eqitems: &mut Vec<equip::Equipment>, enemy: &mut entity::Entity) {
        loop {
        playerstatus.mls();           
        print!("| [Menu]\n| 0) <- \n| 1) Action\n| 2) Item\n| 3) Inspect\n| 4) Equip\n");
        let s = general::input();

        //Action
        if s == "0" {
            break;
        }
        if s == "1" {
            loop {
                let mut i = 1;
                playerstatus.mls();
                println!("| | [Action]\n| | 0) <-");
                for spell in spelllist {
                    let costsign = match spell.costtype {
                        spells::Costtype::Void => "",
                        spells::Costtype::Health => "HP",
                        spells::Costtype::Mana => "MP",
                        spells::Costtype::Stamina => "SP",
                    };
                    if costsign != "" {
                        println!("| | {}) {} [{} {}]", i, spell.name, spell.cost, costsign);
                    } else {
                        println!("| | {}) {}", i, spell.name);
                    }
                    i+=1;
                }
                let ss = general::input();
                if ss == "0" {
                    break;
                }
                let ssint = match ss.parse::<usize> () {
                    Ok(ssint) => ssint,
                    Err(_) => {println!("\nNot a valid entry.\n"); break}
                };
                if ssint-1 >= spelllist.len() {
                    println!("\nNot a valid entry.\n");
                    break;
                }
                let costvalue = match spelllist[ssint-1].costtype {
                    spells::Costtype::Void => &0,
                    spells::Costtype::Health => &playerstatus.curhealth,
                    spells::Costtype::Mana => &playerstatus.curmana,
                    spells::Costtype::Stamina => &playerstatus.curstamina,
                };
                let selftarg = match spelllist[ssint-1].target {
                    spells::Target::Oneself => true,
                    _ => false,
                };
                if &spelllist[ssint-1].cost <= costvalue {
                    if selftarg && spelllist[ssint-1].invert {
                        action(playerstatus, &spelllist[ssint-1], enemy, &eqlist[0]);
                    } else {
                        println!("\nYou can not use this action outside of battle.");
                        general::pause();
                    }
                    break;
                } else {
                    println!("\nYou do not have sufficient resources to perform this action.");
                    general::pause();
                    break;
                }
            }

        //Item
        } else if s == "2" {
            loop {
                let mut i = 1;
                playerstatus.mls();
                println!("| | [Item]\n| | 0) <-");
                for item in &mut itemlist.iter () {
                    println!("| | {}) {} [x{}]", i, item.name, item.amount);
                    i+=1;
                }
                let ss = general::input();
                if ss == "0" {
                    break;
                }
                let ssint = match ss.parse::<usize> () {
                    Ok(ssint) => ssint,
                    Err(_) => {println!("\nNot a valid entry.\n"); break},
                };
                if ssint-1 >= itemlist.len() {
                    println!("\nNot a valid entry.\n");
                    break;
                }
                let selftarg = match &itemlist[ssint-1].target {
                    items::Target::Oneself => true,
                    _ => false,
                };
                if itemlist[ssint-1].amount >= 1 {
                    if selftarg && itemlist[ssint-1].invert {
                        item(playerstatus, &mut itemlist[ssint-1], enemy);
                    } else {
                        println!("\nYou can not use this item outside of battle.");
                        general::pause();
                    }
                    break;
                } else {
                    println!("\nYou do not have sufficient resources to perform this action.");
                    general::pause();
                    break;
                }
            }
        
        //Inspect
        } else if s == "3" {
        let mut skip = false;
            loop {
                if skip == true {
                    break;
                }
                playerstatus.mls();
                let mut ii = 1;
                println!("| | [Inspect]\n| | 0) <-\n| | 1) Self\n| | 2) Action\n| | 3) Item\n| | 4) Equipment"); 
                let ss = general::input();
                if ss == "1" {
                    playerstatus.prntent();
                    break;
                } else if ss == "2" {
                    loop {
                        playerstatus.mls();
                        println!("| | | [Inspect Action]\n| | | 0) <-");
                        for spell in spelllist {
                                let costsign = match spell.costtype {
                                    spells::Costtype::Void => "",
                                    spells::Costtype::Health => "HP",
                                    spells::Costtype::Mana => "MP",
                                    spells::Costtype::Stamina => "SP",
                                };
                            if costsign != "" {
                                println!("| | | {}) {} [{} {}]", ii, spell.name, spell.cost, costsign);
                            } else {
                                println!("| | | {}) {}", ii, spell.name);
                            }
                            ii+=1;
                        }
                        let sss = general::input();
                        if sss == "0" {
                            break;
                        }
                        let sssint = match sss.parse::<usize> () {
                            Ok(sssint) => sssint,
                            Err(_) => {println!("\nNot a valid entry.\n"); break},
                        };
                        if (sssint-1) >= spelllist.len() {
                            println!("\nNot a valid entry.\n");
                            skip=true;
                            break;
                        }
                        spelllist[sssint-1].prntspl();
                        skip=true;
                        break;
                    }
                } else if ss == "3" {
                    loop {
                        playerstatus.mls();
                        println!("| | | [Inspect Item]\n| | | 0) <-");
                        for item in &mut itemlist.iter () {
                            println!("| | | {}) {} [x{}]", ii, item.name, item.amount);
                            ii+=1;
                        }  
                        let sss = general::input();
                        if sss == "0" {
                            break;
                        }
                        let sssint = match sss.parse::<usize> () {
                            Ok(sssint) => sssint,
                            Err(_) => {println!("\nNot a valid entry.\n"); break},
                        };
                        if (sssint-1) >= itemlist.len() {
                            println!("\nNot a valid entry.\n");
                            skip=true;
                            break;
                        }
                        itemlist[sssint-1].prntitem();
                        skip=true;
                        break;
                    }
                } else if ss=="4" {
                    loop {
                        if skip {
                            break;
                        }
                        playerstatus.mls();
                        println!("| | | [Inspect Equipment]\n| | | 0) <-\n| | | 1) L-Hand: [{}]\n| | | 2) R-Hand: [{}]\n| | | 3) Armor: [{}]\n| | | 4) Helm: [{}]\n| | | 5) Acc-1: [{}]\n| | | 6) Acc-2: [{}]\n| | | 7) Inventory . . .",
                        eqlist[0].name, eqlist[1].name, eqlist[2].name, eqlist[3].name, eqlist[4].name, eqlist[5].name);
                        let sss = general::input();
                        if sss == "0" {
                            break;
                        } else if sss == "1" {
                            eqlist[0].prnteq();
                            skip=true;
                            break;
                        } else if sss == "2" {
                            eqlist[1].prnteq();
                            skip=true;
                            break;
                        } else if sss == "3" {
                            eqlist[2].prnteq();
                            skip=true;
                            break;
                        } else if sss == "4" {
                            eqlist[3].prnteq();
                            skip=true;
                            break;
                        } else if sss == "5" {
                            eqlist[4].prnteq();
                            skip=true;
                            break;
                        } else if sss == "6" {
                            eqlist[5].prnteq();
                            skip=true;
                            break;
                        } else if sss == "7" {
                            playerstatus.mls();
                            println!("| | | | [Inspect Inventory]\n| | | | 0) <-");
                            let mut i=1;
                             
                            for eqitem in &mut eqitems.iter() {
                              let typestring = match eqitem.eqtype {
                                equip::Eqtype::OHand => "One-Handed",
                                equip::Eqtype::THand => "Two-Handed",
                                equip::Eqtype::Accessory => "Accessory",
                                equip::Eqtype::Armor => "Armor",
                                equip::Eqtype::Helm => "Helm",
                              };
                              println!("| | | | {}) {} [{}]", i, eqitem.name, typestring);
                              i+=1;
                            }
                            loop {
                                let ssss = general::input();
                                if ssss == "0" {
                                    break;
                                }
                                let ssssint = match ssss.parse::<usize> () {
                                    Ok(ssssint) => ssssint,
                                    Err(_) => {println!("\nNot a valid entry.\n"); break},
                                };
                                if (ssssint-1) >= eqitems.len() {
                                    println!("\nNot a valid entry.\n");
                                    skip=true;
                                    break;
                                }
                                eqitems[ssssint-1].prnteq();
                                break;
                            }
                        }
                    }
                } else if ss=="0" {
                    break;
                } 
            }
        
        //Equip
        } else if s=="4" {
            let mut skip = false;
            loop {
                if skip {
                    break;
                }
                playerstatus.mls();
                println!("| | [Equipment]\n| | | 0) <-\n| | | 1) L-Hand: [{}]\n| | | 2) R-Hand: [{}]\n| | | 3) Armor: [{}]\n| | | 4) Helm: [{}]\n| | | 5) Acc-1: [{}]\n| | | 6) Acc-2: [{}]",
                eqlist[0].name, eqlist[1].name, eqlist[2].name, eqlist[3].name, eqlist[4].name, eqlist[5].name);
                let ss = general::input();
                if ss == "1" {
                    loop {
                        let mut ops: Vec<equip::Equipment> = vec![];
                        let mut i = 2;
                        println!("\n[Equip L-Hand]\n| | | 0) <-\n| | | 1) Unequip");
                        for eq in &mut eqitems.iter() {
                            match eq.eqtype {
                                equip::Eqtype::OHand => {ops.push(eq.clone()); println!("| | | {}) {}", i, eq.name); i+=1},
                                equip::Eqtype::THand => {ops.push(eq.clone()); println!("| | | {}) {} [Two-Handed]", i, eq.name); i+=1},
                                _ => (),
                            }
                        }
                        let ss = general::input();
                        if ss == "0" {
                            break;
                        } else if ss == "1" {
                            unequip(&mut eqlist[0], playerstatus);
                            break;
                        } else {
                            let ssint = match ss.parse::<usize> () {
                                Ok(ssint) => ssint,
                                Err(_) => {println!("\nNot a valid entry.\n"); break},
                            };
                            if (ssint-2) >= ops.len() {
                                println!("\nNot a valid entry.\n");
                                skip=true;
                                break;
                            }
                            let twohand = match eqlist[1].eqtype {
                                equip::Eqtype::OHand => false,
                                equip::Eqtype::THand => true,
                                _ => false,
                            };
                            if eqlist[1].name == ops[ssint-2].name {
                                
                                let mut ii = 0;
                                for eqa in &mut eqitems.iter() {
                                    if eqa.name == ops[ssint-2].name {
                                        ii+=1;
                                    }
                                }
                                if ii >= 2 {
                                    equip(&mut ops[ssint-2], eqlist, true, playerstatus);
                                    if twohand {
                                        unequip(&mut eqlist[1], playerstatus);
                                    }
                                    break;
                                } else {
                                    println!("\nYou do not have two of this equipment.");
                                    break;
                                }
                            } else {
                                equip(&mut ops[ssint-2], eqlist, true, playerstatus);
                                if twohand {
                                    unequip(&mut eqlist[1], playerstatus);
                                }
                                break;
                            }
                        }
                    }
                } else if ss == "2" {
                    loop {
                        let mut ops: Vec<equip::Equipment> = vec![];
                        let mut i = 2;
                        println!("\n[Equip R-Hand]\n| | | 0) <-\n| | | 1) Unequip");
                        for eq in &mut eqitems.iter() {
                            match eq.eqtype {
                                equip::Eqtype::OHand => {ops.push(eq.clone()); println!("| | | {}) {}", i, eq.name); i+=1},
                                equip::Eqtype::THand => {ops.push(eq.clone()); println!("| | | {}) {} (Two-Handed)", i, eq.name); i+=1},
                                _ => (),
                            }
                        }
                        let ss = general::input();
                        if ss == "0" {
                            break;
                        } else if ss == "1" {
                            unequip(&mut eqlist[1], playerstatus);
                            break;
                        } else {
                            let ssint = match ss.parse::<usize> () {
                                Ok(ssint) => ssint,
                                Err(_) => {println!("\nNot a valid entry.\n"); break},
                            };
                            if (ssint-2) >= ops.len() {
                                println!("\nNot a valid entry.\n");
                                skip=true;
                                break;
                            }
                            let twohand = match eqlist[0].eqtype {
                                equip::Eqtype::OHand => false,
                                equip::Eqtype::THand => true,
                                _ => false,
                            };
                            if eqlist[0].name == ops[ssint-2].name {
                                let mut ii = 0;
                                for eqa in &mut eqitems.iter() {
                                    if eqa.name == ops[ssint-2].name {
                                        ii+=1;
                                    }
                                }
                                if ii >= 2 {
                                    equip(&mut ops[ssint-2], eqlist, false, playerstatus);
                                    if twohand {
                                        unequip(&mut eqlist[0], playerstatus);
                                    }
                                    break;
                                } else {
                                    println!("\nYou do not have two of this equipment.");
                                    break;
                                }
                            } else {
                                equip(&mut ops[ssint-2], eqlist, false, playerstatus);
                                if twohand {
                                    unequip(&mut eqlist[0], playerstatus);
                                }
                                break;
                            }
                        }
                    }
                } else if ss == "3" {
                    loop {
                        let mut ops: Vec<equip::Equipment> = vec![];
                        let mut i = 2;
                        println!("\n[Equip Armor]\n| | | 0) <-\n| | | 1) Unequip");
                        for eq in &mut eqitems.iter() {
                            match eq.eqtype {
                                equip::Eqtype::Armor => {ops.push(eq.clone()); println!("| | | {}) {}", i, eq.name); i+=1},
                                _ => (),
                            }
                        }
                        let ss = general::input();
                        if ss == "0" {
                            break;
                        } else if ss == "1" {
                            unequip(&mut eqlist[2], playerstatus);
                            break;
                        } else {
                            let ssint = match ss.parse::<usize> () {
                                Ok(ssint) => ssint,
                                Err(_) => {println!("\nNot a valid entry.\n"); break},
                            };
                            if (ssint-2) >= ops.len() {
                                println!("\nNot a valid entry.\n");
                                skip=true;
                                break;
                            }
                            equip(&mut ops[ssint-2], eqlist, false, playerstatus);
                            break;
                        }
                    }
                } else if ss == "4" {
                    loop {
                        let mut ops: Vec<equip::Equipment> = vec![];
                        let mut i = 2;
                        println!("\n[Equip Helm]\n| | | 0) <-\n| | | 1) Unequip");
                        for eq in &mut eqitems.iter() {
                            match eq.eqtype {
                                equip::Eqtype::Helm => {ops.push(eq.clone()); println!("| | | {}) {}", i, eq.name); i+=1},
                                _ => (),
                            }
                        }
                        let ss = general::input();
                        if ss == "0" {
                            break;
                        } else if ss == "1" {
                            unequip(&mut eqlist[3], playerstatus);
                            break;
                        } else {
                            let ssint = match ss.parse::<usize> () {
                                Ok(ssint) => ssint,
                                Err(_) => {println!("\nNot a valid entry.\n"); break},
                            };
                            if (ssint-2) >= ops.len() {
                                println!("\nNot a valid entry.\n");
                                skip=true;
                                break;
                            }
                            equip(&mut ops[ssint-2], eqlist, false, playerstatus);
                            break;
                        }
                    }
                } else if ss == "5" {
                    loop {
                        let mut ops: Vec<equip::Equipment> = vec![];
                        let mut i = 2;
                        println!("\n[Equip Acc-1]\n| | | 0) <-\n| | | 1) Unequip");
                        for eq in &mut eqitems.iter() {
                            match eq.eqtype {
                                equip::Eqtype::Accessory => {ops.push(eq.clone()); println!("| | | {}) {}", i, eq.name); i+=1},
                                _ => (),
                            }
                        }
                        let ss = general::input();
                        if ss == "0" {
                            break;
                        } else if ss == "1" {
                            unequip(&mut eqlist[4], playerstatus);
                            break;
                        } else {
                            let ssint = match ss.parse::<usize> () {
                                Ok(ssint) => ssint,
                                Err(_) => {println!("\nNot a valid entry.\n"); break},
                            };
                            if (ssint-2) >= ops.len() {
                                println!("\nNot a valid entry.\n");
                                skip=true;
                                break;
                            }
                            if eqlist[5].name == ops[ssint-2].name {
                                
                                let mut ii = 0;
                                for eqa in &mut eqitems.iter() {
                                    if eqa.name == ops[ssint-2].name {
                                        ii+=1;
                                    }
                                }
                                if ii >= 2 {
                                    equip(&mut ops[ssint-2], eqlist, true, playerstatus);
                                    break;
                                } else {
                                    println!("\nYou do not have two of this equipment.");
                                    break;
                                }
                            } else {
                                equip(&mut ops[ssint-2], eqlist, true, playerstatus);
                                break;
                            }
                        }
                    }
                } else if ss == "6" {
                    loop {
                        let mut ops: Vec<equip::Equipment> = vec![];
                        let mut i = 2;
                        println!("\n[Equip Acc-2]\n| | | 0) <-\n| | | 1) Unequip");
                        for eq in &mut eqitems.iter() {
                            match eq.eqtype {
                                equip::Eqtype::Accessory => {ops.push(eq.clone()); println!("| | | {}) {}", i, eq.name); i+=1},
                                _ => (),
                            }
                        }
                        let ss = general::input();
                        if ss == "0" {
                            break;
                        } else if ss == "1" {
                            unequip(&mut eqlist[5], playerstatus);
                            break;
                        } else {
                            let ssint = match ss.parse::<usize> () {
                                Ok(ssint) => ssint,
                                Err(_) => {println!("\nNot a valid entry.\n"); break},
                            };
                            if (ssint-2) >= ops.len() {
                                println!("\nNot a valid entry.\n");
                                skip=true;
                                break;
                            }
                            if eqlist[4].name == ops[ssint-2].name {
                                
                                let mut ii = 0;
                                for eqa in &mut eqitems.iter() {
                                    if eqa.name == ops[ssint-2].name {
                                        ii+=1;
                                    }
                                }
                                if ii >= 2 {
                                    equip(&mut ops[ssint-2], eqlist, false, playerstatus);
                                    break;
                                } else {
                                    println!("\nYou do not have two of this equipment.");
                                    break;
                                }
                            } else {
                                equip(&mut ops[ssint-2], eqlist, false, playerstatus);
                                break;
                            }
                        }
                    }
                } else {
                    println!("\nNot a valid entry.\n");
                    break;
                }
            }
        } else {
            println!("\nNot a valid entry.\n"); 
            break;
        }
    }
}

fn encounter (playerstatus: &mut entity::Entity, spelllist: &Vec<spells::Spell>, eqlist: &[equip::Equipment], itemlist: &mut Vec<items::Item>, eqitems: &mut Vec<equip::Equipment>, 
    enemylist: &mut Vec<(entity::Entity, Vec<spells::Spell>, fn(&mut entity::Entity, &mut entity::Entity, bool, bool, bool, bool))>) {
    let mut rng = rand::thread_rng();
    enemylist.shuffle(&mut rng);
    let mut en_comp0 = enemylist[0].0.clone();
    let mut en_comp1 = enemylist[0].1.clone();
    let mut en_comp2 = enemylist[0].2.clone();
    engage(playerstatus, spelllist, eqlist, itemlist, eqitems, &mut en_comp0, &mut en_comp1, en_comp2, general::enemyappear());        
}


//Battle!
fn engage (playerstatus: &mut entity::Entity, spelllist: &Vec<spells::Spell>, eqlist: &[equip::Equipment], itemlist: &mut Vec<items::Item>, eqitems: &mut Vec<equip::Equipment>, enemy: &mut entity::Entity, enemyspelllist: &mut Vec<spells::Spell>, enemyfunc: fn(&mut entity::Entity, &mut entity::Entity, bool, bool, bool, bool), arrival: String) {
    let mut flee = false;
    let mut enemydefeat = false;
    let mut enemypersuade = false;
    let mut playerdead = false;
    let mut turnend = false;
    let mut playbroke = false;
    let mut enbroke = false;
    let mut paralyzecount = 0;
    let mut sleepcount = 0;
    let mut poisoncount = 0;
    let mut regencount = 0;
    let mut paralyzegone = 0;
    let mut sleepgone = 0;
    let mut poisongone = 0;
    let mut regengone = 0;
    let mut enparalyzecount = 0;
    let mut ensleepcount = 0;
    let mut enpoisoncount = 0;
    let mut enregencount = 0;
    let mut enparalyzegone = 0;
    let mut ensleepgone = 0;
    let mut enpoisongone = 0;
    let mut enregengone = 0;
    
    println!("\nUh, oh!");
    general::pause();
    println!("\n{} {}", enemy.name, arrival);
    general::pause();
    println!("\n- Get ready to battle! - ");

    while !playerdead && !enemydefeat && !flee && !enemypersuade {
        while !playerdead && !enemydefeat && !flee && !enemypersuade && !turnend {
           
            //End loop
            if playerstatus.curhealth <= 0 {
                playerdead = true;
            }
            if playerdead || enemydefeat || enemypersuade || flee {
                break;
            }

            //Break
            if playbroke {
                playbroke = false;
                println!("\nYou recovered some SP.");
                general::pause();
                playerstatus._curstamina(playerstatus.stamina/3);
            }
            if playerstatus.curstamina <= 0 {
                playerstatus.curstamina = 0;
                playbroke = true;
                println!("\nYour SP was depleted!");
                general::pause();
                println!("\n*Stance Break* [Max HP -{}][Max MP -{}][Max SP -{}]", playerstatus.health/3, playerstatus.mana/3, playerstatus.stamina/3);
                playerstatus._healthmod(-playerstatus.health/3);
                playerstatus._manamod(-playerstatus.mana/3);
                playerstatus._staminamod(-playerstatus.stamina/3);
                general::pause();
                println!("You can't act after your stance was broken...");
                general::pause();
                turnend = true;
            }

            //Condition
            let playercondition = match playerstatus.condition {
                entity::Condition::Poison => "*Poisoned*",
                entity::Condition::Sleep => "*Asleep*",
                entity::Condition::Paralyze => "*Paralyzed*",
                entity::Condition::Regenerate => "*Regenerating*",
                _ => "",
            };
            
            if playercondition == "*Poisoned*" {
                playerstatus._curhealth(-(playerstatus.health/10) as i32);
                println!("\nYou're poisoned! You lost {} HP.", playerstatus.health/10);
                general::pause();
                poisoncount += 1;
                if poisoncount ==  6 {
                    poisoncount = 0;
                    poisongone = 0;
                }
            } else if playercondition == "*Asleep*" {
                println!("\nYou're asleep! You can't get up...");
                playerstatus.ls(enemy);
                general::pause();
                sleepcount += 1;
                if sleepcount == 2 {
                    sleepcount = 0;
                    sleepgone = 1;
                }
                turnend = true;
            } else if playercondition == "*Paralyzed*" {
                println!("\nYou're paralyzed!");
                general::pause();
                if (playerstatus.speed as f64 + playerstatus.speedmod as f64)as f64/((enemy.speed as f64 + enemy.speedmod as f64)*2.0) >= general::rand_f() {
                    println!("\nYou managed to act anyway.");
                    paralyzecount += 1;
                    general::pause();
                } else {
                    println!("\nYou couldn't move due to paralysis...");                    
                    paralyzecount += 1;
                    general::pause();
                    turnend = true;
                }
            } else if playercondition == "*Regenerating*" {
                playerstatus._curhealth(playerstatus.health/10);
                if playerstatus.curhealth > playerstatus.health {
                    playerstatus._curhealth(-(playerstatus.curhealth - playerstatus.health));
                }
                println!("\nYou regenerated {} HP.", playerstatus.health/10);
                general::pause(); 
                regencount += 1;
                if regencount == 5 {
                    regencount = 0;
                    regengone = 1;
                }
            }
            if paralyzecount == 4 {
                paralyzecount = 0;
                paralyzegone = 1;
            }
            if poisongone == 1 {
                println!("\nYou got over the poison!");
                general::pause();
                poisongone=0;
                poisoncount=0;
                playerstatus._condition(entity::Condition::Normal);
            }
            if paralyzegone == 1 {
                println!("\nYou got over the paralysis!");
                general::pause();
                paralyzegone=0;
                paralyzecount=0;
                playerstatus._condition(entity::Condition::Normal);
            }
            if sleepgone == 1 {
                println!("\nYou woke up!");
                general::pause();
                sleepgone=0;
                sleepcount=0;
                playerstatus._condition(entity::Condition::Normal);
            }
            if regengone == 1 {
                println!("\nThe regneration wore off...");
                general::pause();
                regengone=0;
                regencount=0;
                playerstatus._condition(entity::Condition::Normal);
            }
            if playerstatus.curhealth <= 0 {
                playerdead=true;
            }

            if playerstatus.curstamina > playerstatus.stamina+playerstatus.staminamod {
                playerstatus.curstamina = playerstatus.stamina+playerstatus.staminamod;
            }
            if playerstatus.curmana > playerstatus.mana+playerstatus.manamod {
                playerstatus.curmana = playerstatus.mana+playerstatus.manamod;
            }
            if playerstatus.curhealth > playerstatus.health+playerstatus.healthmod {
                playerstatus.curhealth = playerstatus.health+playerstatus.healthmod;
            }

            //Menu
            while !turnend && !playerdead && !enemydefeat && !flee {
            playerstatus.ls(enemy);           
            print!("| [Menu]\n| 1) Action\n| 2) Item\n| 3) Inspect\n| 4) Talk\n| 5) Flee\n");
            let s = general::input();

            //Action
            if s == "1" {
                loop {
                    let mut i = 1;
                    playerstatus.ls(enemy);
                    println!("| | [Action]\n| | 0) <-");
                    for spell in spelllist {
                        let costsign = match spell.costtype {
                            spells::Costtype::Void => "",
                            spells::Costtype::Health => "HP",
                            spells::Costtype::Mana => "MP",
                            spells::Costtype::Stamina => "SP",
                        };
                        if costsign != "" {
                            println!("| | {}) {} [{} {}]", i, spell.name, spell.cost, costsign);
                        } else {
                            println!("| | {}) {}", i, spell.name);
                        }
                        i+=1;
                    }
                    let ss = general::input();
                    if ss == "0" {
                        break;
                    }
                    let ssint = match ss.parse::<usize> () {
                        Ok(ssint) => ssint,
                        Err(_) => {println!("\nNot a valid entry.\n"); break}
                    };
                    if ssint-1 >= spelllist.len() {
                        println!("\nNot a valid entry.\n");
                        break;
                    }
                    let costvalue = match spelllist[ssint-1].costtype {
                        spells::Costtype::Void => &0,
                        spells::Costtype::Health => &playerstatus.curhealth,
                        spells::Costtype::Mana => &playerstatus.curmana,
                        spells::Costtype::Stamina => &playerstatus.curstamina,
                    };
                    if &spelllist[ssint-1].cost <= costvalue {
                        action(playerstatus, &spelllist[ssint-1], enemy, &eqlist[0]);
                        turnend = true;
                        break;
                    } else {
                        println!("\nYou do not have sufficient resources to perform this action.");
                        general::pause();
                        break;
                    }
                }

            //Item
            } else if s == "2" {
                loop {
                    let mut i = 1;
                    playerstatus.ls(enemy);
                    println!("| | [Item]\n| | 0) <-");
                    for item in &mut itemlist.iter () {
                        println!("| | {}) {} [x{}]", i, item.name, item.amount);
                        i+=1;
                    }
                    let ss = general::input();
                    if ss == "0" {
                        break;
                    }
                    let ssint = match ss.parse::<usize> () {
                        Ok(ssint) => ssint,
                        Err(_) => {println!("\nNot a valid entry.\n"); break},
                    };
                    if ssint-1 >= itemlist.len() {
                        println!("\nNot a valid entry.\n");
                        break;
                    }
                    if itemlist[ssint-1].amount >= 1 {
                        item(playerstatus, &mut itemlist[ssint-1], enemy);
                        turnend = true;
                        break;
                    } else {
                        println!("\nYou do not have sufficient resources to perform this action.");
                        general::pause();
                        break;
                    }
                }
            
            //Inspect
            } else if s == "3" {
            let mut skip = false;
                loop {
                    if skip == true {
                        break;
                    }
                    playerstatus.ls(enemy);
                    let mut ii = 1;
                    println!("| | [Inspect]\n| | 0) <-\n| | 1) Self\n| | 2) Enemy\n| | 3) Action\n| | 4) Item\n| | 5) Equipment"); 
                    let ss = general::input();
                    if ss == "1" {
                        playerstatus.prntent();
                        break;
                    } else if ss == "2" {
                        enemy.prntent();
                        break;
                    } else if ss == "3" {
                        loop {
                            playerstatus.ls(enemy);
                            println!("| | | [Inspect Action]\n| | | 0) <-");
                            for spell in spelllist {
                                    let costsign = match spell.costtype {
                                    spells::Costtype::Void => "",
                                    spells::Costtype::Health => "HP",
                                    spells::Costtype::Mana => "MP",
                                    spells::Costtype::Stamina => "SP",
                                };
                                if costsign != "" {
                                    println!("| | | {}) {} [{} {}]", ii, spell.name, spell.cost, costsign);
                                } else {
                                    println!("| | | {}) {}", ii, spell.name);
                                }
                                ii+=1;
                            }
                            let sss = general::input();
                            if sss == "0" {
                                break;
                            }
                            let sssint = match sss.parse::<usize> () {
                                Ok(sssint) => sssint,
                                Err(_) => {println!("\nNot a valid entry.\n"); break},
                            };
                            if (sssint-1) >= spelllist.len() {
                                println!("\nNot a valid entry.\n");
                                skip=true;
                                break;
                            }
                            spelllist[sssint-1].prntspl();
                            skip=true;
                            break;
                        }
                    } else if ss == "4" {
                        loop {
                            playerstatus.ls(enemy);
                            println!("| | | [Inspect Item]\n| | | 0) <-");
                            for item in &mut itemlist.iter () {
                                println!("| | | {}) {} [x{}]", ii, item.name, item.amount);
                                ii+=1;
                            }  
                            let sss = general::input();
                            if sss == "0" {
                                break;
                            }
                            let sssint = match sss.parse::<usize> () {
                                Ok(sssint) => sssint,
                                Err(_) => {println!("\nNot a valid entry.\n"); break},
                            };
                            if (sssint-1) >= itemlist.len() {
                                println!("\nNot a valid entry.\n");
                                skip=true;
                                break;
                            }
                            itemlist[sssint-1].prntitem();
                            skip=true;
                            break;
                        }
                    } else if ss=="5" {
                        loop {
                            playerstatus.ls(enemy);
                            println!("| | | [Inspect Equipment]\n| | | 0) <-\n| | | 1) L-Hand: [{}]\n| | | 2) R-Hand: [{}]\n| | | 3) Armor: [{}]\n| | | 4) Helm: [{}]\n| | | 5) Acc-1: [{}]\n| | | 6) Acc-2: [{}]\n| | | 7) Inventory . . .",
                            eqlist[0].name, eqlist[1].name, eqlist[2].name, eqlist[3].name, eqlist[4].name, eqlist[5].name);
                            let sss = general::input();
                            if sss == "0" {
                                break;
                            } else if sss == "1" {
                                eqlist[0].prnteq();
                                skip=true;
                                break;
                            } else if sss == "2" {
                                eqlist[1].prnteq();
                                skip=true;
                                break;
                            } else if sss == "3" {
                                eqlist[2].prnteq();
                                skip=true;
                                break;
                            } else if sss == "4" {
                                eqlist[3].prnteq();
                                skip=true;
                                break;
                            } else if sss == "5" {
                                eqlist[4].prnteq();
                                skip=true;
                                break;
                            } else if sss == "6" {
                                eqlist[5].prnteq();
                                skip=true;
                                break;
                            } else if sss == "7" {
                            println!("| | | | [Inspect Inventory]\n| | | | 0) <-");
                            let mut i=1;
                             
                            for eqitem in &mut eqitems.iter() {
                              let typestring = match eqitem.eqtype {
                                equip::Eqtype::OHand => "One-Handed",
                                equip::Eqtype::THand => "Two-Handed",
                                equip::Eqtype::Accessory => "Accessory",
                                equip::Eqtype::Armor => "Armor",
                                equip::Eqtype::Helm => "Helm",
                              };
                              println!("| | | | {}) {} [{}]", i, eqitem.name, typestring);
                              i+=1;
                            }
                            loop {
                                let ssss = general::input();
                                if ssss == "0" {
                                    break;
                                }
                                let ssssint = match ssss.parse::<usize> () {
                                    Ok(ssssint) => ssssint,
                                    Err(_) => {println!("\nNot a valid entry.\n"); break},
                                };
                                if (ssssint-1) >= eqitems.len() {
                                    println!("\nNot a valid entry.\n");
                                    skip=true;
                                    break;
                                }
                                eqitems[ssssint-1].prnteq();
                                break;
                            }
                        }
                        }
                    } else if ss=="0" {
                        break;
                    } else {
                        println!("\nNot a valid entry.\n"); 
                        break;
                    }
                }

            //Talk    
            } else if s == "4" {
                let rngtalkcalc: f64 = ((playerstatus.charisma as f64 + playerstatus.charismamod as f64) as f64/
                                        ((enemy.charisma as f64+ enemy.charismamod as f64)*2.0))/
                                        ((enemy.curhealth as f64*4.0)/
                                        (enemy.health as f64 + enemy.healthmod as f64) as f64);
                if rngtalkcalc >= general::rand_f() {
                    println!("\n{}: \"{}\"", enemy.name, enemy.tasu);
                    general::pause();
                    println!("\nIt seems the enemy retreated.");
                    general::pause();
                    enemypersuade = true;
                    turnend = true;
                } else {
                    println!("\n{}: \"{}\"", enemy.name, enemy.tafa);
                    general::pause();
                    println!("\nIt didn't seem to do much...");
                    general::pause();
                    turnend = true;
                }

            //Flee
            } else if s == "5" {
                let rngfleecalc: f64 = (playerstatus.speed as f64 + playerstatus.speedmod as f64) as f64/((enemy.speed as f64+ enemy.speedmod as f64)*2.0);
                println!("\nTrying to flee...");
                general::pause();
                if rngfleecalc >= general::rand_f() {
                    println!("\nYou successfully got away!");
                    general::pause();
                    flee = true;
                    turnend = true;
                } else {
                    println!("\nYou couldn't get away...");
                    general::pause();
                    turnend = true;
                }
            }
        }

        if playerstatus.curhealth <= 0 {
            playerdead = true;
            playerstatus.curhealth = 0;
        }
        if enemy.curhealth <= 0 {
            enemydefeat = true;
            enemy.curhealth = 0;
        }
        if playerdead || enemydefeat || enemypersuade || flee {
            break;
        }
        }

        if playerstatus.curstamina > playerstatus.stamina+playerstatus.staminamod {
            playerstatus.curstamina = playerstatus.stamina+playerstatus.staminamod;
        }
        if playerstatus.curmana > playerstatus.mana+playerstatus.manamod {
            playerstatus.curmana = playerstatus.mana+playerstatus.manamod;
        }
        if playerstatus.curhealth > playerstatus.health+playerstatus.healthmod {
            playerstatus.curhealth = playerstatus.health+playerstatus.healthmod;
        }

        //Enemy Break
        if enbroke {
            enbroke = false;
            println!("\n{} recovered some SP.", enemy.name);
            general::pause();
            enemy._curstamina(enemy.stamina/3);
        }
        if enemy.curstamina <= 0 {
            enbroke = true;
            enemy.curstamina = 0;
            println!("\n{} SP was depleted!", enemy.name);
            general::pause();
            println!("\n*Stance Break* [Max HP -{}][Max MP -{}][Max SP -{}]", enemy.health/3, enemy.mana/3, enemy.stamina/3);
            enemy._healthmod(-enemy.health/3);
            enemy._manamod(-enemy.mana/3);
            enemy._staminamod(-enemy.stamina/3);
            general::pause();
            println!("{} can't act after their stance was broken!", enemy.name);
            general::pause();
        }

        //Enemy condition
        if enparalyzecount == 4 {
            enparalyzecount = 0;
            enparalyzegone = 1;
        }
        if enpoisongone == 1 {
            println!("\n{} got over the poison!", enemy.name);
            general::pause();
            enpoisongone=0;
            enpoisoncount=0;
            enemy._condition(entity::Condition::Normal);
        }
        if enparalyzegone == 1 {
            println!("\n{} got over the paralysis!", enemy.name);
            general::pause();
            enparalyzegone=0;
            enparalyzecount=0;
            enemy._condition(entity::Condition::Normal);
        }
        if ensleepgone == 1 {
            println!("\n{} woke up!", enemy.name);
            general::pause();
            ensleepgone=0;
            ensleepcount=0;
            enemy._condition(entity::Condition::Normal);
        }
        if enregengone == 1 {
            println!("\n{}'s regneration wore off...", enemy.name);
            general::pause();
            enregengone=0;
            enregencount=0;
            enemy._condition(entity::Condition::Normal);
        }
        let enemycondition = match enemy.condition {
            entity::Condition::Poison => "*Poisoned*",
            entity::Condition::Sleep => "*Asleep*",
            entity::Condition::Paralyze => "*Paralyzed*",
            entity::Condition::Regenerate => "*Regenerating*",
            _ => "",
        };   
        if enemycondition == "*Poisoned*" {
            enemy._curhealth(-(enemy.health/10) as i32);
            println!("\n{} lost {} HP from poison...", enemy.name, enemy.health/10);
            general::pause();
            enpoisoncount += 1;
            if enpoisoncount ==  6 {
                enpoisoncount = 0;
                enpoisongone = 0;
            }
        } else if enemycondition == "*Asleep*" {
            println!("\n{} is asleep and can't get up!", enemy.name);
            general::pause();
            ensleepcount += 1;
            if ensleepcount == 2 {
                ensleepcount = 0;
                ensleepgone = 1;
            }
            turnend = false;
            continue;
        } else if enemycondition == "*Paralyzed*" {
            println!("\n{} is paralyzed!", enemy.name);
            general::pause();
            if (enemy.speed as f64 + enemy.speedmod as f64)as f64/((playerstatus.speed as f64 + playerstatus.speedmod as f64)*2.0) >= general::rand_f() {
                println!("\n{} managed to act anyway.", enemy.name);
                general::pause();
                enparalyzecount += 1;
            } else {
                println!("\n{} couldn't move due to paralysis...", enemy.name);
                general::pause();
                turnend = false;
                enparalyzecount += 1;
                continue;
            }
        } else if enemycondition == "*Regenerating*" {
            enemy._curhealth(enemy.health/10);
            if enemy.curhealth > enemy.health {
                enemy._curhealth(-(enemy.curhealth - enemy.health));
            }
            println!("\n{} regenerated {} HP.", enemy.name, enemy.health/10);
            general::pause();
            enregencount += 1;
            if enregencount == 5 {
                enregencount = 0;
                enregengone = 1;
            }
        }
        if enemy.curstamina > enemy.stamina+enemy.staminamod {
            enemy.curstamina = enemy.stamina+enemy.staminamod;
        }
        if enemy.curmana > enemy.mana+enemy.manamod {
            enemy.curmana = enemy.mana+enemy.manamod;
        }
        if enemy.curhealth > enemy.health+enemy.healthmod {
            enemy.curhealth = enemy.health+enemy.healthmod;
        }
        if enemy.curhealth <= 0 {
            enemydefeat = true;
            break;
        }
        
        //Enemy action
        if !enbroke {
            enemyfunc(playerstatus, enemy, playerdead, enemydefeat, enemypersuade, flee);
            if playerdead || enemydefeat || enemypersuade || flee {
                break;
            }
            loop {
                let mut enmoverng = rand::thread_rng();
                enemyspelllist.shuffle(&mut enmoverng);
                let enmovecost = match enemyspelllist[0].costtype {
                    spells::Costtype::Void => 0,
                    spells::Costtype::Health => enemy.curhealth,
                    spells::Costtype::Mana => enemy.curmana,
                    spells::Costtype::Stamina => enemy.curstamina,
                };
                if enemyspelllist[0].cost <= enmovecost {
                    action(enemy,&enemyspelllist[0],playerstatus,&eqlist[0]);
                    break;
                } 
            }
        }

        //Reset turn
        turnend = false;
    }

    //Battle result
    if playerdead {
        println!("\nYou died...");
        general::pause();
        println!("\n- G A M E O V E R - ");
        general::pause();
    } else if enemydefeat {
        println!("\nYou defeated {}!", enemy.name);
        general::pause();
        playerstatus._gold(enemy.gold);
        println!("\nYou gained {} gold.", enemy.gold);
        general::pause();
        playerstatus._experience(enemy.experience);
        println!("\nYou gained {} experience.", enemy.experience);
        general::pause();
    } else if enemypersuade {
        println!("\nYou gained no gold.");
        general::pause();
        println!("\nYou gained {} experience.", enemy.experience*3/2);
        playerstatus._experience(enemy.experience*3/2);
        general::pause();
    } else if flee {
        println!("\nYou gained no gold.");
        general::pause();
        println!("\nYou gained no experience.");
        general::pause();
    }
    enemy.reset_cur();
    enemy.reset_mod();
    playerstatus.reset_mod();
}

//Subject Verb Object 
fn action (actor: &mut entity::Entity, action: &spells::Spell, target: &mut entity::Entity, wpn: &equip::Equipment) {
    
    //Cost
    match action.costtype {
        spells::Costtype::Void => (),
        spells::Costtype::Health => actor._curhealth(-action.cost),
        spells::Costtype::Mana => actor._curmana(-action.cost),
        spells::Costtype::Stamina => actor._curstamina(-action.cost),
    }
    
    //Define target
    let selftarg = match action.target {
        spells::Target::Oneself => true,
        _ => false,
    };

    //Accuracy
    let mut hitrate: f64 = action.acc*((actor.speed as f64 + actor.speedmod as f64)/
                             (target.speed as f64 + target.speedmod as f64));

    let targetsleep = match target.condition {
        entity::Condition::Sleep => true,
        _ => false,
    };    
    
    if selftarg {
        hitrate = 1.0;
        println!("\n{} used {}.", actor.name, action.name);
        general::pause();
    } else {
        if targetsleep {
            hitrate = 1.0;
        }
        println!("\n{} used {} on {}.", actor.name, action.name, target.name);
        general::pause();
    }
    if hitrate >= general::rand_f() {

        //Damage
        let offcalcstat: i32 = match action.base {
            spells::Base::Void => 0,
            spells::Base::Health => (actor.health+actor.healthmod - actor.curhealth),
            spells::Base::Mana => actor.curmana,
            spells::Base::Stamina => actor.curstamina,
            spells::Base::Strength => actor.strength+actor.strengthmod,
            spells::Base::Defense => actor.defense+actor.defensemod,
            spells::Base::Intellect => actor.intellect+actor.intellectmod,
            spells::Base::Resistance => actor.resistance+actor.resistancemod,
            spells::Base::Faith => actor.faith+actor.faithmod,
            spells::Base::Charisma => actor.charisma+actor.charismamod,
            spells::Base::Speed => actor.speed+actor.speedmod,
            spells::Base::Dexterity => actor.dexterity+actor.dexteritymod,
            spells::Base::Skill => actor.skill+actor.skillmod,
            spells::Base::Luck => actor.luck+actor.luckmod,
        };
        let defcalcstat: i32 = match action.base {
            spells::Base::Void => 0,
            spells::Base::Health => target.luck+target.luckmod,
            spells::Base::Mana => target.luck+target.luckmod,
            spells::Base::Stamina => target.luck+target.luckmod,
            spells::Base::Strength => target.defense+target.defensemod,
            spells::Base::Defense => target.defense+target.defensemod,
            spells::Base::Intellect => target.resistance+target.resistancemod,
            spells::Base::Resistance => target.resistance+target.resistancemod,
            spells::Base::Faith => target.charisma+target.charismamod,
            spells::Base::Charisma => target.charisma+target.charismamod,
            spells::Base::Speed => target.dexterity+target.dexteritymod,
            spells::Base::Dexterity => target.defense+target.defensemod,
            spells::Base::Skill => target.skill+target.skillmod,
            spells::Base::Luck => target.luck+target.luckmod,
        };
        let mut curdelta: i32 = ((offcalcstat*3+action.value*3)/10)+2 - (defcalcstat/2);
        if curdelta < 0 {
            curdelta = 0;
        }

        //Element application
        let wpel = match wpn.element {
            equip::Element::Neutral => "neutral",
            equip::Element::Fire => "fire",
            equip::Element::Water => "water",
            equip::Element::Air => "air",
            equip::Element::Earth => "earth",
            equip::Element::Light => "light",
            equip::Element::Dark => "dark",
        };
        let mut weakcalc = false;
        let mut strongcalc = false;
        let acel = match action.element {
            spells::Element::Neutral => "neutral",
            spells::Element::Fire => "fire",
            spells::Element::Water => "water",
            spells::Element::Air => "air",
            spells::Element::Earth => "earth",
            spells::Element::Light => "light",
            spells::Element::Dark => "dark",
            spells::Element::Match => wpel,
        }; 
        let tael = match target.element {
                entity::Element::Neutral => "neutral",
                entity::Element::Fire => "fire",
                entity::Element::Water => "water",
                entity::Element::Air => "air",
                entity::Element::Earth => "earth",
                entity::Element::Light => "light",
                entity::Element::Dark => "dark",
        };
        if tael == acel && tael != "neutral" {
            curdelta/=2;
            weakcalc = true;
        }
        if tael == "fire" && acel == "water" {
            curdelta*=2;
            strongcalc = true;
        } else if tael == "water" && acel == "fire" {
            curdelta*=2;
            strongcalc = true;
        } else if tael == "air" && acel == "earth" {
            curdelta*=2;
            strongcalc = true;
        } else if tael == "earth" && acel == "air" {
            curdelta*=2;
            strongcalc = true;
        } else if tael == "light" && acel == "dark" {
            curdelta*=2;
            strongcalc = true;
        } else if tael == "dark" && acel == "light" {
            curdelta*=2;
            strongcalc = true;
        }
        

        //Target application
        if selftarg == false {
            if action.invert == false {
                match action.valuetarg {
                    spells::Valuetarg::Void => target._curhealth(0),
                    spells::Valuetarg::Health => target._curhealth(-curdelta),
                    spells::Valuetarg::Mana => target._curmana(-curdelta),
                    spells::Valuetarg::Stamina => target._curstamina(-curdelta),
                }
            } else if action.invert == true {
                match action.valuetarg {
                    spells::Valuetarg::Void => target._curhealth(0),
                    spells::Valuetarg::Health => target._curhealth(curdelta),
                    spells::Valuetarg::Mana => target._curmana(curdelta),
                    spells::Valuetarg::Stamina => target._curstamina(curdelta),
                }
            }
        } else if selftarg == true {
            if action.invert == false {
                match action.valuetarg {
                    spells::Valuetarg::Void => actor._curhealth(0),
                    spells::Valuetarg::Health => actor._curhealth(-curdelta),
                    spells::Valuetarg::Mana => actor._curmana(-curdelta),
                    spells::Valuetarg::Stamina => actor._curstamina(-curdelta),
                }
            } else if action.invert == true {
                match action.valuetarg {
                    spells::Valuetarg::Void => actor._curhealth(0),
                    spells::Valuetarg::Health => actor._curhealth(curdelta),
                    spells::Valuetarg::Mana => actor._curmana(curdelta),
                    spells::Valuetarg::Stamina => actor._curstamina(curdelta),
                }
            }
        }

        //Status modification
        let playstatself = match action.playstattarg {
            spells::Playstattarg::Void => true,
            spells::Playstattarg::Oneself => true,
            spells::Playstattarg::Enemy => false,
        };
        let modstat = match action.playstateff {
            spells::Playstateff::Void => 0.0,
            spells::Playstateff::Raise => 0.25,
            spells::Playstateff::Raisep => 0.5,
            spells::Playstateff::Lower => -0.25,
            spells::Playstateff::Lowerp => -0.5,
        };
        let mut modstatstring = "";
        if modstat > 0.0 {
            modstatstring = "+";
        }
        if playstatself == false {
            match action.playstat {
                spells::Playstat::Void => target._healthmod(0),
                spells::Playstat::Maxhealth => target._healthmod((target.health as f64*modstat) as i32),
                spells::Playstat::Maxmana => target._manamod((target.mana as f64*modstat) as i32),
                spells::Playstat::Maxstamina => target._staminamod((target.stamina as f64*modstat) as i32),
                spells::Playstat::Strength => target._strengthmod((target.strength as f64*modstat) as i32),
                spells::Playstat::Defense => target._defensemod((target.strength as f64*modstat) as i32),
                spells::Playstat::Intellect => target._intellectmod((target.intellect as f64*modstat) as i32),
                spells::Playstat::Resistance => target._resistancemod((target.resistance as f64*modstat) as i32),
                spells::Playstat::Faith => target._faithmod((target.faith as f64*modstat) as i32),
                spells::Playstat::Charisma => target._charismamod((target.charisma as f64*modstat) as i32),
                spells::Playstat::Speed => target._speedmod((target.speed as f64*modstat) as i32),
                spells::Playstat::Dexterity => target._dexteritymod((target.dexterity as f64*modstat) as i32),
                spells::Playstat::Skill => target._skillmod((target.skill as f64*modstat) as i32),
                spells::Playstat::Luck => target._luckmod((target.luck as f64*modstat) as i32),
            }
            match action.playstat {
                spells::Playstat::Void => (),
                spells::Playstat::Maxhealth => println!("\n[HP {}{}]", modstatstring, (target.health as f64*modstat) as i32),
                spells::Playstat::Maxmana => println!("\n[MP {}{}]", modstatstring, (target.mana as f64*modstat) as i32),
                spells::Playstat::Maxstamina => println!("\n[SP {}{}]", modstatstring, (target.stamina as f64*modstat) as i32),
                spells::Playstat::Strength => println!("\n[STR {}{}]", modstatstring, (target.strength as f64*modstat) as i32),
                spells::Playstat::Defense => println!("\n[DEF {}{}]", modstatstring, (target.strength as f64*modstat) as i32),
                spells::Playstat::Intellect => println!("\n[INT {}{}]", modstatstring, (target.intellect as f64*modstat) as i32),
                spells::Playstat::Resistance => println!("\n[RES {}{}]", modstatstring, (target.resistance as f64*modstat) as i32),
                spells::Playstat::Faith => println!("\n[FAI {}{}]", modstatstring, (target.faith as f64*modstat) as i32),
                spells::Playstat::Charisma => println!("\n[CHA {}{}]", modstatstring, (target.charisma as f64*modstat) as i32),
                spells::Playstat::Speed => println!("\n[SPD {}{}]", modstatstring, (target.speed as f64*modstat) as i32),
                spells::Playstat::Dexterity => println!("\n[DEX {}{}]", modstatstring, (target.dexterity as f64*modstat) as i32),
                spells::Playstat::Skill => println!("\n[SKL {}{}]", modstatstring, (target.skill as f64*modstat) as i32),
                spells::Playstat::Luck => println!("\n[LCK {}{}]", modstatstring, (target.luck as f64*modstat) as i32),
            }
            match action.playstat {
                spells::Playstat::Void => (),
                _ => general::pause(),
            }
        } else if playstatself == true {
            match action.playstat {
                spells::Playstat::Void => actor._healthmod(0),
                spells::Playstat::Maxhealth => actor._healthmod((actor.health as f64*modstat) as i32),
                spells::Playstat::Maxmana => actor._manamod((actor.mana as f64*modstat) as i32),
                spells::Playstat::Maxstamina => actor._staminamod((actor.stamina as f64*modstat) as i32),
                spells::Playstat::Strength => actor._strengthmod((actor.strength as f64*modstat) as i32),
                spells::Playstat::Defense => actor._defensemod((actor.strength as f64*modstat) as i32),
                spells::Playstat::Intellect => actor._intellectmod((actor.intellect as f64*modstat) as i32),
                spells::Playstat::Resistance => actor._resistancemod((actor.resistance as f64*modstat) as i32),
                spells::Playstat::Faith => actor._faithmod((actor.faith as f64*modstat) as i32),
                spells::Playstat::Charisma => actor._charismamod((actor.charisma as f64*modstat) as i32),
                spells::Playstat::Speed => actor._speedmod((actor.speed as f64*modstat) as i32),
                spells::Playstat::Dexterity => actor._dexteritymod((actor.dexterity as f64*modstat) as i32),
                spells::Playstat::Skill => actor._skillmod((actor.skill as f64*modstat) as i32),
                spells::Playstat::Luck => actor._luckmod((actor.luck as f64*modstat) as i32),
            }
            match action.playstat {
                spells::Playstat::Void => (),
                spells::Playstat::Maxhealth => println!("\n[HP {}{}]", modstatstring, (actor.health as f64*modstat) as i32),
                spells::Playstat::Maxmana => println!("\n[MP {}{}]", modstatstring, (actor.mana as f64*modstat) as i32),
                spells::Playstat::Maxstamina => println!("\n[SP {}{}]", modstatstring, (actor.stamina as f64*modstat) as i32),
                spells::Playstat::Strength => println!("\n[STR {}{}]", modstatstring, (actor.strength as f64*modstat) as i32),
                spells::Playstat::Defense => println!("\n[DEF {}{}]", modstatstring, (actor.strength as f64*modstat) as i32),
                spells::Playstat::Intellect => println!("\n[INT {}{}]", modstatstring, (actor.intellect as f64*modstat) as i32),
                spells::Playstat::Resistance => println!("\n[RES {}{}]", modstatstring, (actor.resistance as f64*modstat) as i32),
                spells::Playstat::Faith => println!("\n[FAI {}{}]", modstatstring, (actor.faith as f64*modstat) as i32),
                spells::Playstat::Charisma => println!("\n[CHA {}{}]", modstatstring, (actor.charisma as f64*modstat) as i32),
                spells::Playstat::Speed => println!("\n[SPD {}{}]", modstatstring, (actor.speed as f64*modstat) as i32),
                spells::Playstat::Dexterity => println!("\n[DEX {}{}]", modstatstring, (actor.dexterity as f64*modstat) as i32),
                spells::Playstat::Skill => println!("\n[SKL {}{}]", modstatstring, (actor.skill as f64*modstat) as i32),
                spells::Playstat::Luck => println!("\n[LCK {}{}]", modstatstring, (actor.luck as f64*modstat) as i32),
            }
            match action.playstat {
                spells::Playstat::Void => (),
                _ => general::pause(),
            }
        }

        //Condition modification
        let statself = match action.statustarg {
            spells::Statustarg::Void => true,
            spells::Statustarg::Oneself => true,
            spells::Statustarg::Enemy => false,
        };
        if statself == true {
            match action.status {
                spells::Status::Void => actor.health = actor.health,
                spells::Status::Poison => actor._condition(entity::Condition::Poison),
                spells::Status::Paralyze => actor._condition(entity::Condition::Paralyze),
                spells::Status::Sleep => actor._condition(entity::Condition::Sleep),
                spells::Status::Normal => actor._condition(entity::Condition::Normal),
                spells::Status::Regenerate => actor._condition(entity::Condition::Regenerate),
            }
        } else if statself == false {
            match action.status {
                spells::Status::Void => target.health = target.health,
                spells::Status::Poison => target._condition(entity::Condition::Poison),
                spells::Status::Paralyze => target._condition(entity::Condition::Paralyze),
                spells::Status::Sleep => target._condition(entity::Condition::Sleep),
                spells::Status::Normal => target._condition(entity::Condition::Normal),
                spells::Status::Regenerate => target._condition(entity::Condition::Regenerate),
            }
        }

        //Result
        let valuestring = match action.valuetarg {
            spells::Valuetarg::Void => "",
            spells::Valuetarg::Health => "HP",
            spells::Valuetarg::Mana => "MP",
            spells::Valuetarg::Stamina => "SP",
        };
        if selftarg == false {
            if curdelta > 0 {
                if weakcalc == true {
                    println!("\n{} was resistant to {}...\n({} {} Damaged -> {} {} Damaged)", target.name, acel, curdelta*2, valuestring, curdelta, valuestring);
                } else if strongcalc == true {
                    println!("\n{} was vulnerable to {}!\n({} {} Damaged -> {} {} Damaged)", target.name, acel, curdelta/2, valuestring, curdelta, valuestring);
                } else {
                    println!("\n{} did {} {} damage to {}!", actor.name, curdelta, valuestring, target.name);
                }
            } else if curdelta < 0 {
                if weakcalc == true {
                    println!("\n{} was resistant to {}...\n({} {} Restored -> {} {} Restored)", target.name, acel, curdelta*2, valuestring, curdelta, valuestring);
                } else if strongcalc == true {
                    println!("\n{} was vulnerable to {}!\n({} {} Restored -> {} {} Restored)", target.name, acel, curdelta/2, valuestring, curdelta, valuestring);
                } else {
                    println!("\n{} healed {} points!", actor.name, curdelta);
                }
            } else {
                println!("\n{} did no damage to {}.", actor.name, target.name);
            }
        } else if selftarg == true {
            if curdelta < 0 {
                println!("\n{} dealt {} {} self-damage...", actor.name, curdelta, valuestring);
            } else if curdelta > 0 {
                println!("\n{} restored {} {}!", actor.name, curdelta, valuestring);
            }        
        }
        if target.curhealth > 0 && curdelta > 0 && targetsleep {
            general::pause();
            println!("\n{} woke up from the attack!", target.name);
            target._condition(entity::Condition::Normal);
        }
    } else {
        println!("\n{} dodged {}!", target.name, action.name);
    }
    if actor.curhealth > actor.health {
        actor.curhealth = actor.health;
    }
    if target.curhealth > target.health {
        target.curhealth = target.health;
    }
    general::pause();
}

fn item (actor: &mut entity::Entity, action: &mut items::Item, target: &mut entity::Entity) {
 
    //Damage
    let offcalcstat: i32 = match action.base {
        items::Base::Void => 0,
        items::Base::Health => (actor.health+actor.healthmod - actor.curhealth),
        items::Base::Mana => actor.curmana,
        items::Base::Stamina => actor.curstamina,
        items::Base::Strength => actor.strength+actor.strengthmod,
        items::Base::Defense => actor.defense+actor.defensemod,
        items::Base::Intellect => actor.intellect+actor.intellectmod,
        items::Base::Resistance => actor.resistance+actor.resistancemod,
        items::Base::Faith => actor.faith+actor.faithmod,
        items::Base::Charisma => actor.charisma+actor.charismamod,
        items::Base::Speed => actor.speed+actor.speedmod,
        items::Base::Dexterity => actor.dexterity+actor.dexteritymod,
        items::Base::Skill => actor.skill+actor.skillmod,
        items::Base::Luck => actor.luck+actor.luckmod,
    };
    let defcalcstat: i32 = match action.base {
        items::Base::Void => 0,
        items::Base::Health => target.luck+target.luckmod,
        items::Base::Mana => target.luck+target.luckmod,
        items::Base::Stamina => target.luck+target.luckmod,
        items::Base::Strength => target.defense+target.defensemod,
        items::Base::Defense => target.defense+target.defensemod,
        items::Base::Intellect => target.resistance+target.resistancemod,
        items::Base::Resistance => target.resistance+target.resistancemod,
        items::Base::Faith => target.charisma+target.charismamod,
        items::Base::Charisma => target.charisma+target.charismamod,
        items::Base::Speed => target.dexterity+target.dexteritymod,
        items::Base::Dexterity => target.defense+target.defensemod,
        items::Base::Skill => target.skill+target.skillmod,
        items::Base::Luck => target.luck+target.luckmod,
    };
    let mut curdelta: i32 = ((offcalcstat*3+action.value*3)/10)+2 - (defcalcstat/2);
    if curdelta < 0 {
        curdelta = 0;
    }

    //Define target
    let selftarg = match action.target {
        items::Target::Oneself => true,
        _ => false,
    };
    
    //Element application
    let mut weakcalc = false;
    let mut strongcalc = false;
    let acel = match action.element {
        items::Element::Neutral => "neutral",
        items::Element::Fire => "fire",
        items::Element::Water => "water",
        items::Element::Air => "air",
        items::Element::Earth => "earth",
        items::Element::Light => "light",
        items::Element::Dark => "dark",
    }; 
    let tael = match target.element {
            entity::Element::Neutral => "neutral",
            entity::Element::Fire => "fire",
            entity::Element::Water => "water",
            entity::Element::Air => "air",
            entity::Element::Earth => "earth",
            entity::Element::Light => "light",
            entity::Element::Dark => "dark",
    };
    if tael == acel && tael != "neutral" {
        curdelta/=2;
        weakcalc = true;
    }
    if tael == "fire" && acel == "water" {
        curdelta*=2;
        strongcalc = true;
    } else if tael == "water" && acel == "fire" {
        curdelta*=2;
        strongcalc = true;
    } else if tael == "air" && acel == "earth" {
        curdelta*=2;
        strongcalc = true;
    } else if tael == "earth" && acel == "air" {
        curdelta*=2;
        strongcalc = true;
    } else if tael == "light" && acel == "dark" {
        curdelta*=2;
        strongcalc = true;
    } else if tael == "dark" && acel == "light" {
        curdelta*=2;
        strongcalc = true;
    }

    //Target application
    if selftarg == false {
        if action.invert == false {
            match action.valuetarg {
                items::Valuetarg::Void => target._curhealth(0),
                items::Valuetarg::Health => target._curhealth(-curdelta),
                items::Valuetarg::Mana => target._curmana(-curdelta),
                items::Valuetarg::Stamina => target._curstamina(-curdelta),
            }
        } else if action.invert == true {
            match action.valuetarg {
                items::Valuetarg::Void => target._curhealth(0),
                items::Valuetarg::Health => target._curhealth(curdelta),
                items::Valuetarg::Mana => target._curmana(curdelta),
                items::Valuetarg::Stamina => target._curstamina(curdelta),
            }
        }
    } else if selftarg == true {
        if action.invert == false {
            match action.valuetarg {
                items::Valuetarg::Void => actor._curhealth(0),
                items::Valuetarg::Health => actor._curhealth(-curdelta),
                items::Valuetarg::Mana => actor._curmana(-curdelta),
                items::Valuetarg::Stamina => actor._curstamina(-curdelta),
            }
        } else if action.invert == true {
            match action.valuetarg {
                items::Valuetarg::Void => actor._curhealth(0),
                items::Valuetarg::Health => actor._curhealth(curdelta),
                items::Valuetarg::Mana => actor._curmana(curdelta),
                items::Valuetarg::Stamina => actor._curstamina(curdelta),
            }
        }
    }

    //Status modification
        let playstatself = match action.playstattarg {
            items::Playstattarg::Void => true,
            items::Playstattarg::Oneself => true,
            items::Playstattarg::Enemy => false,
        };
        let modstat = match action.playstateff {
            items::Playstateff::Void => 0.0,
            items::Playstateff::Raise => 0.25,
            items::Playstateff::Raisep => 0.5,
            items::Playstateff::Lower => -0.25,
            items::Playstateff::Lowerp => -0.5,
        };
        let mut modstatstring = "";
        if modstat > 0.0 {
            modstatstring = "+";
        }
        if playstatself == false {
            match action.playstat {
                items::Playstat::Void => target._healthmod(0),
                items::Playstat::Maxhealth => target._healthmod((target.health as f64*modstat) as i32),
                items::Playstat::Maxmana => target._manamod((target.mana as f64*modstat) as i32),
                items::Playstat::Maxstamina => target._staminamod((target.stamina as f64*modstat) as i32),
                items::Playstat::Strength => target._strengthmod((target.strength as f64*modstat) as i32),
                items::Playstat::Defense => target._defensemod((target.strength as f64*modstat) as i32),
                items::Playstat::Intellect => target._intellectmod((target.intellect as f64*modstat) as i32),
                items::Playstat::Resistance => target._resistancemod((target.resistance as f64*modstat) as i32),
                items::Playstat::Faith => target._faithmod((target.faith as f64*modstat) as i32),
                items::Playstat::Charisma => target._charismamod((target.charisma as f64*modstat) as i32),
                items::Playstat::Speed => target._speedmod((target.speed as f64*modstat) as i32),
                items::Playstat::Dexterity => target._dexteritymod((target.dexterity as f64*modstat) as i32),
                items::Playstat::Skill => target._skillmod((target.skill as f64*modstat) as i32),
                items::Playstat::Luck => target._luckmod((target.luck as f64*modstat) as i32),
            }
            match action.playstat {
                items::Playstat::Void => (),
                items::Playstat::Maxhealth => println!("\n[HP {}{}]", modstatstring, (target.health as f64*modstat) as i32),
                items::Playstat::Maxmana => println!("\n[MP {}{}]", modstatstring, (target.mana as f64*modstat) as i32),
                items::Playstat::Maxstamina => println!("\n[SP {}{}]", modstatstring, (target.stamina as f64*modstat) as i32),
                items::Playstat::Strength => println!("\n[STR {}{}]", modstatstring, (target.strength as f64*modstat) as i32),
                items::Playstat::Defense => println!("\n[DEF {}{}]", modstatstring, (target.strength as f64*modstat) as i32),
                items::Playstat::Intellect => println!("\n[INT {}{}]", modstatstring, (target.intellect as f64*modstat) as i32),
                items::Playstat::Resistance => println!("\n[RES {}{}]", modstatstring, (target.resistance as f64*modstat) as i32),
                items::Playstat::Faith => println!("\n[FAI {}{}]", modstatstring, (target.faith as f64*modstat) as i32),
                items::Playstat::Charisma => println!("\n[CHA {}{}]", modstatstring, (target.charisma as f64*modstat) as i32),
                items::Playstat::Speed => println!("\n[SPD {}{}]", modstatstring, (target.speed as f64*modstat) as i32),
                items::Playstat::Dexterity => println!("\n[DEX {}{}]", modstatstring, (target.dexterity as f64*modstat) as i32),
                items::Playstat::Skill => println!("\n[SKL {}{}]", modstatstring, (target.skill as f64*modstat) as i32),
                items::Playstat::Luck => println!("\n[LCK {}{}]", modstatstring, (target.luck as f64*modstat) as i32),
            }
            match action.playstat {
                items::Playstat::Void => (),
                _ => general::pause(),
            }
        } else if playstatself == true {
            match action.playstat {
                items::Playstat::Void => actor._healthmod(0),
                items::Playstat::Maxhealth => actor._healthmod((actor.health as f64*modstat) as i32),
                items::Playstat::Maxmana => actor._manamod((actor.mana as f64*modstat) as i32),
                items::Playstat::Maxstamina => actor._staminamod((actor.stamina as f64*modstat) as i32),
                items::Playstat::Strength => actor._strengthmod((actor.strength as f64*modstat) as i32),
                items::Playstat::Defense => actor._defensemod((actor.strength as f64*modstat) as i32),
                items::Playstat::Intellect => actor._intellectmod((actor.intellect as f64*modstat) as i32),
                items::Playstat::Resistance => actor._resistancemod((actor.resistance as f64*modstat) as i32),
                items::Playstat::Faith => actor._faithmod((actor.faith as f64*modstat) as i32),
                items::Playstat::Charisma => actor._charismamod((actor.charisma as f64*modstat) as i32),
                items::Playstat::Speed => actor._speedmod((actor.speed as f64*modstat) as i32),
                items::Playstat::Dexterity => actor._dexteritymod((actor.dexterity as f64*modstat) as i32),
                items::Playstat::Skill => actor._skillmod((actor.skill as f64*modstat) as i32),
                items::Playstat::Luck => actor._luckmod((actor.luck as f64*modstat) as i32),
            }
            match action.playstat {
                items::Playstat::Void => (),
                items::Playstat::Maxhealth => println!("\n[HP {}{}]", modstatstring, (actor.health as f64*modstat) as i32),
                items::Playstat::Maxmana => println!("\n[MP {}{}]", modstatstring, (actor.mana as f64*modstat) as i32),
                items::Playstat::Maxstamina => println!("\n[SP {}{}]", modstatstring, (actor.stamina as f64*modstat) as i32),
                items::Playstat::Strength => println!("\n[STR {}{}]", modstatstring, (actor.strength as f64*modstat) as i32),
                items::Playstat::Defense => println!("\n[DEF {}{}]", modstatstring, (actor.strength as f64*modstat) as i32),
                items::Playstat::Intellect => println!("\n[INT {}{}]", modstatstring, (actor.intellect as f64*modstat) as i32),
                items::Playstat::Resistance => println!("\n[RES {}{}]", modstatstring, (actor.resistance as f64*modstat) as i32),
                items::Playstat::Faith => println!("\n[FAI {}{}]", modstatstring, (actor.faith as f64*modstat) as i32),
                items::Playstat::Charisma => println!("\n[CHA {}{}]", modstatstring, (actor.charisma as f64*modstat) as i32),
                items::Playstat::Speed => println!("\n[SPD {}{}]", modstatstring, (actor.speed as f64*modstat) as i32),
                items::Playstat::Dexterity => println!("\n[DEX {}{}]", modstatstring, (actor.dexterity as f64*modstat) as i32),
                items::Playstat::Skill => println!("\n[SKL {}{}]", modstatstring, (actor.skill as f64*modstat) as i32),
                items::Playstat::Luck => println!("\n[LCK {}{}]", modstatstring, (actor.luck as f64*modstat) as i32),
            }
            match action.playstat {
                items::Playstat::Void => (),
                _ => general::pause(),
            }
        }
    
    //Condition modification
    let statself = match action.statustarg {
        items::Statustarg::Void => true,
        items::Statustarg::Oneself => true,
        items::Statustarg::Enemy => false,
    };
    if statself == true {
        match action.status {
            items::Status::Void => actor.health = actor.health,
            items::Status::Poison => actor._condition(entity::Condition::Poison),
            items::Status::Paralyze => actor._condition(entity::Condition::Paralyze),
            items::Status::Sleep => actor._condition(entity::Condition::Sleep),
            items::Status::Normal => actor._condition(entity::Condition::Normal),
            items::Status::Regenerate => actor._condition(entity::Condition::Regenerate),
        }
    } else if statself == false {
        match action.status {
            items::Status::Void => target.health = target.health,
            items::Status::Poison => target._condition(entity::Condition::Poison),
            items::Status::Paralyze => target._condition(entity::Condition::Paralyze),
            items::Status::Sleep => target._condition(entity::Condition::Sleep),
            items::Status::Normal => target._condition(entity::Condition::Normal),
            items::Status::Regenerate => target._condition(entity::Condition::Regenerate),
        }
    }

    //Use item
    action.up_amount(-1);
    
    //Result
    let valuestring = match action.valuetarg {
        items::Valuetarg::Void => "",
        items::Valuetarg::Health => "HP",
        items::Valuetarg::Mana => "MP",
        items::Valuetarg::Stamina => "SP",
    };
    if selftarg == false {
        if curdelta > 0 {
            if weakcalc == true {
                println!("\n{} was resistant to {}...\n({} {} Damaged -> {} {} Damaged)", target.name, acel, curdelta*2, valuestring, curdelta, valuestring);
            } else if strongcalc == true {
                println!("\n{} was vulnerable to {}!\n({} {} Damaged -> {} {} Damaged)", target.name, acel, curdelta/2, valuestring, curdelta, valuestring);
            } else {
                println!("\n{} did {} {} damage to {}!", actor.name, curdelta, valuestring, target.name);
            }
        } else if curdelta < 0 {
            if weakcalc == true {
                println!("\n{} was resistant to {}...\n({} {} Restored -> {} {} Restored)", target.name, acel, curdelta*2, valuestring, curdelta, valuestring);
            } else if strongcalc == true {
                println!("\n{} was vulnerable to {}!\n({} {} Restored -> {} {} Restored)", target.name, acel, curdelta/2, valuestring, curdelta, valuestring);
            } else {
                println!("\n{} healed {} points!", actor.name, curdelta);
            }
        } else {
            println!("\n{} did no damage to {}.", actor.name, target.name);
        }
    } else if selftarg == true {
        if curdelta < 0 {
            println!("\n{} dealt {} {} self-damage...", actor.name, curdelta, valuestring);
        } else if curdelta > 0 {
            println!("\n{} restored {} {}!", actor.name, curdelta, valuestring);
        }        
    }   
    if actor.curhealth > actor.health {
        actor.curhealth = actor.health;
    }
    if target.curhealth > target.health {
        target.curhealth = target.health;
    }
    general::pause();
}

//Generate all objects
fn main() {

    //Passive ability list {{{
    
    //Laugh (Charisma-based chance to disable player if health is higher than enemy)
    let laugh = |player: &mut entity::Entity, enemy: &mut entity::Entity, b1: bool, b2: bool, b3: bool, b4: bool| {
        if b1 == false && b2 == false && b3 == false && b4 == false {    
            if player.curhealth < enemy.curhealth {
                if general::rand_f() < ((player.charisma as f64) + (player.charismamod as f64))/((enemy.charisma as f64)*2.0 + (enemy.charismamod as f64)*2.0) as f64 {
                    println!("\n{}: \"{} is worse than {}! Keke!\"", enemy.name, player.name, enemy.name);
                    general::pause();
                    println!("\n{} laughed at you, lowering your resolve.", enemy.name);
                    player.defensemod -= player.defense/2;
                    general::pause();
                    println!("\n[DEF -{}]", player.defense/2);
                    general::pause();
                }
            }
        }
    };
    //}}}
    

    //Construct player {{{
    let mut player = (
    
    //Entity
    entity::player(),
    
    //Spells
    vec![
        spells::dropkick(), 
        spells::bodyslam(), 
        spells::zoom(),
    ],

    //Equipped
    [equip::empeq(), equip::empeq(), equip::empeq(), equip::empeq(), equip::empeq(), equip::empeq()]);
    //}}}
    
    //Construct inventory {{{
    let mut inventory = (
    
    //Items
    vec![
        items::smoothstone(4),
        items::medicinal_leaf(1),
    ],
    
    //Equipment 
    vec![
        //One-Handed
        equip::buckler(), 
        equip::flametongue(),
        //Two-Handed
        equip::zweihander(),
        //Armor 
        equip::chainmail(), 
        //Helm
        equip::pot(), 
        //Accessory
        equip::blessedring(), 
        equip::irongauntlet(),
    ]);
    
    //}}}
    
    //Construct enemies {{{
    let mut goblin = (
    
    //Entity
    entity::goblin(), 
    
    //Spells
    vec![
        spells::punch(), 
        spells::bash(),
    ], 
    
    //Unique Function
    laugh.clone()
    );

    let mut troll = (
        entity::troll(),
        vec![
            spells::dropkick(),
            spells::diamonddust(),
            spells::focusmind(),
        ],
        laugh.clone()
    );

    let mut cave_ent = vec![goblin.clone(), troll.clone()];


    //}}}

    //Demo
    title(&mut player, &mut inventory);   
    menu(&mut player.0, &player.1, &mut player.2, &mut inventory.0, &mut inventory.1, &mut goblin.0);
    engage(&mut player.0, &mut player.1, &mut player.2, &mut inventory.0, &mut inventory.1, &mut goblin.0, &mut goblin.1, goblin.2, general::enemyappear());        
    //encounter(&mut player.0, &mut player.1, &mut player.2, &mut inventory.0, &mut inventory.1, &mut cave_ent);        
    save(&player, &inventory);
}
