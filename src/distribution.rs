use std::collections::HashMap;
use crate::household::Householdstock;

// distribution.rs
use super::household::HouseHold;
use super::inventory::Inventory;

pub struct Distributionrecord {
    households_alloc: Vec<Householdstock>,
}

impl Distributionrecord {
    pub fn distribute(
        &mut self,
        foodname: &str,
        qnt: u32,
        hseid: &str,
        inventory: &mut Inventory,
        houses: &Vec<HouseHold>
    ) -> Result<(), String> {
        if self.check_household(hseid, houses) {
            match inventory.remove_stock(foodname, qnt) {
                Ok(()) => {
                    if !self.check_stockalloc(hseid) {
                        let mut stock = HashMap::new();
                        stock.insert(foodname.to_uppercase(), qnt);
                        self.add_dist(hseid, stock);

                        Ok(())
                    } else {
                        return Err(
                            format!("Household with ID {} has already been served in this cycle", hseid)
                        );
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        } else {
            return Err(format!("Household with ID {} not found", hseid));
        }
    }

    fn add_dist(&mut self, hseid: &str, stock: HashMap<String, u32>) {
        let house_stock = Householdstock {
            houseid: hseid.to_string(),
            stock: stock,
        };
        self.households_alloc.push(house_stock);
    }

    fn check_household(&self, hseid: &str, households: &[HouseHold]) -> bool {
        for house in households {
            if house.id == hseid {
                return true;
            }
        }

        false
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
