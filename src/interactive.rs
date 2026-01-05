//! Interactive CLI Mode
//!
//! Provides a menu-driven interface that reuses the existing command handlers
//! from *_cmd.rs files. Users navigate through menus and provide input
//! interactively instead of remembering CLI arguments.

use anyhow::Result;

use crate::api::ApiClient;
use crate::cli::*;
use crate::commands;
use crate::config::Config;
use crate::constants::{CATEGORIES, PAYMENT_METHODS, RECURRING_FREQUENCIES, ASSET_TYPES};
use crate::display::Display;
use crate::ui::*;

/// Entry point for interactive mode
pub fn run_interactive(api: &ApiClient, display: &Display, _config: &Config) -> Result<()> {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║       Welcome to Family Expense Tracker (Interactive)     ║");
    println!("║           Navigate using numbered menu options            ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    loop {
        match show_main_menu() {
            Ok(MainMenuOption::Expenses) => {
                if let Err(e) = handle_expense_interactive(api, display) {
                    print_error(&format!("Error: {}", e));
                }
            }
            Ok(MainMenuOption::Budgets) => {
                if let Err(e) = handle_budget_interactive(api, display) {
                    print_error(&format!("Error: {}", e));
                }
            }
            Ok(MainMenuOption::Users) => {
                if let Err(e) = handle_user_interactive(api, display) {
                    print_error(&format!("Error: {}", e));
                }
            }
            Ok(MainMenuOption::Cards) => {
                if let Err(e) = handle_card_interactive(api, display) {
                    print_error(&format!("Error: {}", e));
                }
            }
            Ok(MainMenuOption::DebitCards) => {
                if let Err(e) = handle_debit_interactive(api, display) {
                    print_error(&format!("Error: {}", e));
                }
            }
            Ok(MainMenuOption::SavingsAccounts) => {
                if let Err(e) = handle_account_interactive(api, display) {
                    print_error(&format!("Error: {}", e));
                }
            }
            Ok(MainMenuOption::SavingsGoals) => {
                if let Err(e) = handle_goal_interactive(api, display) {
                    print_error(&format!("Error: {}", e));
                }
            }
            Ok(MainMenuOption::Assets) => {
                if let Err(e) = handle_asset_interactive(api, display) {
                    print_error(&format!("Error: {}", e));
                }
            }
            Ok(MainMenuOption::Recurring) => {
                if let Err(e) = handle_recurring_interactive(api, display) {
                    print_error(&format!("Error: {}", e));
                }
            }
            Ok(MainMenuOption::Reports) => {
                if let Err(e) = handle_report_interactive(api, display) {
                    print_error(&format!("Error: {}", e));
                }
            }
            Ok(MainMenuOption::Search) => {
                if let Err(e) = handle_search_interactive(api, display) {
                    print_error(&format!("Error: {}", e));
                }
            }
            Ok(MainMenuOption::Backup) => {
                if let Err(e) = handle_backup_interactive(api, display) {
                    print_error(&format!("Error: {}", e));
                }
            }
            Ok(MainMenuOption::Exit) => {
                println!("\n👋 Goodbye! Thank you for using Family Expense Tracker.\n");
                break;
            }
            Err(e) => {
                print_error(&format!("Menu error: {}", e));
            }
        }
    }

    Ok(())
}

// ============================================================================
// EXPENSE INTERACTIVE HANDLER
// ============================================================================

fn handle_expense_interactive(api: &ApiClient, display: &Display) -> Result<()> {
    loop {
        match show_expense_menu()? {
            ExpenseMenuOption::Create => {
                // Gather expense details interactively
                print_section_header("Add New Expense");
                
                // Select user
                let user_id = interactive_select_user(api, display)?;
                
                // Get amount
                let amount = prompt_float("Amount", None)?;
                
                // Select category
                let category = interactive_select_category()?;
                
                // Optional description
                let description = prompt_string("Description (press Enter to skip)", None, true)?;
                let description = if description.is_empty() { None } else { Some(description) };
                
                // Select date
                let date = Some(select_date_preset()?);
                
                // Select payment method
                let (payment, card_id, debit_id, account_id) = interactive_select_payment(api, display, user_id)?;
                
                // Tags
                let tags = interactive_select_tags()?;
                
                // Is recurring?
                let recurring = prompt_confirm("Is this a recurring expense?", false)?;
                
                // Build and execute command
                let cmd = ExpenseCommands::Add {
                    amount,
                    category,
                    user_id,
                    description,
                    date,
                    payment,
                    card_id,
                    debit_id,
                    account_id,
                    tags,
                    recurring,
                };
                commands::handle_expense_command(api, display, cmd, false)?;
            }
            ExpenseMenuOption::List => {
                print_section_header("List Expenses");
                
                // Optional filters
                let user_id = if prompt_confirm("Filter by user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                
                let category = if prompt_confirm("Filter by category?", false)? {
                    Some(interactive_select_category()?)
                } else {
                    None
                };
                
                let payment = if prompt_confirm("Filter by payment method?", false)? {
                    Some(interactive_select_payment_method()?)
                } else {
                    None
                };
                
                let (from, to) = if prompt_confirm("Filter by date range?", false)? {
                    (
                        Some(prompt_string("From date (YYYY-MM-DD)", None, false)?),
                        Some(prompt_string("To date (YYYY-MM-DD)", None, false)?),
                    )
                } else {
                    (None, None)
                };
                
                let limit = if prompt_confirm("Limit results?", false)? {
                    Some(prompt_int("Max results", Some(20), Some(1), Some(1000))?)
                } else {
                    None
                };
                
                let cmd = ExpenseCommands::List {
                    user_id,
                    category,
                    payment,
                    min: None,
                    max: None,
                    from,
                    to,
                    tags: None,
                    limit,
                };
                commands::handle_expense_command(api, display, cmd, false)?;
            }
            ExpenseMenuOption::View => {
                let id = prompt_int("Expense ID", None, Some(1), None)?;
                let cmd = ExpenseCommands::View { id };
                commands::handle_expense_command(api, display, cmd, false)?;
            }
            ExpenseMenuOption::Update => {
                print_section_header("Update Expense");
                let id = prompt_int("Expense ID to update", None, Some(1), None)?;
                
                // Show current expense first
                let _ = commands::handle_expense_command(api, display, ExpenseCommands::View { id }, false);
                
                println!("\nEnter new values (press Enter to keep current):");
                
                let amount = if prompt_confirm("Update amount?", false)? {
                    Some(prompt_float("New amount", None)?)
                } else {
                    None
                };
                
                let category = if prompt_confirm("Update category?", false)? {
                    Some(interactive_select_category()?)
                } else {
                    None
                };
                
                let description = if prompt_confirm("Update description?", false)? {
                    Some(prompt_string("New description", None, true)?)
                } else {
                    None
                };
                
                let date = if prompt_confirm("Update date?", false)? {
                    Some(select_date_preset()?)
                } else {
                    None
                };
                
                let payment = if prompt_confirm("Update payment method?", false)? {
                    Some(interactive_select_payment_method()?)
                } else {
                    None
                };
                
                let cmd = ExpenseCommands::Update {
                    id,
                    amount,
                    category,
                    description,
                    date,
                    payment,
                    tags: None,
                };
                commands::handle_expense_command(api, display, cmd, false)?;
            }
            ExpenseMenuOption::Delete => {
                let id = prompt_int("Expense ID to delete", None, Some(1), None)?;
                
                // Show expense first
                let _ = commands::handle_expense_command(api, display, ExpenseCommands::View { id }, false);
                
                if prompt_confirm("Are you sure you want to delete this expense?", false)? {
                    let cmd = ExpenseCommands::Delete { id, force: true };
                    commands::handle_expense_command(api, display, cmd, false)?;
                } else {
                    print_info("Deletion cancelled");
                }
            }
            ExpenseMenuOption::SummaryByCategory | ExpenseMenuOption::SummaryByPayment => {
                let user_id = if prompt_confirm("For specific user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                let month = Some(select_month_preset()?);
                
                let cmd = ExpenseCommands::Summary { user_id, month };
                commands::handle_expense_command(api, display, cmd, false)?;
            }
            ExpenseMenuOption::Back => break,
        }
    }
    Ok(())
}

// ============================================================================
// USER INTERACTIVE HANDLER
// ============================================================================

fn handle_user_interactive(api: &ApiClient, display: &Display) -> Result<()> {
    loop {
        match show_user_menu()? {
            UserMenuOption::Create => {
                print_section_header("Add New User");
                
                let name = prompt_string("Name", None, false)?;
                let email = prompt_email("Email", None)?;
                let role_options = vec!["member".to_string(), "admin".to_string()];
                let role_idx = select_with_number("Role", &role_options)?;
                let role = role_options[role_idx].clone();
                
                let cmd = UserCommands::Add { name, email, role };
                commands::handle_user_command(api, display, cmd, false)?;
            }
            UserMenuOption::List => {
                let active = prompt_confirm("Show only active users?", true)?;
                let cmd = UserCommands::List { active };
                commands::handle_user_command(api, display, cmd, false)?;
            }
            UserMenuOption::View => {
                let id = interactive_select_user(api, display)?;
                let cmd = UserCommands::View { id };
                commands::handle_user_command(api, display, cmd, false)?;
            }
            UserMenuOption::Update => {
                print_section_header("Update User");
                let id = interactive_select_user(api, display)?;
                
                // Show current user
                let _ = commands::handle_user_command(api, display, UserCommands::View { id }, false);
                
                println!("\nEnter new values (select what to update):");
                
                let name = if prompt_confirm("Update name?", false)? {
                    Some(prompt_string("New name", None, false)?)
                } else {
                    None
                };
                
                let email = if prompt_confirm("Update email?", false)? {
                    Some(prompt_email("New email", None)?)
                } else {
                    None
                };
                
                let role = if prompt_confirm("Update role?", false)? {
                    let role_options = vec!["member".to_string(), "admin".to_string()];
                    let role_idx = select_with_number("Role", &role_options)?;
                    Some(role_options[role_idx].clone())
                } else {
                    None
                };
                
                let cmd = UserCommands::Update { id, name, email, role, active: None };
                commands::handle_user_command(api, display, cmd, false)?;
            }
            UserMenuOption::Deactivate => {
                let id = interactive_select_user(api, display)?;
                
                if prompt_confirm("Are you sure you want to deactivate this user?", false)? {
                    let cmd = UserCommands::Delete { id, force: true };
                    commands::handle_user_command(api, display, cmd, false)?;
                }
            }
            UserMenuOption::Stats => {
                let id = interactive_select_user(api, display)?;
                let month = if prompt_confirm("For specific month?", false)? {
                    Some(select_month_preset()?)
                } else {
                    None
                };
                
                let cmd = UserCommands::Stats { id, month };
                commands::handle_user_command(api, display, cmd, false)?;
            }
            UserMenuOption::Back => break,
        }
    }
    Ok(())
}

// ============================================================================
// BUDGET INTERACTIVE HANDLER
// ============================================================================

fn handle_budget_interactive(api: &ApiClient, display: &Display) -> Result<()> {
    loop {
        match show_budget_menu()? {
            BudgetMenuOption::Create => {
                print_section_header("Create Budget");
                
                let user_id = if prompt_confirm("For specific user? (No = family budget)", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                
                let category = interactive_select_category()?;
                let amount = prompt_float("Budget amount", None)?;
                let month = select_month_preset()?;
                
                let period_options = vec!["monthly".to_string(), "weekly".to_string(), "yearly".to_string()];
                let period_idx = select_with_number("Budget period", &period_options)?;
                let period = period_options[period_idx].clone();
                
                let tags = interactive_select_tags()?;
                
                let cmd = BudgetCommands::Add { category, amount, month, user_id, period, tags };
                commands::handle_budget_command(api, display, cmd, false)?;
            }
            BudgetMenuOption::List => {
                let user_id = if prompt_confirm("Filter by user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                let month = if prompt_confirm("Filter by month?", false)? {
                    Some(select_month_preset()?)
                } else {
                    None
                };
                let category = if prompt_confirm("Filter by category?", false)? {
                    Some(interactive_select_category()?)
                } else {
                    None
                };
                
                let cmd = BudgetCommands::List { user_id, month, category, active: false };
                commands::handle_budget_command(api, display, cmd, false)?;
            }
            BudgetMenuOption::View => {
                let id = prompt_int("Budget ID", None, Some(1), None)?;
                let cmd = BudgetCommands::View { id };
                commands::handle_budget_command(api, display, cmd, false)?;
            }
            BudgetMenuOption::Update => {
                let id = prompt_int("Budget ID to update", None, Some(1), None)?;
                
                let amount = if prompt_confirm("Update amount?", false)? {
                    Some(prompt_float("New amount", None)?)
                } else {
                    None
                };
                
                let category = if prompt_confirm("Update category?", false)? {
                    Some(interactive_select_category()?)
                } else {
                    None
                };
                
                let cmd = BudgetCommands::Update { id, amount, category, active: None };
                commands::handle_budget_command(api, display, cmd, false)?;
            }
            BudgetMenuOption::Delete => {
                let id = prompt_int("Budget ID to delete", None, Some(1), None)?;
                
                if prompt_confirm("Are you sure you want to delete this budget?", false)? {
                    let cmd = BudgetCommands::Delete { id, force: true };
                    commands::handle_budget_command(api, display, cmd, false)?;
                }
            }
            BudgetMenuOption::Status => {
                let month = if prompt_confirm("For specific month?", true)? {
                    Some(select_month_preset()?)
                } else {
                    None
                };
                let user_id = if prompt_confirm("For specific user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                
                let cmd = BudgetCommands::Status { month, user_id };
                commands::handle_budget_command(api, display, cmd, false)?;
            }
            BudgetMenuOption::Alerts => {
                let cmd = BudgetCommands::Status { month: None, user_id: None };
                commands::handle_budget_command(api, display, cmd, false)?;
            }
            BudgetMenuOption::Compare => {
                let month1 = prompt_string("First month (YYYY-MM)", None, false)?;
                let month2 = prompt_string("Second month (YYYY-MM)", None, false)?;
                let user_id = if prompt_confirm("For specific user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                
                let cmd = BudgetCommands::Compare { month1, month2, user_id };
                commands::handle_budget_command(api, display, cmd, false)?;
            }
            BudgetMenuOption::Back => break,
        }
    }
    Ok(())
}

// ============================================================================
// CREDIT CARD INTERACTIVE HANDLER
// ============================================================================

fn handle_card_interactive(api: &ApiClient, display: &Display) -> Result<()> {
    loop {
        match show_card_menu()? {
            CardMenuOption::Create => {
                print_section_header("Add Credit Card");
                
                let user_id = interactive_select_user(api, display)?;
                let name = prompt_string("Card name (e.g., Chase Sapphire)", None, false)?;
                let last_four = prompt_string("Last 4 digits", None, false)?;
                let limit = prompt_float("Credit limit", None)?;
                let billing_day = prompt_int("Billing day (1-31)", Some(1), Some(1), Some(31))?;
                let tags = interactive_select_tags()?;
                
                let cmd = CardCommands::Add { name, last_four, limit, user_id, billing_day, tags };
                commands::handle_card_command(api, display, cmd, false)?;
            }
            CardMenuOption::List => {
                let user_id = if prompt_confirm("Filter by user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                let cmd = CardCommands::List { user_id, active: false };
                commands::handle_card_command(api, display, cmd, false)?;
            }
            CardMenuOption::View => {
                let id = interactive_select_credit_card(api, display)?;
                let cmd = CardCommands::View { id };
                commands::handle_card_command(api, display, cmd, false)?;
            }
            CardMenuOption::Update => {
                let id = interactive_select_credit_card(api, display)?;
                
                let name = if prompt_confirm("Update name?", false)? {
                    Some(prompt_string("New name", None, false)?)
                } else {
                    None
                };
                
                let limit = if prompt_confirm("Update credit limit?", false)? {
                    Some(prompt_float("New limit", None)?)
                } else {
                    None
                };
                
                let billing_day = if prompt_confirm("Update billing day?", false)? {
                    Some(prompt_int("New billing day", None, Some(1), Some(31))?)
                } else {
                    None
                };
                
                let cmd = CardCommands::Update { id, name, limit, billing_day, active: None };
                commands::handle_card_command(api, display, cmd, false)?;
            }
            CardMenuOption::Deactivate => {
                let id = interactive_select_credit_card(api, display)?;
                
                if prompt_confirm("Are you sure you want to deactivate this card?", false)? {
                    let cmd = CardCommands::Delete { id, force: true };
                    commands::handle_card_command(api, display, cmd, false)?;
                }
            }
            CardMenuOption::Statement => {
                let id = interactive_select_credit_card(api, display)?;
                let month = select_month_preset()?;
                
                let cmd = CardCommands::Statement { id, month };
                commands::handle_card_command(api, display, cmd, false)?;
            }
            CardMenuOption::Utilization => {
                let months = prompt_int("Number of months to analyze", Some(6), Some(1), Some(24))?;
                let cmd = CardCommands::Utilization { months };
                commands::handle_card_command(api, display, cmd, false)?;
            }
            CardMenuOption::Summary => {
                // List all cards as summary
                let cmd = CardCommands::List { user_id: None, active: false };
                commands::handle_card_command(api, display, cmd, false)?;
            }
            CardMenuOption::Back => break,
        }
    }
    Ok(())
}

// ============================================================================
// DEBIT CARD INTERACTIVE HANDLER
// ============================================================================

fn handle_debit_interactive(api: &ApiClient, display: &Display) -> Result<()> {
    loop {
        match show_debit_menu()? {
            DebitMenuOption::Create => {
                print_section_header("Add Debit Card");
                
                let user_id = interactive_select_user(api, display)?;
                let name = prompt_string("Card name", None, false)?;
                let last_four = prompt_string("Last 4 digits", None, false)?;
                
                // Select the account - frontend validation: must be same user's account
                let account = interactive_select_savings_account(api, display)?;
                
                // Get the selected account details to verify ownership
                let accounts = api.list_savings_accounts(None)?;
                let selected_account = accounts.iter().find(|a| a.id == Some(account));
                
                if let Some(acc) = selected_account {
                    if acc.user_id != user_id {
                        print_error(&format!(
                            "⚠️  Account validation: Account {} is owned by User {}, but card owner is User {}",
                            acc.id.unwrap_or(0), acc.user_id, user_id
                        ));
                        print_info("Debit card owner and account owner must be the same.");
                        continue;
                    }
                } else {
                    print_error("Account not found");
                    continue;
                }
                
                let daily_limit = if prompt_confirm("Set daily spending limit?", false)? {
                    Some(prompt_float("Daily limit", None)?)
                } else {
                    None
                };
                
                let tags = interactive_select_tags()?;
                
                let cmd = DebitCommands::Add {
                    name,
                    last_four,
                    user_id,
                    account_id: account,
                    daily_limit,
                    tags,
                };
                commands::handle_debit_command(api, display, cmd, false)?;
            }
            DebitMenuOption::List => {
                let user_id = if prompt_confirm("Filter by user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                let cmd = DebitCommands::List { user_id, active: false };
                commands::handle_debit_command(api, display, cmd, false)?;
            }
            DebitMenuOption::View => {
                let id = interactive_select_debit_card(api, display)?;
                let cmd = DebitCommands::View { id };
                commands::handle_debit_command(api, display, cmd, false)?;
            }
            DebitMenuOption::Update => {
                let id = interactive_select_debit_card(api, display)?;
                
                let name = if prompt_confirm("Update name?", false)? {
                    Some(prompt_string("New name", None, false)?)
                } else {
                    None
                };
                
                let daily_limit = if prompt_confirm("Update daily limit?", false)? {
                    Some(prompt_float("New daily limit", None)?)
                } else {
                    None
                };
                
                let cmd = DebitCommands::Update { id, name, daily_limit, active: None };
                commands::handle_debit_command(api, display, cmd, false)?;
            }
            DebitMenuOption::Deactivate => {
                let id = interactive_select_debit_card(api, display)?;
                
                if prompt_confirm("Are you sure you want to deactivate this card?", false)? {
                    let cmd = DebitCommands::Delete { id, force: true };
                    commands::handle_debit_command(api, display, cmd, false)?;
                }
            }
            DebitMenuOption::Transactions => {
                let id = interactive_select_debit_card(api, display)?;
                
                let (from, to) = if prompt_confirm("Filter by date range?", false)? {
                    (
                        Some(prompt_string("From date (YYYY-MM-DD)", None, false)?),
                        Some(prompt_string("To date (YYYY-MM-DD)", None, false)?),
                    )
                } else {
                    (None, None)
                };
                
                let cmd = DebitCommands::Transactions { id, from, to };
                commands::handle_debit_command(api, display, cmd, false)?;
            }
            DebitMenuOption::Summary => {
                // List all debit cards as summary
                let cmd = DebitCommands::List { user_id: None, active: false };
                commands::handle_debit_command(api, display, cmd, false)?;
            }
            DebitMenuOption::Back => break,
        }
    }
    Ok(())
}

// ============================================================================
// SAVINGS ACCOUNT INTERACTIVE HANDLER
// ============================================================================

fn handle_account_interactive(api: &ApiClient, display: &Display) -> Result<()> {
    loop {
        match show_savings_account_menu()? {
            SavingsAccountMenuOption::Create => {
                print_section_header("Add Savings Account");
                
                let user_id = interactive_select_user(api, display)?;
                let name = prompt_string("Account name", None, false)?;
                let bank = prompt_string("Bank name", None, false)?;
                let last_four = prompt_string("Account last 4 digits", None, false)?;
                
                let account_types = vec!["savings".to_string(), "checking".to_string(), "money_market".to_string()];
                let type_idx = select_with_number("Account type", &account_types)?;
                let account_type = account_types[type_idx].clone();
                
                let interest_rate = if prompt_confirm("Add interest rate?", false)? {
                    prompt_float("Interest rate (%)", Some(0.0))?
                } else {
                    0.0
                };
                
                let min_balance = if prompt_confirm("Add minimum balance requirement?", false)? {
                    prompt_float("Minimum balance", Some(0.0))?
                } else {
                    0.0
                };
                
                let tags = interactive_select_tags()?;
                
                let cmd = AccountCommands::Add {
                    name,
                    bank,
                    last_four,
                    account_type,
                    user_id,
                    balance: 0.0,
                    interest_rate,
                    min_balance,
                    tags,
                };
                commands::handle_account_command(api, display, cmd, false)?;
            }
            SavingsAccountMenuOption::List => {
                let user_id = if prompt_confirm("Filter by user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                let cmd = AccountCommands::List { user_id, active: false };
                commands::handle_account_command(api, display, cmd, false)?;
            }
            SavingsAccountMenuOption::View => {
                let id = interactive_select_savings_account(api, display)?;
                let cmd = AccountCommands::View { id };
                commands::handle_account_command(api, display, cmd, false)?;
            }
            SavingsAccountMenuOption::Deposit => {
                let id = interactive_select_savings_account(api, display)?;
                let amount = prompt_float("Deposit amount", None)?;
                let description = prompt_string("Description (optional)", None, true)?;
                let description = if description.is_empty() { None } else { Some(description) };
                
                let cmd = AccountCommands::Deposit { id, amount, description };
                commands::handle_account_command(api, display, cmd, false)?;
            }
            SavingsAccountMenuOption::Withdraw => {
                let id = interactive_select_savings_account(api, display)?;
                let amount = prompt_float("Withdrawal amount", None)?;
                let description = prompt_string("Description (optional)", None, true)?;
                let description = if description.is_empty() { None } else { Some(description) };
                
                let cmd = AccountCommands::Withdraw { id, amount, description };
                commands::handle_account_command(api, display, cmd, false)?;
            }
            SavingsAccountMenuOption::Transactions => {
                let id = interactive_select_savings_account(api, display)?;
                
                let (from, to) = if prompt_confirm("Filter by date range?", false)? {
                    (
                        Some(prompt_string("From date (YYYY-MM-DD)", None, false)?),
                        Some(prompt_string("To date (YYYY-MM-DD)", None, false)?),
                    )
                } else {
                    (None, None)
                };
                
                let cmd = AccountCommands::Transactions { id, from, to, txn_type: None };
                commands::handle_account_command(api, display, cmd, false)?;
            }
            SavingsAccountMenuOption::Delete => {
                let id = interactive_select_savings_account(api, display)?;
                
                if prompt_confirm("Are you sure you want to delete this account?", false)? {
                    let cmd = AccountCommands::Delete { id, force: true };
                    commands::handle_account_command(api, display, cmd, false)?;
                }
            }
            SavingsAccountMenuOption::Back => break,
        }
    }
    Ok(())
}

// ============================================================================
// SAVINGS GOAL INTERACTIVE HANDLER
// ============================================================================

fn handle_goal_interactive(api: &ApiClient, display: &Display) -> Result<()> {
    loop {
        match show_savings_menu()? {
            SavingsMenuOption::Create => {
                print_section_header("Create Savings Goal");
                
                let user_id = interactive_select_user(api, display)?;
                let name = prompt_string("Goal name", None, false)?;
                let target = prompt_float("Target amount", None)?;
                let deadline = prompt_string("Target date (YYYY-MM-DD)", None, false)?;
                let current = if prompt_confirm("Set initial amount?", false)? {
                    prompt_float("Initial amount", Some(0.0))?
                } else {
                    0.0
                };
                let description = prompt_string("Description (optional)", None, true)?;
                let description = if description.is_empty() { None } else { Some(description) };
                let tags = interactive_select_tags()?;
                
                let cmd = GoalCommands::Add {
                    name,
                    target,
                    deadline,
                    user_id,
                    current,
                    description,
                    tags,
                };
                commands::handle_goal_command(api, display, cmd, false)?;
            }
            SavingsMenuOption::List => {
                let user_id = if prompt_confirm("Filter by user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                let cmd = GoalCommands::List { user_id, active: false };
                commands::handle_goal_command(api, display, cmd, false)?;
            }
            SavingsMenuOption::View => {
                let id = interactive_select_savings_goal(api, display)?;
                let cmd = GoalCommands::View { id };
                commands::handle_goal_command(api, display, cmd, false)?;
            }
            SavingsMenuOption::Deposit => {
                let id = interactive_select_savings_goal(api, display)?;
                let amount = prompt_float("Contribution amount", None)?;
                
                let cmd = GoalCommands::Contribute { id, amount };
                commands::handle_goal_command(api, display, cmd, false)?;
            }
            SavingsMenuOption::Withdraw => {
                // Update goal to reduce current amount
                let id = interactive_select_savings_goal(api, display)?;
                let amount = prompt_float("Withdrawal amount", None)?;
                
                // We need to get current amount and subtract
                print_info("Withdrawing from goal (reducing progress)...");
                let cmd = GoalCommands::Contribute { id, amount: -amount };
                commands::handle_goal_command(api, display, cmd, false)?;
            }
            SavingsMenuOption::Delete => {
                let id = interactive_select_savings_goal(api, display)?;
                
                if prompt_confirm("Are you sure you want to delete this goal?", false)? {
                    let cmd = GoalCommands::Delete { id, force: true };
                    commands::handle_goal_command(api, display, cmd, false)?;
                }
            }
            SavingsMenuOption::Back => break,
        }
    }
    Ok(())
}

// ============================================================================
// ASSET INTERACTIVE HANDLER
// ============================================================================

fn handle_asset_interactive(api: &ApiClient, display: &Display) -> Result<()> {
    loop {
        match show_asset_menu()? {
            AssetMenuOption::Create => {
                print_section_header("Add Asset");
                
                let user_id = interactive_select_user(api, display)?;
                let name = prompt_string("Asset name", None, false)?;
                
                let asset_type = interactive_select_asset_type()?;
                let purchase_value = prompt_float("Purchase value", None)?;
                let current_value = prompt_float("Current value", Some(purchase_value))?;
                let purchase_date = select_date_preset()?;
                
                let (payment, card_id, _, account_id) = interactive_select_payment(api, display, user_id)?;
                
                let description = prompt_string("Description (optional)", None, true)?;
                let description = if description.is_empty() { None } else { Some(description) };
                
                let location = prompt_string("Location (optional)", None, true)?;
                let location = if location.is_empty() { None } else { Some(location) };
                
                let tags = interactive_select_tags()?;
                
                let cmd = AssetCommands::Add {
                    name,
                    asset_type,
                    purchase_value,
                    current_value,
                    purchase_date,
                    user_id,
                    payment,
                    card_id,
                    account_id,
                    description,
                    location,
                    tags,
                };
                commands::handle_asset_command(api, display, cmd, false)?;
            }
            AssetMenuOption::List => {
                let user_id = if prompt_confirm("Filter by user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                
                let asset_type = if prompt_confirm("Filter by asset type?", false)? {
                    Some(interactive_select_asset_type()?)
                } else {
                    None
                };
                
                let cmd = AssetCommands::List { user_id, asset_type, active: false };
                commands::handle_asset_command(api, display, cmd, false)?;
            }
            AssetMenuOption::View => {
                let id = interactive_select_asset(api, display)?;
                let cmd = AssetCommands::View { id };
                commands::handle_asset_command(api, display, cmd, false)?;
            }
            AssetMenuOption::Update => {
                let id = interactive_select_asset(api, display)?;
                
                let name = if prompt_confirm("Update name?", false)? {
                    Some(prompt_string("New name", None, false)?)
                } else {
                    None
                };
                
                let current_value = if prompt_confirm("Update current value?", false)? {
                    Some(prompt_float("New current value", None)?)
                } else {
                    None
                };
                
                let location = if prompt_confirm("Update location?", false)? {
                    Some(prompt_string("New location", None, true)?)
                } else {
                    None
                };
                
                let cmd = AssetCommands::Update { id, name, current_value, location, active: None };
                commands::handle_asset_command(api, display, cmd, false)?;
            }
            AssetMenuOption::UpdateValue => {
                let id = interactive_select_asset(api, display)?;
                let value = prompt_float("New value", None)?;
                
                let cmd = AssetCommands::Value { id, value };
                commands::handle_asset_command(api, display, cmd, false)?;
            }
            AssetMenuOption::Delete => {
                let id = interactive_select_asset(api, display)?;
                
                if prompt_confirm("Are you sure you want to delete this asset?", false)? {
                    let cmd = AssetCommands::Delete { id, force: true };
                    commands::handle_asset_command(api, display, cmd, false)?;
                }
            }
            AssetMenuOption::Summary => {
                let user_id = if prompt_confirm("For specific user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                let cmd = AssetCommands::Summary { user_id };
                commands::handle_asset_command(api, display, cmd, false)?;
            }
            AssetMenuOption::Back => break,
        }
    }
    Ok(())
}

// ============================================================================
// RECURRING EXPENSE INTERACTIVE HANDLER
// ============================================================================

fn handle_recurring_interactive(api: &ApiClient, display: &Display) -> Result<()> {
    loop {
        match show_recurring_menu()? {
            RecurringMenuOption::Create => {
                print_section_header("Create Recurring Expense Template");
                
                let user_id = interactive_select_user(api, display)?;
                let amount = prompt_float("Amount", None)?;
                let category = interactive_select_category()?;
                let frequency = interactive_select_frequency()?;
                let start_date = prompt_string("Start date (YYYY-MM-DD)", None, false)?;
                
                let description = prompt_string("Description (optional)", None, true)?;
                let description = if description.is_empty() { None } else { Some(description) };
                
                let interval = prompt_int("Interval (every N periods)", Some(1), Some(1), Some(365))?;
                
                let day_of_week = if frequency == "weekly" {
                    Some(prompt_int("Day of week (0=Monday, 6=Sunday)", Some(0), Some(0), Some(6))?)
                } else {
                    None
                };
                
                let day_of_month = if frequency == "monthly" {
                    Some(prompt_int("Day of month (1-31)", Some(1), Some(1), Some(31))?)
                } else {
                    None
                };
                
                let end_date = if prompt_confirm("Set end date?", false)? {
                    Some(prompt_string("End date (YYYY-MM-DD)", None, false)?)
                } else {
                    None
                };
                
                let tags = interactive_select_tags()?;
                
                let cmd = RecurringCommands::Add {
                    amount,
                    category,
                    user_id,
                    frequency,
                    start_date,
                    description,
                    interval,
                    day_of_week,
                    day_of_month,
                    end_date,
                    tags,
                };
                commands::handle_recurring_command(api, display, cmd, false)?;
            }
            RecurringMenuOption::List => {
                let user_id = if prompt_confirm("Filter by user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                let cmd = RecurringCommands::List { user_id, active: false };
                commands::handle_recurring_command(api, display, cmd, false)?;
            }
            RecurringMenuOption::View => {
                let id = interactive_select_recurring_template(api, display)?;
                let cmd = RecurringCommands::View { id };
                commands::handle_recurring_command(api, display, cmd, false)?;
            }
            RecurringMenuOption::Update => {
                let id = interactive_select_recurring_template(api, display)?;
                
                let amount = if prompt_confirm("Update amount?", false)? {
                    Some(prompt_float("New amount", None)?)
                } else {
                    None
                };
                
                let category = if prompt_confirm("Update category?", false)? {
                    Some(interactive_select_category()?)
                } else {
                    None
                };
                
                let cmd = RecurringCommands::Update { id, amount, category, active: None };
                commands::handle_recurring_command(api, display, cmd, false)?;
            }
            RecurringMenuOption::Generate => {
                let cmd = RecurringCommands::Process;
                commands::handle_recurring_command(api, display, cmd, false)?;
            }
            RecurringMenuOption::Pause => {
                let id = interactive_select_recurring_template(api, display)?;
                let active = prompt_confirm("Set as active?", true)?;
                
                let cmd = RecurringCommands::Update { id, amount: None, category: None, active: Some(active) };
                commands::handle_recurring_command(api, display, cmd, false)?;
            }
            RecurringMenuOption::Delete => {
                let id = interactive_select_recurring_template(api, display)?;
                
                if prompt_confirm("Are you sure you want to delete this template?", false)? {
                    let cmd = RecurringCommands::Delete { id, force: true };
                    commands::handle_recurring_command(api, display, cmd, false)?;
                }
            }
            RecurringMenuOption::Back => break,
        }
    }
    Ok(())
}

// ============================================================================
// REPORT INTERACTIVE HANDLER
// ============================================================================

fn handle_report_interactive(api: &ApiClient, display: &Display) -> Result<()> {
    loop {
        match show_report_menu()? {
            ReportMenuOption::Monthly => {
                let month = select_month_preset()?;
                let user_id = if prompt_confirm("For specific user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                
                let cmd = ReportCommands::Monthly { month, user_id };
                commands::handle_report_command(api, display, cmd, false)?;
            }
            ReportMenuOption::Family => {
                let month = select_month_preset()?;
                
                let cmd = ReportCommands::Family { month };
                commands::handle_report_command(api, display, cmd, false)?;
            }
            ReportMenuOption::Category => {
                let category = interactive_select_category()?;
                let from = prompt_string("From date (YYYY-MM-DD)", None, false)?;
                let to = prompt_string("To date (YYYY-MM-DD)", None, false)?;
                let user_id = if prompt_confirm("For specific user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                
                let cmd = ReportCommands::Category { category, from, to, user_id };
                commands::handle_report_command(api, display, cmd, false)?;
            }
            ReportMenuOption::Trends => {
                let months = prompt_int("Number of months", Some(6), Some(1), Some(24))?;
                let user_id = if prompt_confirm("For specific user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                
                let cmd = ReportCommands::Trends { months, user_id };
                commands::handle_report_command(api, display, cmd, false)?;
            }
            ReportMenuOption::Payments => {
                let month = select_month_preset()?;
                let user_id = if prompt_confirm("For specific user?", false)? {
                    Some(interactive_select_user(api, display)?)
                } else {
                    None
                };
                
                let cmd = ReportCommands::Payments { month, user_id };
                commands::handle_report_command(api, display, cmd, false)?;
            }
            ReportMenuOption::Export => {
                print_info("Export functionality - use dashboard command with --json flag");
                print_info("Example: expense dashboard --month 2026-01 --json");
            }
            ReportMenuOption::Back => break,
        }
    }
    Ok(())
}

// ============================================================================
// SEARCH INTERACTIVE HANDLER
// ============================================================================

fn handle_search_interactive(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Search");
    
    let query = prompt_string("Search query", None, false)?;
    
    let entity = if prompt_confirm("Search specific entity type?", false)? {
        let entities = vec![
            "expenses".to_string(),
            "users".to_string(),
            "budgets".to_string(),
            "cards".to_string(),
            "accounts".to_string(),
            "goals".to_string(),
            "assets".to_string(),
        ];
        let idx = select_with_number("Entity type", &entities)?;
        Some(entities[idx].clone())
    } else {
        None
    };
    
    let (from, to) = if prompt_confirm("Filter by date range?", false)? {
        (
            Some(prompt_string("From date (YYYY-MM-DD)", None, false)?),
            Some(prompt_string("To date (YYYY-MM-DD)", None, false)?),
        )
    } else {
        (None, None)
    };
    
    let (min, max) = if prompt_confirm("Filter by amount range?", false)? {
        (
            Some(prompt_float("Minimum amount", None)?),
            Some(prompt_float("Maximum amount", None)?),
        )
    } else {
        (None, None)
    };
    
    let args = SearchArgs { query, entity, from, to, min, max };
    commands::handle_search_command(api, display, args, false)?;
    
    Ok(())
}

// ============================================================================
// BACKUP INTERACTIVE HANDLER
// ============================================================================

fn handle_backup_interactive(api: &ApiClient, display: &Display) -> Result<()> {
    loop {
        match show_backup_menu()? {
            BackupMenuOption::Create => {
                let output = if prompt_confirm("Specify output path?", false)? {
                    Some(prompt_string("Output file path", None, false)?)
                } else {
                    None
                };
                
                let cmd = BackupCommands::Create { output };
                commands::handle_backup_command(api, display, cmd, false)?;
            }
            BackupMenuOption::Restore => {
                // List available backups first
                print_info("Available backups:");
                let _ = commands::handle_backup_command(api, display, BackupCommands::List, false);
                
                let filename = prompt_string("Backup filename to restore", None, false)?;
                
                if prompt_confirm("Are you sure you want to restore this backup? This will overwrite current data.", false)? {
                    let cmd = BackupCommands::Restore { file: filename, force: true };
                    commands::handle_backup_command(api, display, cmd, false)?;
                }
            }
            BackupMenuOption::List => {
                let cmd = BackupCommands::List;
                commands::handle_backup_command(api, display, cmd, false)?;
            }
            BackupMenuOption::Back => break,
        }
    }
    Ok(())
}

// ============================================================================
// HELPER FUNCTIONS FOR INTERACTIVE SELECTION
// ============================================================================

/// Select a user interactively from the list
fn interactive_select_user(api: &ApiClient, display: &Display) -> Result<i32> {
    let users = api.list_users(Some(true))?;
    
    if users.is_empty() {
        print_error("No active users found. Please create a user first.");
        anyhow::bail!("No users available");
    }
    
    display.show("users", &users)?;
    
    let user_options: Vec<String> = users.iter()
        .map(|u| format!("ID:{} - {} ({})", u.id.unwrap_or(0), u.name, u.email))
        .collect();
    
    let idx = select_with_number("Select User", &user_options)?;
    Ok(users[idx].id.unwrap_or(0))
}

/// Select a category from predefined list
fn interactive_select_category() -> Result<String> {
    let categories: Vec<String> = CATEGORIES.iter().map(|s| s.to_string()).collect();
    let idx = select_with_number("Select Category", &categories)?;
    Ok(categories[idx].clone())
}

/// Select a payment method (without account/card)
fn interactive_select_payment_method() -> Result<String> {
    let methods: Vec<String> = PAYMENT_METHODS.iter().map(|s| s.to_string()).collect();
    let idx = select_with_number("Select Payment Method", &methods)?;
    Ok(methods[idx].clone())
}

/// Select payment method with associated card/account selection
/// In a family expense tracker, any family member can use any card/account
fn interactive_select_payment(api: &ApiClient, display: &Display, _user_id: i32) -> Result<(String, Option<i32>, Option<i32>, Option<i32>)> {
    let method = interactive_select_payment_method()?;
    
    let mut card_id = None;
    let mut debit_id = None;
    let mut account_id = None;
    
    match method.as_str() {
        "credit_card" => {
            // Show all family credit cards, not just the user's
            let cards = api.list_credit_cards(None)?;
            if cards.is_empty() {
                print_error("No credit cards found. Please add a credit card first.");
                anyhow::bail!("No credit cards available");
            }
            display.show("credit_cards", &cards)?;
            let options: Vec<String> = cards.iter()
                .map(|c| format!("ID:{} - {} (****{}) - Owner ID:{}", c.id.unwrap_or(0), c.card_name, c.last_four, c.user_id))
                .collect();
            let idx = select_with_number("Select Credit Card", &options)?;
            card_id = cards[idx].id;
        }
        "debit_card" => {
            // Show all family debit cards
            let debits = api.list_debit_cards(None)?;
            if debits.is_empty() {
                print_error("No debit cards found. Please add a debit card first.");
                anyhow::bail!("No debit cards available");
            }
            display.show("debit_cards", &debits)?;
            let options: Vec<String> = debits.iter()
                .map(|d| format!("ID:{} - {} (****{}) - Owner ID:{}", d.id.unwrap_or(0), d.card_name, d.last_four, d.user_id))
                .collect();
            let idx = select_with_number("Select Debit Card", &options)?;
            debit_id = debits[idx].id;
        }
        "savings_account" => {
            // Show all family savings accounts
            let accounts = api.list_savings_accounts(None)?;
            if accounts.is_empty() {
                print_error("No savings accounts found. Please add a savings account first.");
                anyhow::bail!("No savings accounts available");
            }
            display.show("savings_accounts", &accounts)?;
            let options: Vec<String> = accounts.iter()
                .map(|a| format!("ID:{} - {} ({}) - ${:.2} - Owner ID:{}", a.id.unwrap_or(0), a.account_name, a.bank_name, a.current_balance, a.user_id))
                .collect();
            let idx = select_with_number("Select Savings Account", &options)?;
            account_id = accounts[idx].id;
        }
        _ => {}
    }
    
    Ok((method, card_id, debit_id, account_id))
}

/// Select tags interactively
fn interactive_select_tags() -> Result<Option<String>> {
    if !prompt_confirm("Add tags?", false)? {
        return Ok(None);
    }
    
    let tags = prompt_string("Enter tags (comma-separated)", None, true)?;
    if tags.is_empty() {
        Ok(None)
    } else {
        Ok(Some(tags))
    }
}

/// Select a credit card
fn interactive_select_credit_card(api: &ApiClient, display: &Display) -> Result<i32> {
    let cards = api.list_credit_cards(None)?;
    
    if cards.is_empty() {
        print_error("No credit cards found.");
        anyhow::bail!("No credit cards available");
    }
    
    display.show("credit_cards", &cards)?;
    
    let options: Vec<String> = cards.iter()
        .map(|c| format!("ID:{} - {} (****{}) - Limit: ${:.2}", c.id.unwrap_or(0), c.card_name, c.last_four, c.credit_limit))
        .collect();
    
    let idx = select_with_number("Select Credit Card", &options)?;
    Ok(cards[idx].id.unwrap_or(0))
}

/// Select a debit card
fn interactive_select_debit_card(api: &ApiClient, display: &Display) -> Result<i32> {
    let cards = api.list_debit_cards(None)?;
    
    if cards.is_empty() {
        print_error("No debit cards found.");
        anyhow::bail!("No debit cards available");
    }
    
    display.show("debit_cards", &cards)?;
    
    let options: Vec<String> = cards.iter()
        .map(|c| format!("ID:{} - {} (****{}) - Owner ID:{}", c.id.unwrap_or(0), c.card_name, c.last_four, c.user_id))
        .collect();
    
    let idx = select_with_number("Select Debit Card", &options)?;
    Ok(cards[idx].id.unwrap_or(0))
}

/// Select a savings account
fn interactive_select_savings_account(api: &ApiClient, display: &Display) -> Result<i32> {
    let accounts = api.list_savings_accounts(None)?;
    
    if accounts.is_empty() {
        print_error("No savings accounts found.");
        anyhow::bail!("No savings accounts available");
    }
    
    display.show("savings_accounts", &accounts)?;
    
    let options: Vec<String> = accounts.iter()
        .map(|a| format!("ID:{} - {} ({}) - Balance: ${:.2}", a.id.unwrap_or(0), a.account_name, a.bank_name, a.current_balance))
        .collect();
    
    let idx = select_with_number("Select Savings Account", &options)?;
    Ok(accounts[idx].id.unwrap_or(0))
}

/// Select a savings goal
fn interactive_select_savings_goal(api: &ApiClient, display: &Display) -> Result<i32> {
    let goals = api.list_savings_goals(None)?;
    
    if goals.is_empty() {
        print_error("No savings goals found.");
        anyhow::bail!("No savings goals available");
    }
    
    display.show("savings_goals", &goals)?;
    
    let options: Vec<String> = goals.iter()
        .map(|g| format!("ID:{} - {} - ${:.2}/${:.2}", g.id.unwrap_or(0), g.name, g.current_amount, g.target_amount))
        .collect();
    
    let idx = select_with_number("Select Savings Goal", &options)?;
    Ok(goals[idx].id.unwrap_or(0))
}

/// Select an asset
fn interactive_select_asset(api: &ApiClient, display: &Display) -> Result<i32> {
    let assets = api.list_assets(None, None)?;
    
    if assets.is_empty() {
        print_error("No assets found.");
        anyhow::bail!("No assets available");
    }
    
    display.show("assets", &assets)?;
    
    let options: Vec<String> = assets.iter()
        .map(|a| format!("ID:{} - {} ({}) - Value: ${:.2}", a.id.unwrap_or(0), a.name, a.asset_type, a.current_value))
        .collect();
    
    let idx = select_with_number("Select Asset", &options)?;
    Ok(assets[idx].id.unwrap_or(0))
}

/// Select an asset type
fn interactive_select_asset_type() -> Result<String> {
    let types: Vec<String> = ASSET_TYPES.iter().map(|s| s.to_string()).collect();
    let idx = select_with_number("Select Asset Type", &types)?;
    Ok(types[idx].clone())
}

/// Select a frequency
fn interactive_select_frequency() -> Result<String> {
    let frequencies: Vec<String> = RECURRING_FREQUENCIES.iter().map(|s| s.to_string()).collect();
    let idx = select_with_number("Select Frequency", &frequencies)?;
    Ok(frequencies[idx].clone())
}

/// Select a recurring template
fn interactive_select_recurring_template(api: &ApiClient, display: &Display) -> Result<i32> {
    let templates = api.list_recurring_templates(None, None)?;
    
    if templates.is_empty() {
        print_error("No recurring expense templates found.");
        anyhow::bail!("No templates available");
    }
    
    display.show("recurring_templates", &templates)?;
    
    let options: Vec<String> = templates.iter()
        .map(|t| format!("ID:{} - {} - ${:.2} ({})", t.id.unwrap_or(0), t.category, t.amount, t.frequency))
        .collect();
    
    let idx = select_with_number("Select Template", &options)?;
    Ok(templates[idx].id.unwrap_or(0))
}
