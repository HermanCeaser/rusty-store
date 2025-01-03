use rusty_store::{inventory, Store};

fn main() {
    let mut store = Store::new();

    loop {
        println!("\n--- Rusty Store Management ---");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. List Products");
        println!("5. Exit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Enter product details:");

                println!("Name:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();

                println!("Description:");
                let mut description = String::new();
                std::io::stdin().read_line(&mut description).unwrap();

                println!("Price:");
                let mut price = String::new();
                std::io::stdin().read_line(&mut price).unwrap();
                let price: f64 = price.trim().parse().unwrap_or(-1.0);

                println!("Quantity:");
                let mut quantity = String::new();
                std::io::stdin().read_line(&mut quantity).unwrap();
                let quantity: u32 = quantity.trim().parse().unwrap_or(0);

                let product = inventory::Product {
                    name: name.trim().to_string(),
                    description: description.trim().to_string(),
                    price,
                    quantity,
                };

                inventory::add_product(&mut store.inventory, product);
            }
            "2" => {
                println!("Enter the name of the product to edit:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();

                println!("Enter new description (leave empty to skip):");
                let mut description = String::new();
                std::io::stdin().read_line(&mut description).unwrap();
                let description = if description.trim().is_empty() {
                    None
                } else {
                    Some(description.trim().to_string())
                };

                println!("Enter new price (leave empty to skip):");
                let mut price = String::new();
                std::io::stdin().read_line(&mut price).unwrap();
                let price = if price.trim().is_empty() {
                    None
                } else {
                    Some(price.trim().parse().unwrap_or(-1.0))
                };

                println!("Enter new quantity (leave empty to skip):");
                let mut quantity = String::new();
                std::io::stdin().read_line(&mut quantity).unwrap();
                let quantity = if quantity.trim().is_empty() {
                    None
                } else {
                    Some(quantity.trim().parse().unwrap_or(0))
                };

                inventory::edit_product(&mut store.inventory, name, description, price, quantity);
            }
            "3" => {
                println!("Enter the name of the product to delete:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();

                inventory::delete_product(&mut store.inventory, name);
            }
            "4" => {
                inventory::list_products(&store.inventory);
            }
            "5" => {
                println!("Exiting Rusty Store Management.");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}