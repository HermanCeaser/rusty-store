use rusty_store::{display_menu, execute, Store};

fn main() {
    let mut store = Store::new();

    loop {
        display_menu();

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();


        execute(&mut store, choice);
    }
}