//! # Family Expense Tracker CLI
//!
//! A command-line interface for managing family expenses.
//!
//! ## Usage
//! ```
//! expense expense add --amount 50 --category Food --user-id 1
//! expense expense list --user-id 1 --category Food
//! expense expense view 1
//! expense user list
//! expense budget status --month 2026-01
//! expense report monthly --month 2026-01
//! expense dashboard
//! ```
//! 
//! ## Interactive Mode
//! ```
//! expense --interactive
//! ```

mod api;
mod cli;
mod commands;
mod config;
mod constants;
mod display;
mod interactive;
mod models;
mod ui;

use anyhow::Result;
use clap::Parser;

use api::ApiClient;
use cli::*;
use config::Config;
use display::Display;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;
    let api = ApiClient::new(&config)?;
    let display = Display::new(&config);
    let json_output = cli.json;
    
    // Handle interactive mode
    if cli.interactive {
        return interactive::run_interactive(&api, &display, &config);
    }
    
    // If no command provided, show help
    let Some(command) = cli.command else {
        println!("Use --interactive for menu-driven mode, or provide a command.");
        println!("Run 'expense --help' for usage information.");
        return Ok(());
    };

    match command {
        Commands::Expense(cmd) => commands::handle_expense_command(&api, &display, cmd, json_output)?,
        Commands::User(cmd) => commands::handle_user_command(&api, &display, cmd, json_output)?,
        Commands::Budget(cmd) => commands::handle_budget_command(&api, &display, cmd, json_output)?,
        Commands::Card(cmd) => commands::handle_card_command(&api, &display, cmd, json_output)?,
        Commands::Debit(cmd) => commands::handle_debit_command(&api, &display, cmd, json_output)?,
        Commands::Account(cmd) => commands::handle_account_command(&api, &display, cmd, json_output)?,
        Commands::Goal(cmd) => commands::handle_goal_command(&api, &display, cmd, json_output)?,
        Commands::Asset(cmd) => commands::handle_asset_command(&api, &display, cmd, json_output)?,
        Commands::Recurring(cmd) => commands::handle_recurring_command(&api, &display, cmd, json_output)?,
        Commands::Report(cmd) => commands::handle_report_command(&api, &display, cmd, json_output)?,
        Commands::Search(args) => commands::handle_search_command(&api, &display, args, json_output)?,
        Commands::Backup(cmd) => commands::handle_backup_command(&api, &display, cmd, json_output)?,
        Commands::Dashboard(args) => commands::handle_dashboard_command(&api, &display, args, json_output)?,
    }

    Ok(())
}
