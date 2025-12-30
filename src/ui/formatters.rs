pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

pub fn format_date(date: &str) -> String {
    date.to_string()
}

pub fn print_section_header(title: &str) {
    println!("\n─────────────────────────────────────");
    println!("{}", title);
    println!("─────────────────────────────────────\n");
}

pub fn print_success(message: &str) {
    println!("✓ {}", message);
}

pub fn print_error(message: &str) {
    println!("❌ {}", message);
}

pub fn print_info(message: &str) {
    println!("ℹ️  {}", message);
}