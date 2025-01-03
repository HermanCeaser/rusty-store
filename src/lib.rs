mod display;
pub mod inventory;
use inventory::Inventory;


pub struct Store {
    pub inventory: Inventory
}


impl Store {
    pub fn new() -> Self {
        Store {
            inventory: Inventory::new(),
        }
    }
}