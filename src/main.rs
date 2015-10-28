////////////////////////////////////////////////////
//                  SETUP                         //
////////////////////////////////////////////////////
extern crate lib_andre;
extern crate rand;

use std::error::Error;
use std::thread::sleep_ms;
use lib_andre::io::prompt;
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

struct Monster {
    attributes: Attr,
    stats     : Stats,
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
        write!(f, "Health: {} Attack: {} Speed: {} Crit Chance: {}",
                self.health, self.attack, self.speed, self.critch)
    }
}

fn main() {
    try_main().unwrap();
}


fn try_main() -> Result<(), Box<Error>>{
    let mut player_attributes = Attr {
                    str: 0,
                    vit: 0,
                    agi: 0,
                    dex: 0,
                    luk: 0,
                };
    let mut player_stats =     Stats {
                    attack: 0,
                    health: 0,
                    speed : 0,
                    critch: 0,
                    dropra: 0,
                };

    try!(intro(&mut player_attributes));

    try!(calculate_stats(&player_attributes, &mut player_stats));

    let mut monster = try!(spawn_monster(1));
    println!("==============================PLAYER==================\n|| {}", player_stats);
    println!("======================================================");
    println!("==============================ENEMY===================\n|| {}", monster.stats);
    println!("======================================================");

    monster.stats.health = attack(player_stats.attack, monster.stats.health);

    println!("==============================ENEMY===================\n|| {}", monster.stats);
    println!("======================================================");

    Ok(())

}

////////////////////////////////////////////////////
//                 PHASES                         //
////////////////////////////////////////////////////
fn intro(attributes: &mut Attr) -> Result<(), Box<Error>>{
    let mut attributes = attributes;
    let unallocated = 10;

    println!("{}", CLEAR);
    println!("Welcome to the RPG! You have 10 unallocated attribute points.");
    println!("Your current attributes are:\n\t{}\nLet's allocate them.", attributes);

    try!(allocate(unallocated, &mut attributes));

    println!("Your final attributes are:\n\t{}", attributes);
    println!("Let's start your journey!");
    try!(prompt("Are you ready?: "));
    println!("Good!");
    sleep_ms(1000);
    println!("{}", CLEAR);
    

    Ok(())

}

fn spawn_monster(level: u32) -> Result<Monster, Box<Error>> {

    let mut monster = Monster {
            attributes: Attr {  str: rand::random::<u32>() % (level.checked_mul(5).unwrap()),
                                vit: rand::random::<u32>() % (level.checked_mul(5).unwrap()),
                                agi: rand::random::<u32>() % (level.checked_mul(5).unwrap()),
                                dex: rand::random::<u32>() % (level.checked_mul(5).unwrap()),
                                luk: rand::random::<u32>() % (level.checked_mul(5).unwrap()),
                             },
            stats:     Stats {  attack: 0,
                                health: 0,
                                speed : 0,
                                critch: 0,
                                dropra: 0,
                             },
            };

    calculate_stats(&monster.attributes, &mut monster.stats);

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
        *attr = response as u32;
        points_avail - response as u32 
    }
}

fn calculate_stats(attributes: &Attr, stats: &mut Stats) -> Result<(), Box<Error>>{
    //should be checking for overflow. will do later.
    //dont forget about u32.checked_mul
    stats.attack = (attributes.str as f64).mul_add(1.5, 5.0) as u32;
    stats.health = (attributes.vit as f64).mul_add(2.7, 20.0) as u32;
    stats.speed  = (attributes.agi as f64).mul_add(1.1, 5.0) as u32;
    stats.critch = (attributes.dex as f64).mul_add(1.1, 0.5) as u32;
    stats.dropra = (attributes.luk as f64).mul_add(1.5, 0.5) as u32;

    Ok(())
}
////////////////////////////////////////////////////
//              END STATS STUFF                   //
////////////////////////////////////////////////////

////////////////////////////////////////////////////
//                BATTLE STUFF                    //
////////////////////////////////////////////////////

fn attack(agent_attack: u32, patient_health: u32) -> u32 {
 patient_health - ((agent_attack as f64 * 0.9) as u32 + ((rand::random::<u32>() % ((agent_attack as f64 * 0.2) as u32))))
}
