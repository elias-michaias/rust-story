# rust-story
A simple, text-based, role-playing game written in Rust.

(In development.)

## Featuring:
- Witty writing :memo:
- Complex, turn-based, 1-on-1 combat :collision:
- Familiar style, unique mechanics :gear:
- Flexible growth :bar_chart:
- Easy savedata management (via `.json`) :floppy_disk: 
- And more! :jigsaw:

### Progress: 
- [x] (Most all battle mechanics!) :crossed_swords:
- [x] `Stance Break` mechanic :dizzy:
 - *When `SP` is depleted, Max `HP`, `MP`, and `SP` are lowered and a turn is lost* 
- [x] Elemental type system :fire:
 - *`Fire` vs. `Water`, `Air` vs. `Earth`, `Light` vs. `Dark` - Opposites yield double damage, aligned deal half*  
- [x] Status conditions :zzz:
 - *`Poison`, `Paralysis`, `Sleep`, `Regeneration` - two status ailments can not be applied simultaneously* 
- [x] Status modifiers :game_die:
 - *Statuses include: `HP`, `MP`, `SP`, `STR`, `DEF`, `INT`, `RES`, `FAI`, `CHA`, `SKL`, `LCK`*
- [x] Functioning items, equipment, moves, enemies :bricks:
 - *All algorithms and functions work within battle based off of set values*
- [ ] Fully populate [item table](https://github.com/emskeirik/rust-story/blob/master/src/items.rs "items.rs") :chocolate_bar:
- [ ] Fully populate [equipment table](https://github.com/emskeirik/rust-story/blob/master/src/equip.rs "equip.rs") :school_satchel:
- [ ] Fully populate [move table](https://github.com/emskeirik/rust-story/blob/master/src/spells.rs "spells.rs") :mage:
- [ ] Fully populate [enemy table](https://github.com/emskeirik/rust-story/blob/master/src/entity.rs "entity.rs") :japanese_goblin:
- [ ] Finish fleshing out enemy conversation :speech_balloon:
- [ ] Develop story :movie_camera:
 