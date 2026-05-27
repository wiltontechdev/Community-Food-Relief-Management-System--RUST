use std::collections::HashMap;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum PriorityLevel {
    High,
    Low,
    Medium,
    Critical,
}
#[derive(Debug)]
pub struct Householdstock {
    pub houseid: String,
    pub stock: HashMap<String, u32>,
    pub served: Served,
}

#[derive(Debug)]
pub enum Served {
    Yes,
    No,
}

#[derive(Debug)]
pub struct HouseHold {
    pub id: String,
    pub name: String,
    pub no_members: u32,
    pub village: String,
    pub priority: PriorityLevel,
}

impl HouseHold {
    pub fn display(&self) {
        println!(
            "House {} details\n 
        Name: {}
        Head Count: {}
        Village: {}
        PriorityLevel: {:?}",
            self.id,
            self.name,
            self.no_members,
            self.village,
            self.priority
        );
    }

    // pub fn check_household(&self, hseid: &str) -> bool {
    //     self.id == hseid
    // }

    pub fn get_priority(&self) -> &PriorityLevel {
        &self.priority
    }
}
