use std::collections::HashMap;
use crate::household::*;
use super::inventory::Inventory;

pub struct DistributionRecord {
    households_alloc: Vec<Householdstock>,
}

impl DistributionRecord {
    pub fn get_records(&self) -> &Vec<Householdstock> {
        &self.households_alloc
    }
    pub fn distribute(
        &mut self,
        foodname: &str,
        qnt: u32,
        hseid: &str,
        inventory: &mut Inventory,
        houses: &HashMap<String, HouseHold>
    ) -> Result<(), String> {
        // Check if household exists
        if !self.check_household(hseid, houses) {
            return Err(format!("Household with ID {} not found", hseid));
        }

        // Prevent duplicate allocation
        if self.check_stockalloc(hseid) {
            return Err(
                format!("Household with ID {} has already been served in this cycle", hseid)
            );
        }

        // Remove stock only after validations pass
        inventory.remove_stock(foodname, qnt)?;

        // Record distribution
        let mut stock = HashMap::new();
        stock.insert(foodname.to_uppercase(), qnt);

        self.add_dist(hseid, stock);

        Ok(())
    }

    fn add_dist(&mut self, hseid: &str, stock: HashMap<String, u32>) {
        let house_stock = Householdstock {
            houseid: hseid.to_string(),
            stock,
            served: Served::Yes,
        };

        self.households_alloc.push(house_stock);
    }

    fn check_household(&self, hseid: &str, houses: &HashMap<String, HouseHold>) -> bool {
        match houses.get(hseid) {
            Some(household) => true,
            None => false,
        }
    }

    fn check_stockalloc(&self, hseid: &str) -> bool {
        for house in &self.households_alloc {
            if house.houseid == hseid {
                return true;
            }
        }

        false
    }
}
