pub mod household;
pub mod inventory;
pub mod distribution;
pub mod menus;

use household::HouseHold;
use household::PriorityLevel::*;
use std::collections::HashMap;
use std::io::{ self, Write };

use crate::household::PriorityLevel;

fn gen_id(id_count: &mut u32) -> String {
    let id = format!("AB-{}", id_count);
    id
}

fn main() {
    let mut id_count: u32 = 1;
    let mut houses: HashMap<String, HouseHold> = HashMap::new();

    id_count += 1;

    id_count += 1;
}
