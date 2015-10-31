////////////////////////////////////////////////////
//                  SETUP                         //
////////////////////////////////////////////////////
extern crate lib_andre;
extern crate rand;

use lib_andre::io::{prompt, print_file};
use std::error::Error;
use std::thread::sleep_ms;
use std::fs::{OpenOptions, remove_file};
use std::io::Write;
use std::fmt;

const CLEAR: &'static str = "\u{1B}[1;1H\u{1B}[2J";

struct Attr {
    str: u32,
    vit: u32,
    agi: u32,
    dex: u32,
    luk: u32,
}

struct Stats {
    attack: u32,
    health: u32,
    speed : u32,
    critch: u32,
    dropra: u32,

}

struct Creature {
    attributes: Attr,
    stats     : Stats,
    item      : Attr,
}

//This was fun! You can't just print a struct, because the compiler doesn't
//know what to do with it. This tells it how.
impl fmt::Display for Attr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "STR: {} VIT: {} AGI: {} DEX: {} LUK: {}",
                self.str, self.vit, self.agi, self.dex, self.luk)
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Health: {} Attack: {} Speed: {} Crit Chance: {}%",
                self.health, self.attack, self.speed, self.critch)
    }
}


////////////////////////////////////////////////////
//                   MAIN                         //
////////////////////////////////////////////////////


fn main() {
    try_main().unwrap();
}


fn try_main() -> Result<(), Box<Error>>{
    let mut player = Creature {
            attributes: Attr {  str: 0,
                                vit: 0,
                                agi: 0,
                                dex: 0,
                                luk: 0,
                             },
            stats:     Stats {  attack: 0,
                                health: 0,
                                speed : 0,
                                critch: 0,
                                dropra: 0,
                             },
            item:       Attr {  str: 0,
                                vit: 0,
                                agi: 0,
                                dex: 0,
                                luk: 0,
                             },
            };


    try!(intro(&mut player.attributes));


    for level in 1..11 {
        match remove_file("log/log.rpg") {
            Ok(_)  => {},
            Err(_) => {},
        }

        try!(calculate_stats(&player.attributes, &player.item, &mut player.stats));
        let mut monster = try!(spawn_monster(level));
        refresh_stats(&player.stats, &monster.stats, level);

        loop {
            let response: Vec<char> = try!(prompt("command: "))
                                            .chars()
                                            .collect();
            let response_key = if response.len() == 0 {
                                    'q'
                                }else{
                                    response[0].to_lowercase().next().unwrap()
                                };
    
            match response_key {
                'a' => { let player_damage = calc_damage(player.stats.attack, player.stats.critch);
                         basic_attack(player_damage, &mut monster.stats.health);
                         try!(log(format!("Dealt {} damage!", player_damage)));
                         if monster.stats.health == 0 { refresh_stats(&player.stats, &monster.stats, level);
                                                        try!(show_log());
                                                        println!("You win!"); break;
                                                      }
                       },
                'h' => { player.stats.health += { let heal = (player.stats.health as f64 * 0.2) as u32;
                                                  try!(log(format!("You healed for {}", heal)));
                                                  heal };
                       }, 
                 _  => { println!("Not a valid action.");
                         refresh_stats(&player.stats, &monster.stats, level);
                         continue; },
            };
    
            refresh_stats(&player.stats, &monster.stats, level);
            try!(show_log());
    
            let monster_damage = calc_damage(monster.stats.attack, monster.stats.critch);
            basic_attack(monster_damage, &mut player.stats.health);
            try!(log(format!("Took {} damage!", monster_damage)));
            refresh_stats(&player.stats, &monster.stats, level);
            try!(show_log());
            if player.stats.health == 0 { println!("You died! :("); return Ok(()); }
    
        }

        if (rand::random::<u32>() % 100) <= player.stats.dropra {
            player.item = Attr {  str: {if (rand::random::<u32>() % 100) <= 50 {
                                            (rand::random::<u32>() % (level + 1)) * 2
                                        } else { 0 }},
                                  vit: {if (rand::random::<u32>() % 100) <= 50 {
                                            (rand::random::<u32>() % (level + 1)) * 2
                                        } else { 0 }},
                                  agi: {if (rand::random::<u32>() % 100) <= 50 {
                                            (rand::random::<u32>() % (level + 1)) * 2
                                        } else { 0 }},
                                  dex: {if (rand::random::<u32>() % 100) <= 50 {
                                            (rand::random::<u32>() % (level + 1)) * 2
                                        } else { 0 }},
                                  luk: {if (rand::random::<u32>() % 100) <= 50 {
                                            (rand::random::<u32>() % (level + 1)) * 2
                                        } else { 0 }},
                               };
            println!("You found a stat boosting item! It provides:\n{}", player.item);
        }

        try!(prompt("Press enter to continue..."));
        println!("You've gained 5 attribute points! Go ahead and assign them.");
        try!(allocate(5, &mut player.attributes));

    }

    println!("You've beaten all 10 floors! Yay!");
    Ok(())

}

////////////////////////////////////////////////////
//                 PHASES                         //
////////////////////////////////////////////////////
fn intro(attributes: &mut Attr) -> Result<(), Box<Error>>{
    let mut attributes = attributes;
    let unallocated = 10;

    clear();
    println!("Welcome to the RPG! Your task is to get through 10 floors,");
    println!("each containing a monster.");
    println!("To get you started, you have 10 unallocated attribute points.");
    println!("Your current attributes are:\n\t{}\nLet's allocate them.", attributes);

    try!(allocate(unallocated, &mut attributes));

    println!("Your final attributes are:\n\t{}", attributes);
    println!("Let's start your journey!");
    try!(prompt("Are you ready?: "));
    println!("Good!");
    sleep_ms(1000);
    clear();

    Ok(())

}

////////////////////////////////////////////////////
//                 CONTROL                        //
////////////////////////////////////////////////////

fn clear() {
    println!("{}", CLEAR);
}

fn refresh_stats(player_stats: &Stats, monster_stats: &Stats, floor: u32) {
    clear();
    println!("Floor {}", floor);
    println!("==PLAYER==============================================\n|| {}", *player_stats);
    println!("======================================================");
    println!("==ENEMY===============================================\n|| {}", *monster_stats);
    println!("======================================================");
}

fn log(event: String) -> Result<(), Box<Error>>{
//    let mut file_path = try!(current_dir());
//    file_path.push("log");
//    file_path.push("log.rpg");
    let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("log/log.rpg")
            .unwrap();
 
    match write!(file, "{}\n", event) {
        Ok(..) => {},
        Err(e) => println!("{}", e),
    }

    Ok(())
}

fn show_log() -> Result<(), Box<Error>>{
//    let mut file_path = try!(current_dir());
//    file_path.push("log");
//    file_path.push("log.rpg");

    println!("{}", try!(print_file("log/log.rpg")));

    Ok(())
}

fn spawn_monster(level: u32) -> Result<Creature, Box<Error>> {

    let mut monster = Creature {
            attributes: Attr {  str: rand::random::<u32>() % (level.checked_mul(3).unwrap()),
                                vit: rand::random::<u32>() % (level.checked_mul(3).unwrap()) + level,
                                agi: rand::random::<u32>() % (level.checked_mul(3).unwrap()),
                                dex: rand::random::<u32>() % (level.checked_mul(3).unwrap()),
                                luk: 0,
                             },
            stats:     Stats {  attack: 0,
                                health: 0,
                                speed : 0,
                                critch: 0,
                                dropra: 0,
                             },
            item: Attr {  str: 0,
                                vit: 0,
                                agi: 0,
                                dex: 0,
                                luk: 0,
                             }
            };

    try!(calculate_stats(&monster.attributes, &monster.item, &mut monster.stats));

    Ok(monster)

}

////////////////////////////////////////////////////
//               STATS STUFF                      //
////////////////////////////////////////////////////
fn allocate(total: u32, attributes: &mut Attr) -> Result <u32, Box<Error>>{
    let mut total = total;

    while total > 0 {
        let response: Vec<char> = try!(prompt("What attribute? (str/vit/agi/dex/luk): "))
                                        .chars()
                                        .collect();
        let response_key = if response.len() == 0 {
                                'q'
                            }else{
                                response[0].to_lowercase().next().unwrap()
                            };

        total = match response_key {
            's' => add_attr(&mut attributes.str, total, "Strength: "),
            'v' => add_attr(&mut attributes.vit, total, "Vitality: "),
            'a' => add_attr(&mut attributes.agi, total, "Agility: "),
            'd' => add_attr(&mut attributes.dex, total, "Dexterity: "),
            'l' => add_attr(&mut attributes.luk, total, "Luck: "),
             _  => total,
        };

        println!("\t{}", attributes);
        println!("You have {} attribute points left to allocate.", total);
    }
    Ok(total)
}

fn add_attr(attr: &mut u32, points_avail: u32, attribute: &str) -> u32{
    let response = match prompt(attribute).unwrap().parse::<i32>() {
                              Ok(n) => n,
                              Err(_) => -1,
                          };

    if response < 0 {
        println!("Not a valid number.");
        points_avail
    } else if response as u32 > points_avail {
        println!("You don't have that many attribute points.");
        points_avail
    } else {
        *attr += response as u32;
        points_avail - response as u32 
    }
}

fn calculate_stats(attributes: &Attr, item: &Attr, stats: &mut Stats) -> Result<(), Box<Error>>{
    //should be checking for overflow. will do later.
    //dont forget about u32.checked_mul
    stats.attack = ((attributes.str + item.str)as f64).mul_add(1.5, 5.0) as u32;
    stats.health = ((attributes.vit + item.vit)as f64).mul_add(2.7, 20.0) as u32;
    stats.speed  = ((attributes.agi + item.agi)as f64).mul_add(1.1, 5.0) as u32;
    stats.critch = ((attributes.dex + item.dex)as f64).mul_add(1.6, 0.5) as u32;
    stats.dropra = ((attributes.luk + item.luk)as f64).mul_add(1.5, 0.5) as u32;

    Ok(())
}

////////////////////////////////////////////////////
//                BATTLE STUFF                    //
////////////////////////////////////////////////////

fn calc_damage(agent_attack: u32, agent_critch: u32) -> u32 {
 ((agent_attack as f64 * 0.9) as u32 + ((rand::random::<u32>() % ((agent_attack as f64 * 0.2) as u32)))
                    + { if (rand::random::<u32>() % 100) <= agent_critch { println!("Crit!"); ((agent_attack as f64 * 0.9) as u32 + ((rand::random::<u32>() % ((agent_attack as f64 * 0.2) as u32))))
                                                                      }else{0}
                      }
                  )
}

fn basic_attack(agent_damage: u32, patient_health: &mut u32) {
    if agent_damage > *patient_health {
        *patient_health = 0;
    } else {
        *patient_health -= agent_damage;
    }
}
