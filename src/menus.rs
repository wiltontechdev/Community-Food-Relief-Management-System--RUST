use super::inventory::*;
use super::household::*;
use super::distribution::*;
use std::collections::HashMap;
use std::io::{ self, Write };

// pub fn total_households_served(households: &[HouseHold]) -> u32 {
//     let mut count: u32 = households.len();
// }

fn gen_id(id_count: &mut u32) -> String {
    let id = format!("AB-{}", id_count);
    id
}

pub fn distribution_menu(
    distribution_record: &mut DistributionRecord,
    houses: &HashMap<String, HouseHold>,
    inventory: &mut Inventory
) {
    loop {
        // print menu
        println!("\n=== Distribution Manager ===");
        println!("1. Distribute");
        println!("2. Check Available Houses");
        println!("3. View Previous Distributions");
        println!("3. Exit distribution Menu");
        print!("Enter option: ");
        io::stdout().flush().unwrap(); // ensures prompt prints before input

        // read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => distribute(distribution_record, houses, inventory),
            "2" => check_avilable_houses(houses),
            "3" => {
                println!("Exiting Distribution Menu!");
                break;
            }
            _ => println!("Invalid option, try again."),
        }
    }
}

fn check_avilable_houses(households: &HashMap<String, HouseHold>) {
    for (id, house) in households {
        println!("\n ID: {} Name: {} PriorityLevel: {:?}", id, house.name, house.get_priority());
    }
}

fn distribute(distribution_record: &mut DistributionRecord) {
    let foodname = read_input("Enter the name of the stock: ");
    let hseid = read_input("Enter the house ID: ");

    let qnt: u32 = loop {
        let input = read_input("Enter stock count: ");
        match input.parse::<u32>() {
            Ok(n) => {
                break n;
            }
            Err(_) => println!("Invalid, please enter a number"),
        }
    };

    distribution_record.distribute(&foodname, qnt, &hseid, inventory, houses)
}

//Inventory Menu
pub fn invetory_menu(inventory: &mut Inventory) {
    loop {
        // print menu
        println!("\n=== Inventory Manager ===");
        println!("1. Add Stock");
        println!("2. Remove Stock");
        println!("3. List Available Stock");
        println!("4. Check Stock");
        println!("5. Exit Inventory menu");
        print!("Enter option: ");
        io::stdout().flush().unwrap(); // ensures prompt prints before input

        // read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => add_stock(inventory),
            "2" => remove_stock(inventory),
            "3" => list_inventory(inventory),
            "4" => get_stock_details(inventory),
            "6" => {
                println!("Menu exited!");
                break;
            }
            _ => println!("Invalid option, try again."),
        }
    }
}

fn add_stock(inventory: &mut Inventory) {
    let foodname = read_input("Enter the name of the stock: ");
    let qnt: u32 = loop {
        let input = read_input("Enter stock count: ");
        match input.parse::<u32>() {
            Ok(n) => {
                break n;
            }
            Err(_) => println!("Invalid, please enter a number"),
        }
    };

    inventory.add_stock(foodname, qnt);
}

fn remove_stock(inventory: &mut Inventory) {
    let foodname = read_input("Enter the name of the stock: ");
    let qnt: u32 = loop {
        let input = read_input("Enter stock count to remove: ");
        match input.parse::<u32>() {
            Ok(n) => {
                break n;
            }
            Err(_) => println!("Invalid, please enter a number"),
        }
    };

    match inventory.remove_stock(&foodname, qnt) {
        Ok(()) => println!("\nstock removed successfully"),
        Err(err) => println!("{}", err),
    }
}

fn get_stock_details(inventory: &Inventory) {
    let foodname = read_input("Enter the name of the stock: ");
    inventory.get_food(&foodname);
}

fn list_inventory(inventory: &Inventory) {
    inventory.check_stock();
}

// Household menu
pub fn household_menu(id_count: &mut u32, houses: &mut HashMap<String, HouseHold>) {
    loop {
        // print menu
        println!("\n=== Household Manager ===");
        println!("1. Add Household");
        println!("2. Remove Household");
        println!("3. List Households");
        println!("4. Check Household Details");
        println!("5. Exit Menu");
        print!("Enter option: ");
        io::stdout().flush().unwrap(); // ensures prompt prints before input

        // read input
        let mut input = String::new();
        let id = gen_id(id_count);
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                add_household(id, houses);
                *id_count += 1;
            }
            "2" => remove_household(&id, houses),
            "3" => list_households(houses),
            "4" => check_household_details(&id, houses),
            "6" => {
                println!("Menu exited!");
                break;
            }
            _ => println!("Invalid option, try again."),
        }
    }
}

fn check_household_details(id: &str, houses: &HashMap<String, HouseHold>) {
    match houses.get(id) {
        Some(house) => {
            house.display();
        }
        None => {
            println!("Household does not exist!");
        }
    }
}

fn list_households(households: &HashMap<String, HouseHold>) {
    for (id, household) in households {
        println!(
            "House ID: {}
            House Name: {}
            House head Count: {} \n",
            id,
            household.name,
            household.no_members
        );
    }
}

fn remove_household(id: &str, households: &mut HashMap<String, HouseHold>) {
    households.remove(id);
}

fn add_household(id: String, households: &mut HashMap<String, HouseHold>) {
    let name = read_input("Enter the house name: ");
    let village = read_input("Enter the village name; ");
    let no_members: u32 = loop {
        let input = read_input("Enter a Headcount of the family: ");
        match input.parse::<u32>() {
            Ok(n) => {
                break n;
            }
            Err(_) => println!("Invalid, please enter a number"),
        }
    };

    let priority = loop {
        match
            read_input("What is the condition of the household (Low/Medium/High/Critical): ")
                .to_uppercase()
                .as_str()
        {
            "LOW" => {
                break PriorityLevel::Low;
            }
            "MEDIUM" => {
                break PriorityLevel::Medium;
            }
            "HIGH" => {
                break PriorityLevel::High;
            }
            "CRITICAL" => {
                break PriorityLevel::Critical;
            }
            _ => println!("Invalid, enter Low, Medium or High"),
        }
    };

    let key = id.clone();

    let household = HouseHold {
        id,
        name,
        village,
        no_members,
        priority,
    };

    households.insert(key, household);
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
