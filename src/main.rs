use rusty_store::{
    auth::{self, AuthManager},
    display_menu, execute, Store,
};

#[tokio::main]
async fn main() {
    let auth_manager = AuthManager::new("sqlite:db/rusty_store.db").await;
    auth_manager.initialize().await;
    let mut store = Store::new();

    println!("--- User Authentication ---");
    println!("1. Login");
    println!("2. Register");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "1" => auth::login(&auth_manager).await,
        "2" => auth::register(&auth_manager).await,
        _ => {
            println!("Invalid choice, exiting.");
            return;
        }
    }

    loop {
        display_menu();

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        execute(&mut store, choice);
    }
}
