//! Savings account command handlers

use anyhow::{Result, Context};
use chrono::Local;
use serde_json::json;

use crate::api::ApiClient;
use crate::cli::AccountCommands;
use crate::display::Display;
use crate::models::{SavingsAccountCreate, SavingsAccountDeposit, SavingsAccountWithdraw};

pub fn handle_account_command(api: &ApiClient, display: &Display, cmd: AccountCommands, json_output: bool) -> Result<()> {
    match cmd {
        AccountCommands::Add { name, bank, last_four, user_id, account_type, balance, min_balance, interest_rate, tags } => {
            add_account(api, display, name, bank, last_four, user_id, account_type, balance, min_balance, interest_rate, tags, json_output)
        }
        AccountCommands::List { user_id, active } => {
            list_accounts(api, display, user_id, active, json_output)
        }
        AccountCommands::View { id } => {
            view_account(api, display, id, json_output)
        }
        AccountCommands::Update { id, name, min_balance, interest_rate, active } => {
            update_account(api, display, id, name, min_balance, interest_rate, active, json_output)
        }
        AccountCommands::Delete { id, force } => {
            delete_account(api, id, force, json_output)
        }
        AccountCommands::Deposit { id, amount, description } => {
            deposit_to_account(api, id, amount, description, json_output)
        }
        AccountCommands::Withdraw { id, amount, description } => {
            withdraw_from_account(api, id, amount, description, json_output)
        }
        AccountCommands::Transactions { id, txn_type, from, to } => {
            list_transactions(api, display, id, txn_type, from, to, json_output)
        }
        AccountCommands::Summary { user_id } => {
            show_summary(api, display, user_id, json_output)
        }
    }
}

fn add_account(
    api: &ApiClient,
    display: &Display,
    name: String,
    bank: String,
    last_four: String,
    user_id: i32,
    account_type: String,
    balance: f64,
    min_balance: f64,
    interest_rate: f64,
    tags: Option<String>,
    json_output: bool,
) -> Result<()> {
    let account = SavingsAccountCreate {
        user_id,
        account_name: name,
        bank_name: bank,
        account_number_last_four: last_four,
        account_type,
        minimum_balance: min_balance,
        interest_rate,
        tags,
    };
    let created = api.create_savings_account(&account)?;
    let account_id = created.id.unwrap_or(0);
    
    // If initial balance provided, deposit it
    let initial_deposit_made = if balance > 0.0 {
        let deposit = SavingsAccountDeposit {
            amount: balance,
            date: Some(Local::now().format("%Y-%m-%d").to_string()),
            description: Some("Initial deposit".to_string()),
            tags: None,
        };
        api.deposit_to_account(account_id, &deposit)?;
        true
    } else {
        false
    };
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "message": "Savings account created",
            "account_id": account_id,
            "initial_deposit": if initial_deposit_made { Some(balance) } else { None },
            "account": created
        }))?);
        return Ok(());
    }
    
    println!("✓ Savings account created with ID: {}", account_id);
    if initial_deposit_made {
        println!("   Initial deposit of ${:.2} made", balance);
    }
    display.show("savings_accounts", &vec![created])?;
    Ok(())
}

fn list_accounts(
    api: &ApiClient,
    display: &Display,
    user_id: Option<i32>,
    _active: bool,
    json_output: bool,
) -> Result<()> {
    let accounts = api.list_savings_accounts(user_id)?;
    let total: f64 = accounts.iter().map(|a| a.current_balance).sum();
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "count": accounts.len(),
            "total_balance": total,
            "accounts": accounts
        }))?);
        return Ok(());
    }
    
    if accounts.is_empty() {
        println!("No savings accounts found.");
        return Ok(());
    }
    println!("Found {} account(s)", accounts.len());
    display.show("savings_accounts", &accounts)?;
    println!("\n📊 Summary: Total Balance: ${:.2}", total);
    Ok(())
}

fn view_account(
    api: &ApiClient,
    display: &Display,
    id: i32,
    json_output: bool,
) -> Result<()> {
    let summary = api.get_account_summary(id)?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "account_id": id,
            "summary": summary
        }))?);
        return Ok(());
    }
    
    display.show("savings_account_summary", &summary)?;
    Ok(())
}

fn update_account(
    api: &ApiClient,
    display: &Display,
    id: i32,
    name: Option<String>,
    min_balance: Option<f64>,
    interest_rate: Option<f64>,
    _active: Option<bool>,
    json_output: bool,
) -> Result<()> {
    // Get current account from list (no direct get method)
    let accounts = api.list_savings_accounts(None)?;
    let current = accounts.into_iter()
        .find(|a| a.id == Some(id))
        .context(format!("Account {} not found", id))?;
    let update = SavingsAccountCreate {
        user_id: current.user_id,
        account_name: name.unwrap_or(current.account_name),
        bank_name: current.bank_name,
        account_number_last_four: current.account_number_last_four,
        account_type: current.account_type,
        minimum_balance: min_balance.unwrap_or(current.minimum_balance),
        interest_rate: interest_rate.unwrap_or(current.interest_rate),
        tags: current.tags,
    };
    let updated = api.update_savings_account(id, &update)?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "message": "Account updated",
            "account_id": id,
            "account": updated
        }))?);
        return Ok(());
    }
    
    println!("✓ Account {} updated", id);
    display.show("savings_accounts", &vec![updated])?;
    Ok(())
}

fn delete_account(
    api: &ApiClient,
    id: i32,
    force: bool,
    json_output: bool,
) -> Result<()> {
    if !force {
        // Get account from list
        let accounts = api.list_savings_accounts(None)?;
        if let Some(account) = accounts.into_iter().find(|a| a.id == Some(id)) {
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": false,
                    "message": "Use --force to confirm deletion",
                    "account_id": id,
                    "account_name": account.account_name,
                    "bank_name": account.bank_name,
                    "current_balance": account.current_balance
                }))?);
                return Ok(());
            }
            println!("About to delete: {} at {} (Balance: ${:.2})", 
                account.account_name, account.bank_name, account.current_balance);
        }
        anyhow::bail!("Use --force to confirm deletion");
    }
    api.delete_savings_account(id)?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "message": "Account deleted",
            "account_id": id
        }))?);
        return Ok(());
    }
    
    println!("✓ Account {} deleted", id);
    Ok(())
}

fn deposit_to_account(
    api: &ApiClient,
    id: i32,
    amount: f64,
    description: Option<String>,
    json_output: bool,
) -> Result<()> {
    let deposit = SavingsAccountDeposit {
        amount,
        date: Some(Local::now().format("%Y-%m-%d").to_string()),
        description,
        tags: None,
    };
    let result = api.deposit_to_account(id, &deposit)?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "message": "Deposit successful",
            "account_id": id,
            "amount": amount,
            "new_balance": result.current_balance
        }))?);
        return Ok(());
    }
    
    println!("✓ Deposited ${:.2} to account {}", amount, id);
    println!("   New Balance: ${:.2}", result.current_balance);
    Ok(())
}

fn withdraw_from_account(
    api: &ApiClient,
    id: i32,
    amount: f64,
    description: Option<String>,
    json_output: bool,
) -> Result<()> {
    let withdraw = SavingsAccountWithdraw {
        amount,
        date: Some(Local::now().format("%Y-%m-%d").to_string()),
        description,
        tags: None,
    };
    let result = api.withdraw_from_account(id, &withdraw)?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "message": "Withdrawal successful",
            "account_id": id,
            "amount": amount,
            "new_balance": result.current_balance
        }))?);
        return Ok(());
    }
    
    println!("✓ Withdrew ${:.2} from account {}", amount, id);
    println!("   New Balance: ${:.2}", result.current_balance);
    Ok(())
}

fn list_transactions(
    api: &ApiClient,
    display: &Display,
    id: i32,
    txn_type: Option<String>,
    from: Option<String>,
    to: Option<String>,
    json_output: bool,
) -> Result<()> {
    let transactions = api.get_account_transactions(id, from.as_deref(), to.as_deref(), txn_type.as_deref())?;
    
    let count = transactions.as_array().map(|a| a.len()).unwrap_or(0);
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "account_id": id,
            "count": count,
            "transactions": transactions
        }))?);
        return Ok(());
    }
    
    display.show("savings_account_transactions", &transactions)?;
    Ok(())
}

fn show_summary(
    api: &ApiClient,
    display: &Display,
    user_id: Option<i32>,
    json_output: bool,
) -> Result<()> {
    let summary = api.get_all_accounts_summary(user_id)?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "summary": summary
        }))?);
        return Ok(());
    }
    
    display.show("all_savings_accounts_summary", &summary)?;
    Ok(())
}
