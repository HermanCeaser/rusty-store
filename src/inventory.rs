use std::collections::HashMap;

use crate::display;

#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
}

pub trait InventoryManagement {
    fn add_product(&mut self, product: Product) -> Result<(), String>;
    fn edit_product(
        &mut self,
        product_name: &str,
        description: Option<String>,
        price: Option<f64>,
        quantity: Option<u32>,
    ) -> Result<(), String>;
    fn delete_product(&mut self, product_name: &str) -> Result<(), String>;
}

pub struct Inventory {
    pub products: HashMap<String, Product>,
}

impl InventoryManagement for Inventory {
    fn add_product(&mut self, product: Product) -> Result<(), String> {
        if product.price < 0.0 {
            return Err("Price cannot be negative!".to_string());
        }

        self.products.insert(product.name.clone(), product);
        Ok(())
    }

    fn edit_product(
        &mut self,
        product_name: &str,
        description: Option<String>,
        price: Option<f64>,
        quantity: Option<u32>,
    ) -> Result<(), String> {
        if let Some(product) = self.products.get_mut(product_name) {
            let mut updated = false;

            if let Some(new_description) = description {
                product.description = new_description;
                updated = true;
            }

            if let Some(new_price) = price {
                if new_price < 0.0 {
                    return Err("Price cannot be negative!".to_string());
                }
                product.price = new_price;
                updated = true;
            }

            if let Some(new_quantity) = quantity {
                product.quantity = new_quantity;
                updated = true;
            }

            if !updated {
                return Err("At least one field must be provided for update.".to_string());
            }

            Ok(())
        } else {
            Err(format!("Product '{}' not found.", product_name))
        }
    }

    fn delete_product(&mut self, product_name: &str) -> Result<(), String> {
        if self.products.remove(product_name).is_none() {
            return Err(format!("Product '{}' not found.", product_name));
        } 
        Ok(())
      
    }
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            products: HashMap::new(),
        }
    }
}

/// Public function to add a product to the inventory
pub fn add_product(inventory: &mut Inventory, product: Product) {
    match inventory.add_product(product) {
        Ok(_) => println!("Product added successfully!"),
        Err(err) => println!("Error adding product: {}", err),
    }
}

/// Public function to edit a product in the inventory
pub fn edit_product(
    inventory: &mut Inventory,
    product_name: &str,
    description: Option<String>,
    price: Option<f64>,
    quantity: Option<u32>,
) {
    match inventory.edit_product(product_name, description, price, quantity) {
        Ok(_) => println!("Product edited successfully!"),
        Err(err) => println!("Error editing product: {}", err),
    }
}

/// Public function to delete atablet from the inventory
pub fn delete_product(inventory: &mut Inventory, product_name: &str) {
    match inventory.delete_product(product_name) {
        Ok(_) => println!("Product deleted successfully!"),
        Err(err) => println!("Error deleting product: {}", err),
    }
}

/// Public function to list products in the inventory
pub fn list_products(inventory: &Inventory) {

    if inventory.products.is_empty() {
        println!("No products available in the inventory.");
        return;       
    }

    let headers = vec!["No", "Name", "Description", "Price", "Quantity"];
    let rows: Vec<Vec<String>> = inventory
        .products
        .values()
        .enumerate()
        .map(|(index, product)| {
            vec![
                (index + 1).to_string(), 
                product.name.clone(),
                product.description.clone(),
                format!("{:.2}", product.price),
                product.quantity.to_string(),
            ]
        })
        .collect();

    let table = display::format_table(headers, rows);
    println!("{}", table);
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_valid_product() {
        let mut inventory = Inventory::new();

        let result = inventory.add_product(Product {
            name: "Laptop".to_string(),
            description: "A gaming laptop".to_string(),
            price: 1200.0,
            quantity: 20,
        });

        assert!(result.is_ok());
        assert!(inventory.products.contains_key("Laptop"));
    }

    #[test]

    fn prevents_adding_product_with_negative_price() {
        let mut inventory = Inventory::new();

        let result = inventory.add_product(Product {
            name: "Smartphone".to_string(),
            description: "A high-end smartphone".to_string(),
            price: -999.0, // Invalid Price
            quantity: 20,
        });

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Price cannot be negative!".to_string());
        assert!(!inventory.products.contains_key("Smartphone"));
    }

    #[test]
    fn updates_product_with_single_field() {
        let mut inventory = Inventory::new();

        let _ = inventory.add_product(Product {
            name: "Tablet".to_string(),
            description: "A basic tablet".to_string(),
            price: 300.0,
            quantity: 50,
        });

        let result = inventory.edit_product("Tablet", None, Some(280.0), None);

        assert!(result.is_ok());
        let product = inventory.products.get("Tablet").unwrap();
        assert_eq!(product.price, 280.0);
    }

    #[test]
    fn updates_product_with_multiple_fields() {
        let mut inventory: Inventory = Inventory::new();

        let _ = inventory.add_product(Product {
            name: "Test Product".to_string(),
            description: "A product for testing".to_string(),
            price: 100.0,
            quantity: 20,
        });

        let result = inventory.edit_product(
            "Test Product",
            Some("Updated description".to_string()),
            Some(150.0),
            Some(25),
        );

        let product = inventory.products.get("Test Product").unwrap();

        assert!(result.is_ok());
        assert_eq!(product.description, "Updated description");
        assert_eq!(product.price, 150.0);
        assert_eq!(product.quantity, 25);
    }

    #[test]
    fn update_product_requires_at_least_one_field() {
        let mut inventory = Inventory::new();

        let _ = inventory.add_product(Product {
            name: "Laptop".to_string(),
            description: "A gaming laptop".to_string(),
            price: 1200.0,
            quantity: 10,
        });

        let result = inventory.edit_product("Laptop", None, None, None);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "At least one field must be provided for update.".to_string()
        );
    }

    #[test]
    fn handles_missing_product() {
        let mut inventory = Inventory::new();

        let result = inventory.edit_product("Nonexistent", None, Some(500.0), None);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Product 'Nonexistent' not found.".to_string()
        );
    }

    #[test]
    fn deletes_product() {
        let mut inventory = Inventory::new();
        let _ = inventory.add_product(Product {
            name: "Laptop".to_string(),
            description: "A gaming laptop".to_string(),
            price: 1200.0,
            quantity: 10,
        });

        let product = inventory.products.get("Laptop").unwrap();
        // Assert that the added product exists
        assert_eq!(product.name, "Laptop");
        assert!(inventory.products.contains_key("Laptop"));

        let result = inventory.delete_product("Laptop");
        // Assert that deleted product nolonger exists
        assert!(result.is_ok());
        assert!(!inventory.products.contains_key("Laptop"));
    }

    #[test]
    fn deletes_non_existent_product() {
        let mut inventory = Inventory::new();
        
        let result = inventory.delete_product("Laptop");
        // Assert that deletion of nonexistent product is not successfull
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Product 'Laptop' not found.".to_string()
        );
    }
}
