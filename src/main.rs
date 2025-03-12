use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome traveler, may I know your name?");
    let mut trav_name = String::new();
    io::stdin()
        .read_line(&mut trav_name)
        .expect("Failed to read line");
    let trav_name = trav_name.trim();
    println!("What a pretty name you have, {}!", trav_name);
    println!("Now let's get you started with your journey.");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Pick a number from 1 to 100:");
    
    let mut gold = String::new();
    io::stdin()
        .read_line(&mut gold)
        .expect("Failed to read line");
    let gold: u32 = gold.trim().parse().expect("Please enter a valid number");
    println!("Nice, you got {} gold!", gold);
    
    match gold.cmp(&secret_number) {
        Ordering::Less => println!("That's too little for a traveler like you, hehe.."),
        Ordering::Greater => println!("You lucky seadog, you hit the bounty!"),
        Ordering::Equal => println!("That's the perfect amount you should get ;>")
    }
    
    println!("\nYou got your gold, nice! Let's continue.");
    println!("\nLet's get you a partner for your journey.");
    println!("Here's your horse, what are you naming him, matey?");
    
    let mut horse_name = String::new();
    io::stdin()
        .read_line(&mut horse_name)
        .expect("Failed to read line");
    let horse_name = horse_name.trim();
    println!("He is a good fella, isn't he?");
    
    if horse_name.eq_ignore_ascii_case("asher") {
        println!("Less goo! You have chosen a strong name.");
    }
    
    println!("\nNow, {} and {}, let's set off on an adventure!", trav_name, horse_name);
    println!("After traveling for a while, you encounter a bandit! Do you choose to (fight) or (run)?");
    
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice = choice.trim().to_lowercase();
    
    if choice == "fight" {
        let player_attack = rand::thread_rng().gen_range(1..=10);
        let bandit_attack = rand::thread_rng().gen_range(1..=10);
        
        println!("You attack the bandit with a power of {}!", player_attack);
        println!("The bandit strikes back with a power of {}!", bandit_attack);
        
        if player_attack > bandit_attack {
            println!("You have defeated the bandit! You loot 20 gold from him.");
        } else {
            println!("The bandit overpowers you, but you escape with your life!");
        }
    } else {
        println!("You swiftly turn {} around and gallop away, escaping safely!", horse_name);
    }
    
    println!("The journey continues...");
}
