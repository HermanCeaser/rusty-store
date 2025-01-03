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