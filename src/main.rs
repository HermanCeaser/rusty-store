use rusty_store::{Store, StoreConfig};

fn main() {
    let mut store = Store::new();
    let config = StoreConfig::new();

    loop {
        config.display_menu();

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        if choice == "6" {
            break;
        }

        if !config.execute(choice, &mut store.inventory) {
            println!("Invalid choice, please try again.");
        }
    }
}