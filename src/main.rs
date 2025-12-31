mod api;
mod commands;
mod config;
mod constants;
mod display;
mod models;
mod ui;

use anyhow::Result;
use clap::Parser;
use api::ApiClient;
use config::Config;
use display::Display;
use ui::*;

#[derive(Parser)]
#[command(name = "expense")]
#[command(version = "2.0.0")]
#[command(about = "Family Expense Tracker CLI", long_about = None)]
struct Cli {
    /// Quick action: 'q' for quick expense, 't' for today, 'm' for month
    #[arg(short, long)]
    quick: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;
    let api = ApiClient::new(&config)?;
    let display = Display::new(&config);
    
    // Check for alerts on startup
    let _ = commands::check_alerts_on_startup(&api);
    
    // Handle quick actions
    if let Some(action) = cli.quick {
        return handle_quick_action(&action, &api, &display, &config);
    }
    
    // Main interactive loop
    loop {
        match show_main_menu()? {
            MainMenuOption::Dashboard => commands::show_dashboard(&api, &display)?,
            MainMenuOption::Search => commands::handle_search(&api, &display)?,
            MainMenuOption::Alerts => commands::handle_alerts(&api, &display)?,
            MainMenuOption::Backup => commands::handle_backup(&api, &display)?,
            MainMenuOption::Savings => commands::handle_savings(&api, &display)?,
            MainMenuOption::Assets => commands::handle_assets(&api, &display)?,  
            MainMenuOption::Recurring => commands::handle_recurring(&api, &display)?,
            MainMenuOption::Users => commands::handle_users(&api, &display, &config)?,
            MainMenuOption::Expenses => commands::handle_expenses(&api, &display, &config)?,
            MainMenuOption::Budgets => commands::handle_budgets(&api, &display, &config)?,
            MainMenuOption::Cards => commands::handle_cards(&api, &display, &config)?,
            MainMenuOption::Reports => commands::handle_reports(&api, &display, &config)?,
            MainMenuOption::Settings => commands::handle_settings(&api, &display, &config)?,
            MainMenuOption::Exit => {
                println!("\n👋 Goodbye!");
                break;
            }
        }
    }
    
    Ok(())
}

fn handle_quick_action(
    action: &str,
    api: &ApiClient,
    display: &Display,
    config: &Config,
) -> Result<()> {
    match action.to_lowercase().as_str() {
        "d" | "dashboard" => commands::show_dashboard(api, display)?,
        "s" | "search" => commands::handle_search(api, display)?,
        "a" | "alerts" => commands::handle_alerts(api, display)?,
        "b" | "backup" => commands::handle_backup(api, display)?,
        "g" | "goals" | "savings" => commands::handle_savings(api, display)?,
        "w" | "wealth" | "assets" => commands::handle_assets(api, display)?,  
        "r" | "recurring" => commands::handle_recurring(api, display)?,
        "q" | "quick" => quick_expense(api, display, config)?,
        "t" | "today" => today_summary(api, display)?,
        "m" | "month" => month_summary(api, display)?,
        _ => {
            print_error(&format!("Unknown quick action: {}", action));
            println!("Available quick actions:");
            println!("  d, dashboard - Dashboard view");
            println!("  s, search - Search expenses");
            println!("  a, alerts - Budget alerts");
            println!("  b, backup - Backup & Restore");
            println!("  g, goals, savings - Savings Goals");
            println!("  w, wealth, assets - Asset Management");  
            println!("  r, recurring - Recurring Expenses");
            println!("  q, quick - Quick expense entry");
            println!("  t, today - Today's summary");
            println!("  m, month - This month's summary");
        }
    }
    Ok(())
}

fn quick_expense(api: &ApiClient, display: &Display, config: &Config) -> Result<()> {
    print_section_header("Quick Expense Entry");
    
    let amount = prompt_float("Amount", None)?;
    let category = constants::CATEGORIES[0].to_string(); // Default to Food
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let payment_method = "cash".to_string();
    let user_id = config.defaults.default_user_id as i32;
    
    print_info("Creating quick expense...");
    
    let expense_create = models::ExpenseCreate {
        user_id,
        amount,
        category,
        description: None,
        date,
        payment_method,
        credit_card_id: None,
        is_recurring: Some(false),
        tags: None,
    };
    
    let expense = api.create_expense(&expense_create)?;
    print_success(&format!("Quick expense created with ID: {}", expense.id.unwrap_or(0)));
    display.show("expenses", &vec![expense])?;
    
    Ok(())
}

fn today_summary(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Today's Summary");
    
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    
    print_info("Fetching today's expenses...");
    
    let filters = models::ExpenseFilters {
        from_date: Some(today.clone()),
        to_date: Some(today),
        ..Default::default()
    };
    
    let expenses = api.list_expenses_filtered(&filters)?;
    
    if expenses.is_empty() {
        print_info("No expenses found for today");
    } else {
        let total: f64 = expenses.iter().map(|e| e.amount).sum();
        println!("\nTotal for today: ${:.2}", total);
        println!("Transactions: {}\n", expenses.len());
        display.show("expenses", &expenses)?;
    }
    
    Ok(())
}

fn month_summary(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("This Month's Summary");
    
    let month = chrono::Local::now().format("%Y-%m").to_string();
    
    print_info(&format!("Generating report for {}...", month));
    let report = api.get_monthly_report(&month, None)?;
    display.show("monthly_report", &report)?;
    
    Ok(())
}