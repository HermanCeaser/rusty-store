use std::fmt;

use crate::inventory::{Inventory, Product};

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Sale,
    Purchase,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub product_name: String,
    pub quantity: u32,
    pub price: f64, //Sale Price or Cost Price
    pub total: f64, // Total Cost or Revenue
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<10} {:<20} {:<10} {:<10.2} {:<10.2}",
            match self.transaction_type {
                TransactionType::Sale => "Sale",
                TransactionType::Purchase => "Purchase",
            },
            self.product_name,
            self.quantity,
            self.price,
            self.total
        )
    }
}

pub struct TransactionManager {
    pub transactions: Vec<Transaction>,
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    pub fn record_sale(
        &mut self,
        inventory: &mut Inventory,
        product_name: &str,
        quantity: u32,
        sale_price: f64,
    ) -> Result<(), String> {
        let product = inventory.products.get_mut(product_name);
        if let Some(product) = product {
            if product.quantity < quantity {
                return Err("Not enough stock to complete the sale.".to_string());
            }

            product.quantity -= quantity;
            let total_revenue = sale_price * quantity as f64;

            self.transactions.push(Transaction {
                transaction_type: TransactionType::Sale,
                product_name: product_name.to_string(),
                quantity,
                price: sale_price,
                total: total_revenue,
            });

            Ok(())
        } else {
            Err("Product not found.".to_string())
        }
    }

    pub fn record_purchase(
        &mut self,
        inventory: &mut Inventory,
        product_name: &str,
        quantity: u32,
        purchase_price: f64,
    ) -> Result<(), String> {
        let product = inventory.products.get_mut(product_name);

        if let Some(product) = product {
            product.quantity += quantity;
        } else {
            inventory.products.insert(
                product_name.to_string(),
                Product {
                    name: product_name.to_string(),
                    description: "Purchased Product".to_string(),
                    price: purchase_price,
                    quantity,
                },
            );
        }

        let total_cost = purchase_price * quantity as f64;

        self.transactions.push(Transaction {
            transaction_type: TransactionType::Purchase,
            product_name: product_name.to_string(),
            quantity,
            price: purchase_price,
            total: total_cost,
        });

        Ok(())
    }

    pub fn list_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_a_valid_sale() {
        let mut inventory = Inventory::new();
        let mut transaction_manager = TransactionManager::new();

        inventory.products.insert(
            "Widget".to_string(),
            Product {
                name: "Widget".to_string(),
                description: "A test widget".to_string(),
                price: 50.0,
                quantity: 100,
            },
        );

        // Record a sale
        let result = transaction_manager.record_sale(&mut inventory, "Widget", 10, 55.0);
        assert!(result.is_ok());
        assert_eq!(inventory.products.get("Widget").unwrap().quantity, 90); 
        assert_eq!(transaction_manager.transactions.len(), 1);

        let transaction = &transaction_manager.transactions[0];
        assert_eq!(transaction.transaction_type, TransactionType::Sale);
        assert_eq!(transaction.product_name, "Widget");
        assert_eq!(transaction.quantity, 10);
        assert_eq!(transaction.price, 55.0);
        assert_eq!(transaction.total, 550.0); // 10 * 55
    }

    #[test]
    fn record_sale_of_product_with_insufficient_stock() {
        let mut inventory = Inventory::new();
        let mut transaction_manager = TransactionManager::new();

        // Add a product to the inventory
        inventory
            .products
            .insert("Widget".to_string(), Product {
                name: "Widget".to_string(),
                description: "A test widget".to_string(),
                price: 50.0,
                quantity: 5,
            });

        // Attempt to sell more than available stock
        let result = transaction_manager.record_sale(&mut inventory, "Widget", 10, 55.0);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Not enough stock to complete the sale.");
        assert_eq!(inventory.products.get("Widget").unwrap().quantity, 5); // Stock unchanged
        assert!(transaction_manager.transactions.is_empty());
    }

    #[test]
    fn record_purchase_of_existing_product() {
        let mut inventory = Inventory::new();
        let mut transaction_manager = TransactionManager::new();

        // Add a product to the inventory
        inventory
            .products
            .insert("Widget".to_string(), Product {
                name: "Widget".to_string(),
                description: "A test widget".to_string(),
                price: 50.0,
                quantity: 5,
            });

        // Record a purchase
        let result = transaction_manager.record_purchase(&mut inventory, "Widget", 20, 45.0);

        assert!(result.is_ok());
        assert_eq!(inventory.products.get("Widget").unwrap().quantity, 25); // Stock increased
        assert_eq!(transaction_manager.transactions.len(), 1);

        let transaction = &transaction_manager.transactions[0];
        assert_eq!(transaction.transaction_type, TransactionType::Purchase);
        assert_eq!(transaction.product_name, "Widget");
        assert_eq!(transaction.quantity, 20);
        assert_eq!(transaction.price, 45.0);
        assert_eq!(transaction.total, 900.0); // 20 * 45
    }

    #[test]
    fn record_purchase_of_new_product() {
        let mut inventory = Inventory::new();
        let mut transaction_manager = TransactionManager::new();

        // Record a purchase for a new product
        let result = transaction_manager.record_purchase(&mut inventory, "Gadget", 15, 30.0);

        assert!(result.is_ok());
        let product = inventory.products.get("Gadget").unwrap();
        assert_eq!(product.quantity, 15); // New stock added
        assert_eq!(product.price, 30.0); // Price set as purchase price

        assert_eq!(transaction_manager.transactions.len(), 1);

        let transaction = &transaction_manager.transactions[0];
        assert_eq!(transaction.transaction_type, TransactionType::Purchase);
        assert_eq!(transaction.product_name, "Gadget");
        assert_eq!(transaction.quantity, 15);
        assert_eq!(transaction.price, 30.0);
        assert_eq!(transaction.total, 450.0); // 15 * 30
    }

    #[test]
    fn test_list_transactions() {
        let mut transaction_manager = TransactionManager::new();

        // Add some dummy transactions
        transaction_manager.transactions.push(Transaction {
            transaction_type: TransactionType::Sale,
            product_name: "Widget".to_string(),
            quantity: 5,
            price: 50.0,
            total: 250.0,
        });

        transaction_manager.transactions.push(Transaction {
            transaction_type: TransactionType::Purchase,
            product_name: "Gadget".to_string(),
            quantity: 10,
            price: 30.0,
            total: 300.0,
        });

        let transactions = transaction_manager.list_transactions();
        assert_eq!(transactions.len(), 2);

        let first = &transactions[0];
        assert_eq!(first.transaction_type, TransactionType::Sale);
        assert_eq!(first.product_name, "Widget");
        assert_eq!(first.total, 250.0);

        let second = &transactions[1];
        assert_eq!(second.transaction_type, TransactionType::Purchase);
        assert_eq!(second.product_name, "Gadget");
        assert_eq!(second.total, 300.0);
    }
}