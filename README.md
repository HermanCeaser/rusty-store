# Rusty Store Inventory Management System

The Rusty Store Inventory Management System is a command-line application built with Rust for managing a retail store's inventory, sales, and purchases.

## Features

1. **Inventory Management**: Add, edit, delete products with attributes like name, description, price, and quantity.
2. **Sales Management**: Record sales transactions and calculate profits.
3. **Purchase Management**: Record purchase transactions and calculate costs.
4. **Reporting**: Generate reports for inventory, sales, and purchase history.
5. **Modular Architecture**: Organized into separate modules for reusability.
6. **Authentication**: Basic authentication system for store managers.
7. **Persistence**: Data stored in JSON file backups.

## Prerequisites

1. [Rust](https://www.rust-lang.org/) installed (version 1.70+ recommended).

## Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/rusty-store.git
    cd rusty-store
    ```

2. Install dependencies:
    ```bash
    cargo build
    ```

## Configuration

The application uses SQLite as the primary data storage. 
create a file under db folder with the name `rusty_store.db` or if you save the file as a different name, update the connection string in `main.rs`:
```rust
let auth_manager = AuthManager::new("sqlite:db/rusty_store.db").await;
```

## Usage

### Running the Application
To start the application:
```bash
cargo run
```

### Example Workflow

1. **Add a Product**:
    ```bash
    > Add Product
    Name: Laptop
    Description: High-performance laptop
    Price: 1200.50
    Quantity: 10
    Product added successfully.
    ```

2. **Record a Sale**:
    ```bash
    > Record Sale
    Product: Laptop
    Quantity: 2
    Sale recorded successfully. Total: $2401.00
    ```

8. **Generate a Report**:
    ```bash
    > Generate Report
    Inventory Report:
    -----------------
    Product: Laptop
    Description: High-performance laptop
    Price: $1200.50
    Quantity: 8

    Sales Report:
    -------------
    Product: Laptop
    Quantity Sold: 2
    Total Sales: $2401.00
    ```

### Backup and Restore

1. **Backup Inventory**:
    Inventory Data is automatically backed up to `inventory.json` when the program shuts down.
    Transaction Data is automatically backed up to `transactions.json` when the program shuts down.

2. **Restore Inventory**:
    The system will automatically load inventory data from `inventory.json` on start.
    The system will automatically load transactions data from `transactions.json` on start.

## Project Structure

```plaintext
src/
├── auth.rs                // Handles authentication logic
├── inventory.rs           // Manages product inventory
├── transactions.rs        // Handles sales and purchase transactions
├── reporting.rs           // Generates reports
├── lib.rs                 // Orchestrates modules and exposes APIs
├── main.rs                // Entry point of the application
├── util.rs                // Utility functions to format output and get user input
db/
├── rusty_store.db         // Handles Storage for authenticated users (You are to create this file manually)
├── inventory.json         // JSON file for backup storage of products in inventory
├── transactions.json      // JSON file for backup storage of transactions made

```

## Tests

Run tests to ensure the system is functioning correctly:
```bash
cargo test
```

## Contributing

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-name`.
3. Commit changes: `git commit -m 'Add feature'`.
4. Push the branch: `git push origin feature-name`.
5. Open a pull request.

## License

This project is licensed under the MIT License. See `LICENSE` for details.

## Acknowledgments

Thanks to the Rust community and [Risein](https://www.risein.com) Foundation for their amazing resources on rust.