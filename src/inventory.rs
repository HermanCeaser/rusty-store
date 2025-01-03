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
    fn edit_product(&mut self, product_name: &str, description: String, price: f64, quantity: u32);
    fn delete_product(&mut self, product_name: &str);
    
}

pub struct Inventory {
    pub products: HashMap<String, Product>
}

impl InventoryManagement for Inventory {
    fn add_product(&mut self, product:Product) {
        self.products.insert(product.name.clone(), product);
    }

    fn edit_product(&mut self, product_name: &str, description: String, price: f64, quantity: u32) {
        if let Some(product) = self.products.get_mut(product_name) {
            product.description = description;
            product.price = price;
            product.quantity = quantity;
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_product() {
        let mut inventory = Inventory::new();

        inventory.add_product(Product {
            name: "Test Product".to_string(),
            description: "A product for testing".to_string(),
            price: 100.0,
            quantity: 20,
        });

        assert!(inventory.products.contains_key("Test Product"));
    }

    #[test]
    fn edits_product() {
        let mut inventory: Inventory = Inventory::new();
        inventory.add_product(Product {
            name: "Test Product".to_string(),
            description: "A product for testing".to_string(),
            price: 100.0,
            quantity: 20,
        });
        inventory.edit_product("Test Product", 
            "Updated description".to_string(),
            150.0,
            15,
        );

        let product = inventory.products.get("Test Product").unwrap();

        assert_eq!(product.description, "Updated description");
        assert_eq!(product.price, 150.0);
        assert_eq!(product.quantity, 15);

    }
}