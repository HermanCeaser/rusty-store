use std::fmt;

use crate::{
    inventory::{Inventory, Product},
    util,
};

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

    fn record_sale(
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

    fn record_purchase(
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

    fn list_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn sales(&self) -> Vec<&Transaction> {
        self
            .transactions
            .iter()
            .filter(|transaction| transaction.transaction_type == TransactionType::Sale)
            .collect()
    }

    pub fn purchases(&self) -> Vec<&Transaction> {
        self
            .transactions
            .iter()
            .filter(|transaction| transaction.transaction_type == TransactionType::Purchase)
            .collect()
    }
}

/// Handles Sales transactions.
///
/// # Arguments
///
/// * transaction_manager - A mutable instance of `TransactionManager`
/// * inventory - The Inventory list of products to sale from
///
pub fn handle_sale_transaction(
    transaction_manager: &mut TransactionManager,
    inventory: &mut Inventory,
) {
    println!("\n--- Record Sale ---");

    let product_name = util::get_user_input("Enter product name: ");
    let quantity = util::get_user_input("Enter quantity sold: ")
        .parse::<u32>()
        .unwrap_or_else(|_| {
            println!("Invalid quantity. Defaulting to 0.");
            0
        });
    let price = util::get_user_input("Enter sale price: ")
        .parse::<f64>()
        .unwrap_or_else(|_| {
            println!("Invalid price. Defaulting to 0.0.");
            0.0
        });

    match transaction_manager.record_sale(inventory, &product_name, quantity, price) {
        Ok(_) => println!("Sale recorded successfully."),
        Err(e) => println!("Error recording sale: {}", e),
    }
}

/// Handles Purchase transactions.
///
/// # Arguments
///
/// * transaction_manager - A mutable instance of `TransactionManager`
/// * inventory - The Inventory list of products to sale from
///
pub fn handle_purchase_transaction(
    transaction_manager: &mut TransactionManager,
    inventory: &mut Inventory,
) {
    println!("\n--- Record Purchase ---");

    let product_name = util::get_user_input("Enter product name: ");
    let quantity = util::get_user_input("Enter quantity purchased: ")
        .parse::<u32>()
        .unwrap_or_else(|_| {
            println!("Invalid quantity. Defaulting to 0.");
            0
        });

    let price = util::get_user_input("Enter purchase price: ")
        .parse::<f64>()
        .unwrap_or_else(|_| {
            println!("Invalid price. Defaulting to 0.0.");
            0.0
        });

    match transaction_manager.record_purchase(inventory, &product_name, quantity, price) {
        Ok(_) => println!("Purchase recorded successfully."),
        Err(e) => println!("Error recording purchase: {}", e),
    }
}

/// A public function that lists all the transactions.
///
/// # Arguments
///
/// * transaction_manager - An instance of `TransactionManager`
///
pub fn list_transactions(transaction_manager: &TransactionManager) {
    // Headers for the transaction table
    let headers = vec![
        "No",
        "Type",
        "Product",
        "Quantity",
        "Price per Unit",
        "Total Amount",
    ];

    // Collect all transactions (sales and purchases) into a single list
    let transactions = transaction_manager.list_transactions();

    let mut rows: Vec<Vec<String>> = Vec::new();
    for (i, transaction) in transactions.iter().enumerate() {
        let trans_type = match transaction.transaction_type {
            TransactionType::Sale => "Sale",
            TransactionType::Purchase => "Purchase",
        };
        let total_amount = transaction.price * transaction.quantity as f64;
        rows.push(vec![
            (i + 1).to_string(),
            trans_type.to_string(),
            transaction.product_name.clone(),
            transaction.quantity.to_string(),
            format!("${:.2}", transaction.price),
            format!("${:.2}", total_amount),
        ])
    }

    let formatted_table = util::format_table(headers, rows);
    println!("{}", formatted_table);
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
        inventory.products.insert(
            "Widget".to_string(),
            Product {
                name: "Widget".to_string(),
                description: "A test widget".to_string(),
                price: 50.0,
                quantity: 5,
            },
        );

        // Attempt to sell more than available stock
        let result = transaction_manager.record_sale(&mut inventory, "Widget", 10, 55.0);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Not enough stock to complete the sale."
        );
        assert_eq!(inventory.products.get("Widget").unwrap().quantity, 5); // Stock unchanged
        assert!(transaction_manager.transactions.is_empty());
    }

    #[test]
    fn record_purchase_of_existing_product() {
        let mut inventory = Inventory::new();
        let mut transaction_manager = TransactionManager::new();

        // Add a product to the inventory
        inventory.products.insert(
            "Widget".to_string(),
            Product {
                name: "Widget".to_string(),
                description: "A test widget".to_string(),
                price: 50.0,
                quantity: 5,
            },
        );

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
