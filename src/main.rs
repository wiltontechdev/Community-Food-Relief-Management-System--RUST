pub mod household;
pub mod inventory;
pub mod distribution;
pub mod report;
pub mod menus;

use distribution::DistributionRecord;
use household::{ HouseHold, PriorityLevel };
use inventory::Inventory;
use menus::{ distribution_menu, household_menu, invetory_menu, read_input };
use report::{ critical_household_count, households_hold_served, inventory_remaining, search };

use std::collections::HashMap;

fn main() {
    let mut id_count: u32 = 4;

    // =========================
    // Household Storage
    // =========================
    let mut houses: HashMap<String, HouseHold> = HashMap::new();

    // Sample Households
    let house1 = HouseHold {
        id: String::from("AB-1"),
        name: String::from("Otieno Family"),
        no_members: 6,
        village: String::from("Kibera"),
        priority: PriorityLevel::Critical,
    };

    let house2 = HouseHold {
        id: String::from("AB-2"),
        name: String::from("Mwangi Family"),
        no_members: 4,
        village: String::from("Mathare"),
        priority: PriorityLevel::High,
    };

    let house3 = HouseHold {
        id: String::from("AB-3"),
        name: String::from("Achieng Family"),
        no_members: 8,
        village: String::from("BabaDogo"),
        priority: PriorityLevel::Medium,
    };

    houses.insert(house1.id.clone(), house1);
    houses.insert(house2.id.clone(), house2);
    houses.insert(house3.id.clone(), house3);

    // =========================
    // Inventory Setup
    // =========================
    let mut inventory = Inventory::new();

    inventory.add_stock(String::from("Rice"), 100);
    inventory.add_stock(String::from("Beans"), 50);
    inventory.add_stock(String::from("CookingOil"), 30);
    inventory.add_stock(String::from("MaizeFlour"), 75);

    // =========================
    // Distribution Records
    // =========================
    let mut distribution_record = DistributionRecord {
        households_alloc: Vec::new(),
    };

    // =========================
    // Main Program Loop
    // =========================
    loop {
        println!("\n==================================");
        println!(" Community Food Relief System ");
        println!("==================================");
        println!("1. Household Management");
        println!("2. Inventory Management");
        println!("3. Distribution Management");
        println!("4. Reports");
        println!("5. Search");
        println!("6. Exit");

        let choice = read_input("Enter option: ");

        match choice.trim() {
            "1" => household_menu(&mut id_count, &mut houses),

            "2" => invetory_menu(&mut inventory),

            "3" => {
                distribution_menu(&mut distribution_record, &houses, &mut inventory);
            }

            "4" => {
                println!("\n========= REPORTS =========");

                println!("\n--- Inventory Remaining ---");
                inventory_remaining(&inventory);

                println!("\n--- Critical Household Count ---");
                critical_household_count(&houses);

                println!("\n--- Distribution Records ---");
                households_hold_served(&distribution_record);
            }

            "5" => search(&houses),

            "6" => {
                println!("Exiting program...");
                break;
            }

            _ => println!("Invalid option!"),
        }
    }
}
