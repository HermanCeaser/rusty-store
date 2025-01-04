pub mod inventory;
pub mod transaction;
pub mod reporting;
mod util;

use inventory::Inventory;
use transaction::TransactionManager;

pub struct Store {
    pub inventory: Inventory,
    pub transaction_manager: TransactionManager,
}

impl Store {
    pub fn new() -> Self {
        Store {
            inventory: Inventory::new(),
            transaction_manager: TransactionManager::new(),
        }
    }
}

pub fn execute(store: &mut Store, choice: &str) {
    match choice {
        "1" => inventory::add_product(&mut store.inventory),
        "2" => inventory::edit_product(&mut store.inventory),
        "3" => inventory::delete_product(&mut store.inventory),
        "4" => inventory::list_products(&mut store.inventory),
        "5" => transaction::handle_sale_transaction(
            &mut store.transaction_manager,
            &mut store.inventory,
        ),
        "6" => transaction::handle_purchase_transaction(
            &mut store.transaction_manager,
            &mut store.inventory,
        ),
        "7" => transaction::list_transactions(&store.transaction_manager),
        "8" => reporting::generate_reports(&store.transaction_manager, &store.inventory),
        "9" => {
            println!("--- Exiting Rusty Store ---");
            println!("Goodbye!");
            std::process::exit(0);
        }
        _ => println!("Invalid choice, please try again."),
    }
}

pub fn display_menu() {
    println!("\n--- Rusty Store Management ---");
    println!("1. Add Product");
    println!("2. Edit Product");
    println!("3. Delete Product");
    println!("4. List Products");
    println!("5. Record Sale");
    println!("6. Record Purchase");
    println!("7. List Transactions");
    println!("8. Generate Reports");
    println!("9. Exit");
}
