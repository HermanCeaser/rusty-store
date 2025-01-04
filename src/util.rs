use std::io;

pub fn format_table(headers: Vec<&str>, rows: Vec<Vec<String>>) -> String {
    let mut table = String::new();

    let column_widths: Vec<usize> = headers
    .iter()
    .enumerate()
    .map(|(i, header)| {
        let max_data_width = rows.iter().map(|row| row[i].len()).max().unwrap_or(0);
        header.len().max(max_data_width)
    })
    .collect();

     // Add headers to the table
    for (i, header) in headers.iter().enumerate() {
        table.push_str(&format!("{:<width$} ", header, width = column_widths[i]));
    }
    table.push('\n');

    // add a separator row
    for width in &column_widths {
        table.push_str(&format!("{:-<width$} ", "-", width = *width));
    }
    table.push('\n');

    // Add rows to the table
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            table.push_str(&format!("{:<width$} ", cell, width = column_widths[i]));
        }
        table.push('\n');
    }

    table


}

/// Formats data as a simple list
pub fn format_list(headers: Vec<&str>, rows: Vec<Vec<String>>) -> String {
    let mut list = String::new();

    for (index, row) in rows.iter().enumerate() {
        list.push_str(&format!("Item {}:\n", index + 1));
        for (header, cell) in headers.iter().zip(row) {
            list.push_str(&format!("  {}: {}\n", header, cell));
        }
        list.push('\n');
    }

    list
}


pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout()).unwrap(); // Ensure the prompt is printed before input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}