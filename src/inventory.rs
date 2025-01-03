use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
}

pub trait InventoryManagement {
    fn add_product(&mut self, product:Product);
    fn edit_product(&mut self, product_name: &str, new_product: Product);
    fn delete_product(&mut self, product_name: &str);
    
}

pub struct Inventory {
    pub products: HashMap<String, Product>
}

impl InventoryManagement for Inventory {
    fn add_product(&mut self, product:Product) {
        self.products.insert(product.name.clone(), product);
    }

    fn edit_product(&mut self, product_name: &str, new_product: Product) {
        if self.products.contains_key(product_name) {
            self.products.insert(product_name.to_string(), new_product);
        } else {
            //Handle Product not found later
            println!("Error: Product not found")
        }
    }

    fn delete_product(&mut self, product_name: &str) {
        if self.products.contains_key(product_name) {
            self.products.remove(product_name);
        } else {
            // Handle product not found error
            println!("Error: Product not found.");
        }
    }
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            products: HashMap::new(),
        }
    }
}

