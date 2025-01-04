pub mod inventory;
pub mod transaction;
mod util;

use inventory::Inventory;
use inventory::{add_product, delete_product, edit_product, list_products};

pub struct Store {
    pub inventory: Inventory,
}

pub struct StoreConfig<'a> {
    pub actions: Vec<(&'a str, &'a str, fn(&mut Inventory))>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            inventory: Inventory::new(),
        }
    }
}

impl<'a> StoreConfig<'a> {
    pub fn new() -> Self {
        StoreConfig {
            actions: vec![
                ("1", "Add Product", add_product),
                ("2", "Edit Product", edit_product),
                ("3", "Delete Product", delete_product),
                ("4", "List Products", list_products),
            ],
        }
    }

    pub fn execute(&self, choice: &str, inventory: &mut Inventory) -> bool {
        if let Some((_, _, action)) = self.actions.iter().find(|(key, _, _)| key == &choice) {
            action(inventory);
            true
        } else {
            false
        }
    }

    pub fn display_menu(&self) {
        println!("\n--- Rusty Store Management ---");
        for (key, description, _) in &self.actions {
            println!("{}: {}", key, description);
        }
        println!("6: Exit");
    }
}
