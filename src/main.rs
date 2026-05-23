pub mod household;
pub mod inventory;
pub mod distribution;
pub mod reports;

use household::HouseHold;
use household::PriorityLevel::*;
use std::collections::HashMap;

fn gen_id(id_count: u32) -> String {
    let id = format!("AB-{}", id_count);
    id
}

fn main() {
    let mut id_count: u32 = 1;
    let mut houses: HashMap<u32, HouseHold> = HashMap::new();

    let hse1 = HouseHold {
        id: gen_id(id_count),
        name: String::from("House 1"),
        village: String::from("BabaDogo"),
        no_members: 6,
        priority: High,
    };

    houses.insert(id_count, hse1);

    id_count += 1;

    let hse2 = HouseHold {
        id: gen_id(id_count),
        name: String::from("House 2"),
        village: String::from("LuckySummer"),
        no_members: 3,
        priority: Critical,
    };

    id_count += 1;

    houses.insert(id_count, hse2);
}
