use super::inventory::Inventory;
use super::transaction::TransactionManager;
use super::util;

pub fn generate_reports(transactions: &TransactionManager, inventory: &Inventory, ) {
    println!("\n> Generate Report\n");

    // Prompt user to choose a specific report or show all
    let choice = util::get_user_input("Choose a report: \n 1. Inventory, \n 2. Sales, \n 3. Purchases, \n (Leave blank ). All: ");

    match choice.as_str() {
        "1" => println!("{}", generate_inventory_report(inventory)),
        "2" => println!("{}", generate_sales_report(transactions)),
        "3" => println!("{}", generate_purchase_report(transactions)),
        _ => {
            println!("{}", generate_inventory_report(inventory));
            println!("{}", generate_sales_report(transactions));
            println!("{}", generate_purchase_report(transactions));
            println!("{}", generate_profit_loss_summary(transactions));
        }
    }
}

/// Generates an inventory report
fn generate_inventory_report(inventory: &Inventory) -> String {
    let headers = vec!["Product", "Description", "Price", "Quantity"];
    let rows: Vec<Vec<String>> = inventory
        .products
        .iter()
        .map(|(_name, product)| {
            vec![
                format!("{}", product.name),
                product.description.clone(),
                format!("${:.2}", product.price),
                product.quantity.to_string(),
            ]
        })
        .collect();

    let mut report = String::new();
    report.push_str("\n--- Inventory Report: ---\n");
    report.push_str("-----------------\n");
    report.push_str(&util::format_list(headers, rows));
    report
}

/// Generates a sales report
fn generate_sales_report(transactions: &TransactionManager) -> String {
    let headers = vec!["Product", "Quantity Sold", "Sale Price", "Amount"];
    let rows: Vec<Vec<String>> = transactions
        .sales()
        .iter()
        .map(|sale| {
            let amount = sale.price * sale.quantity as f64;
            vec![
                format!("{}", sale.product_name),
                sale.quantity.to_string(),
                format!("${:.2}", sale.price),
                format!("${:.2}", amount),
            ]
        })
        .collect();

    let total_sales: f64 = transactions
        .sales()
        .iter()
        .map(|sale| sale.price * sale.quantity as f64)
        .sum();

    let mut report = String::new();
    report.push_str("\n--- Sales Report: ---\n");
    report.push_str("----------------------\n");
    report.push_str(&util::format_list(headers, rows));
    report.push_str(&format!("Total Sales: ${:.2}\n", total_sales));
    report
}

/// Generates a purchase report
fn generate_purchase_report(transactions: &TransactionManager) -> String {
    let headers = vec!["Product", "Quantity Bought", "Purchase Price", "Amount"];
    let rows: Vec<Vec<String>> = transactions
        .purchases()
        .iter()
        .map(|purchase| {
            let amount = purchase.price * purchase.quantity as f64;
            vec![
                format!("{}", purchase.product_name),
                purchase.quantity.to_string(),
                format!("${:.2}", purchase.price),
                format!("${:.2}", amount),
            ]
        })
        .collect();

    let total_purchases: f64 = transactions
        .purchases()
        .iter()
        .map(|purchase| purchase.price * purchase.quantity as f64)
        .sum();

    let mut report = String::new();
    report.push_str("\n--- Purchase Report: ---\n");
    report.push_str("--------------------------\n");
    report.push_str(&util::format_list(headers, rows));
    report.push_str(&format!("Total Purchases: ${:.2}\n", total_purchases));
    report
}

/// Generates a profit/loss summary
fn generate_profit_loss_summary(transactions: &TransactionManager) -> String {
    let total_sales: f64 = transactions
        .sales()
        .iter()
        .map(|sale| sale.price * sale.quantity as f64)
        .sum();

    let total_purchases: f64 = transactions
        .purchases()
        .iter()
        .map(|purchase| purchase.price * purchase.quantity as f64)
        .sum();

    let profit_or_loss = total_sales - total_purchases;

    let mut summary = String::new();
    summary.push_str("===============================\n");
    summary.push_str(&format!(
        "Total Profit/Loss: {}${:.2}\n",
        if profit_or_loss < 0.0 { "-" } else { "" },
        profit_or_loss.abs()
    ));
    summary.push_str("===============================\n");
    summary
}
