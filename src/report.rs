use super::household::*;
use super::inventory::*;
use super::distribution::*;
use super::menus::read_input;
use std::collections::HashMap;
use std::io;
use std::io::Write;

pub fn households_hold_served(distribution: &DistributionRecord) {
    for house in distribution.get_records() {
        println!("House Id: {}
        House", house.houseid);

        for (foodname, qnt) in &house.stock {
            println!("Foodname: {} Quantity: {}", foodname, qnt);
        }
        println!("\n--------------------------------------------------");
    }
}

pub fn critical_household_count(households: &HashMap<String, HouseHold>) {
    let mut critical_households: Vec<String> = Vec::new();
    for (hseid, household) in households {
        if let PriorityLevel::Critical = household.get_priority() {
            critical_households.push(hseid.clone());
        }
    }

    println!("Critical Household Count: {}", critical_households.len());
    println!("Here is the list of House IDs with Critical Count");
    for id in critical_households {
        println!("House {}", id);
    }
}

pub fn inventory_remaining(inventory: &Inventory) {
    inventory.check_stock();
}

pub fn search(households: &HashMap<String, HouseHold>) {
    loop {
        // print menu
        println!("\n=== Search ===");
        println!("1. Search By ID");
        println!("2. Search By Village");
        println!("3. Search By Priority");
        println!("4. Exit Menu");
        print!("Enter option: ");
        io::stdout().flush().unwrap(); // ensures prompt prints before input

        // read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => search_by_id(households),
            "2" => search_by_village(households),
            "3" => search_by_priority(households),
            "4" => {
                println!("Menu exited!");
                break;
            }
            _ => println!("Invalid option, try again."),
        }
    }
}

fn search_by_id(households: &HashMap<String, HouseHold>) {
    let id = read_input("Enter the ID of the House: ");
    match households.get(&id) {
        Some(household) => household.display(),
        None => {
            println!("House does not exist!");
        }
    }
}
fn search_by_village(households: &HashMap<String, HouseHold>) {
    let village = read_input("Enter the village name of the House: ");
    let mut count = 0;

    for (_, household) in households {
        if household.village == village {
            household.display();
            count += 1;
        }
    }

    if count == 0 {
        println!("House with Village does not exist!");
    }
}

fn search_by_priority(households: &HashMap<String, HouseHold>) {
    loop {
        // print menu
        println!("\n=== Priority Level ===");
        println!("1. Critical");
        println!("2. High");
        println!("3. Medium");
        println!("4. Low");
        println!("5. Exit Menu");
        print!("Enter option: ");
        io::stdout().flush().unwrap(); // ensures prompt prints before input

        // read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => print_household_bypriority(PriorityLevel::Critical, households),
            "2" => print_household_bypriority(PriorityLevel::High, households),
            "3" => print_household_bypriority(PriorityLevel::Medium, households),
            "4" => print_household_bypriority(PriorityLevel::Low, households),
            "5" => {
                println!("Menu exited!");
                break;
            }
            _ => println!("Invalid option, try again."),
        }
    }
}

fn print_household_bypriority(priority: PriorityLevel, households: &HashMap<String, HouseHold>) {
    for (_, household) in households {
        if *household.get_priority() == priority {
            household.display();
            println!("\n------------------------------------------");
        }
    }
}
