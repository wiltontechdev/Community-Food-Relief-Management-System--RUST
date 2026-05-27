use std::{ collections::HashMap };

pub struct Inventory {
    stock: HashMap<String, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            stock: HashMap::new(),
        }
    }

    pub fn add_stock(&mut self, foodname: String, qnt: u32) {
        self.stock.insert(foodname.to_uppercase(), qnt);
    }

    pub fn check_stock(&self) {
        for (foodtype, qnty) in &self.stock {
            println!("{}: {}", foodtype, qnty);
        }
    }

    pub fn get_food(&self, foodname: &str) -> Option<u32> {
        self.stock.get(&foodname.to_uppercase()).copied()
    }

    pub fn remove_stock(&mut self, foodname: &str, qnt: u32) -> Result<(), String> {
        let key = foodname.to_uppercase();

        match self.stock.get_mut(&key) {
            Some(current_qnt) => {
                if qnt > *current_qnt {
                    return Err(format!("Not enough stock for {}", foodname));
                }

                *current_qnt -= qnt;
                Ok(())
            }

            None => Err(format!("Item '{}' not found in stock", foodname)),
        }
    }

    //Learn How to handle errors then come to this

    // pub fn remove_stock(&mut self, foodname: &str, qnt: u32) -> Result<>
}
