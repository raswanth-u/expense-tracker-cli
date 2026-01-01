//! Debit card command handlers for the non-interactive CLI

use anyhow::{Result, Context};
use serde_json::json;

use crate::api::ApiClient;
use crate::cli::DebitCommands;
use crate::display::Display;
use crate::models::DebitCardCreate;

/// Handle all debit card subcommands
pub fn handle_debit_command(api: &ApiClient, display: &Display, cmd: DebitCommands, json_output: bool) -> Result<()> {
    match cmd {
        DebitCommands::Add { name, last_four, user_id, account_id, daily_limit, tags } => {
            add_debit_card(api, display, name, last_four, user_id, account_id, daily_limit, tags, json_output)
        }
        DebitCommands::List { user_id, active } => list_debit_cards(api, display, user_id, active, json_output),
        DebitCommands::View { id } => view_debit_card(api, display, id, json_output),
        DebitCommands::Update { id, name, daily_limit, active } => {
            update_debit_card(api, display, id, name, daily_limit, active, json_output)
        }
        DebitCommands::Delete { id, force } => delete_debit_card(api, id, force, json_output),
        DebitCommands::Transactions { id, from, to } => {
            debit_transactions(api, display, id, from, to, json_output)
        }
    }
}

fn add_debit_card(
    api: &ApiClient,
    _display: &Display,
    name: String,
    last_four: String,
    user_id: i32,
    account_id: i32,
    daily_limit: Option<f64>,
    tags: Option<String>,
    json_output: bool,
) -> Result<()> {
    // Validate last_four
    if last_four.len() != 4 || !last_four.chars().all(|c| c.is_ascii_digit()) {
        anyhow::bail!("Last four digits must be exactly 4 numeric characters");
    }
    
    let card_create = DebitCardCreate {
        user_id,
        card_name: name,
        last_four,
        savings_account_id: account_id,
        daily_limit,
        tags,
    };
    
    let card = api.create_debit_card(&card_create)
        .context("Failed to create debit card")?;
    
    // Get linked account details
    let details = api.get_debit_card_details(card.id.unwrap_or(0))?;
    let linked_account = details.get("linked_account").cloned();
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "id": card.id.unwrap_or(0),
            "debit_card": {
                "id": card.id,
                "user_id": card.user_id,
                "card_name": card.card_name,
                "last_four": card.last_four,
                "savings_account_id": card.savings_account_id,
                "daily_limit": card.daily_limit,
                "tags": card.tags,
                "is_active": card.is_active
            },
            "linked_account": linked_account
        }))?);
        return Ok(());
    }
    
    println!("✓ Debit card created with ID: {}", card.id.unwrap_or(0));
    println!("   Linked to savings account ID: {}", card.savings_account_id);
    
    // Show the created card with linked account details
    if let Some(account) = linked_account {
        println!("\n📦 Linked Account:");
        println!("   Name: {}", account.get("account_name").and_then(|v| v.as_str()).unwrap_or("N/A"));
        println!("   Bank: {}", account.get("bank_name").and_then(|v| v.as_str()).unwrap_or("N/A"));
        println!("   Balance: ${:.2}", account.get("current_balance").and_then(|v| v.as_f64()).unwrap_or(0.0));
    }
    
    Ok(())
}

fn list_debit_cards(api: &ApiClient, _display: &Display, user_id: Option<i32>, _active: bool, json_output: bool) -> Result<()> {
    let cards = api.list_debit_cards(user_id)
        .context("Failed to fetch debit cards")?;
    
    if json_output {
        let cards_json: Vec<_> = cards.iter().map(|card| {
            json!({
                "id": card.id,
                "user_id": card.user_id,
                "card_name": card.card_name,
                "last_four": card.last_four,
                "savings_account_id": card.savings_account_id,
                "daily_limit": card.daily_limit,
                "tags": card.tags,
                "is_active": card.is_active
            })
        }).collect();
        
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "count": cards.len(),
            "debit_cards": cards_json
        }))?);
        return Ok(());
    }
    
    if cards.is_empty() {
        println!("No debit cards found.");
        return Ok(());
    }
    
    println!("Found {} debit card(s)\n", cards.len());
    
    // Display as table
    println!("{:<6} {:<20} {:<10} {:<12} {:<15}", "ID", "Name", "Last Four", "Daily Limit", "Account ID");
    println!("{}", "-".repeat(70));
    
    for card in &cards {
        let limit_str = card.daily_limit
            .map(|l| format!("${:.2}", l))
            .unwrap_or_else(|| "No limit".to_string());
        
        println!("{:<6} {:<20} ****{:<6} {:<12} {}",
            card.id.unwrap_or(0),
            &card.card_name,
            &card.last_four,
            limit_str,
            card.savings_account_id
        );
    }
    
    Ok(())
}

fn view_debit_card(api: &ApiClient, _display: &Display, id: i32, json_output: bool) -> Result<()> {
    let details = api.get_debit_card_details(id)
        .context(format!("Failed to fetch debit card {}", id))?;
    
    let card = &details["card"];
    let account = &details["linked_account"];
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "debit_card": card,
            "linked_account": account
        }))?);
        return Ok(());
    }
    
    println!("💳 Debit Card Details\n");
    println!("Card Information:");
    println!("   ID: {}", card.get("id").and_then(|v| v.as_i64()).unwrap_or(0));
    println!("   Name: {}", card.get("card_name").and_then(|v| v.as_str()).unwrap_or("N/A"));
    println!("   Last Four: ****{}", card.get("last_four").and_then(|v| v.as_str()).unwrap_or("????"));
    
    if let Some(limit) = card.get("daily_limit").and_then(|v| v.as_f64()) {
        println!("   Daily Limit: ${:.2}", limit);
    } else {
        println!("   Daily Limit: No limit");
    }
    
    let active = card.get("is_active").and_then(|v| v.as_bool()).unwrap_or(true);
    println!("   Status: {}", if active { "Active" } else { "Inactive" });
    
    println!("\n🏦 Linked Savings Account:");
    println!("   Account ID: {}", account.get("id").and_then(|v| v.as_i64()).unwrap_or(0));
    println!("   Name: {}", account.get("account_name").and_then(|v| v.as_str()).unwrap_or("N/A"));
    println!("   Bank: {}", account.get("bank_name").and_then(|v| v.as_str()).unwrap_or("N/A"));
    println!("   Balance: ${:.2}", account.get("current_balance").and_then(|v| v.as_f64()).unwrap_or(0.0));
    println!("   Last Four: ****{}", account.get("account_number_last_four").and_then(|v| v.as_str()).unwrap_or("????"));
    
    // Get recent transactions and calculate stats in frontend
    if let Ok(transactions) = api.get_debit_card_transactions(id, None, None) {
        if !transactions.is_empty() {
            println!("\n📊 Transaction Statistics (calculated in frontend):");
            
            let withdrawals: f64 = transactions.iter()
                .filter(|t| t.transaction_type == "withdrawal")
                .map(|t| t.amount)
                .sum();
            let deposits: f64 = transactions.iter()
                .filter(|t| t.transaction_type == "deposit")
                .map(|t| t.amount)
                .sum();
            
            println!("   Total Withdrawals: ${:.2}", withdrawals);
            println!("   Total Deposits: ${:.2}", deposits);
            println!("   Transaction Count: {}", transactions.len());
        }
    }
    
    Ok(())
}

fn update_debit_card(
    api: &ApiClient,
    _display: &Display,
    id: i32,
    name: Option<String>,
    daily_limit: Option<f64>,
    active: Option<bool>,
    json_output: bool,
) -> Result<()> {
    let current = api.get_debit_card(id)
        .context(format!("Failed to fetch debit card {}", id))?;
    
    let card_update = DebitCardCreate {
        user_id: current.user_id,
        card_name: name.unwrap_or(current.card_name),
        last_four: current.last_four,
        savings_account_id: current.savings_account_id,
        daily_limit: daily_limit.or(current.daily_limit),
        tags: current.tags,
    };
    
    if active.is_some() && !json_output {
        println!("Note: Active status update not yet fully implemented in API");
    }
    
    let updated = api.update_debit_card(id, &card_update)
        .context(format!("Failed to update debit card {}", id))?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "id": id,
            "debit_card": {
                "id": updated.id,
                "user_id": updated.user_id,
                "card_name": updated.card_name,
                "last_four": updated.last_four,
                "savings_account_id": updated.savings_account_id,
                "daily_limit": updated.daily_limit,
                "tags": updated.tags,
                "is_active": updated.is_active
            }
        }))?);
        return Ok(());
    }
    
    println!("✓ Debit card {} updated successfully", id);
    println!("   Name: {}", updated.card_name);
    if let Some(limit) = updated.daily_limit {
        println!("   Daily Limit: ${:.2}", limit);
    }
    
    Ok(())
}

fn delete_debit_card(api: &ApiClient, id: i32, force: bool, json_output: bool) -> Result<()> {
    if !force && !json_output {
        let card = api.get_debit_card(id)
            .context(format!("Failed to fetch debit card {}", id))?;
        
        println!("About to delete debit card:");
        println!("  ID: {}", id);
        println!("  Name: {}", card.card_name);
        println!("  Last Four: ****{}", card.last_four);
        println!("  Linked Account ID: {}", card.savings_account_id);
        println!("\nUse --force to skip this confirmation.");
        
        anyhow::bail!("Confirmation required. Use --force to delete.");
    }
    
    if !force && json_output {
        anyhow::bail!("Confirmation required. Use --force to delete.");
    }
    
    api.delete_debit_card(id)
        .context(format!("Failed to delete debit card {}", id))?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "id": id,
            "message": format!("Debit card {} deleted successfully", id)
        }))?);
        return Ok(());
    }
    
    println!("✓ Debit card {} deleted successfully", id);
    
    Ok(())
}

fn debit_transactions(
    api: &ApiClient,
    _display: &Display,
    id: i32,
    from: Option<String>,
    to: Option<String>,
    json_output: bool,
) -> Result<()> {
    let transactions = api.get_debit_card_transactions(id, from.as_deref(), to.as_deref())
        .context(format!("Failed to fetch transactions for debit card {}", id))?;
    
    if json_output {
        let withdrawals: f64 = transactions.iter()
            .filter(|t| t.transaction_type == "withdrawal")
            .map(|t| t.amount)
            .sum();
        let deposits: f64 = transactions.iter()
            .filter(|t| t.transaction_type == "deposit")
            .map(|t| t.amount)
            .sum();
        
        let txn_json: Vec<_> = transactions.iter().map(|txn| {
            json!({
                "id": txn.id,
                "savings_account_id": txn.savings_account_id,
                "transaction_type": txn.transaction_type,
                "amount": txn.amount,
                "balance_after": txn.balance_after,
                "date": txn.date,
                "description": txn.description
            })
        }).collect();
        
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "debit_card_id": id,
            "count": transactions.len(),
            "transactions": txn_json,
            "summary": {
                "total_withdrawals": withdrawals,
                "total_deposits": deposits,
                "net_change": deposits - withdrawals
            }
        }))?);
        return Ok(());
    }
    
    if transactions.is_empty() {
        println!("No transactions found for debit card {}", id);
        return Ok(());
    }
    
    println!("Found {} transaction(s) for debit card {}\n", transactions.len(), id);
    
    // Display transactions as table
    println!("{:<8} {:<12} {:>12} {:>12} {}", "ID", "Type", "Amount", "Balance", "Date");
    println!("{}", "-".repeat(60));
    
    for txn in &transactions {
        println!("{:<8} {:<12} {:>12.2} {:>12.2} {}",
            txn.id.unwrap_or(0),
            txn.transaction_type,
            txn.amount,
            txn.balance_after,
            txn.date
        );
    }
    
    // Calculate summary in frontend
    let withdrawals: f64 = transactions.iter()
        .filter(|t| t.transaction_type == "withdrawal")
        .map(|t| t.amount)
        .sum();
    let deposits: f64 = transactions.iter()
        .filter(|t| t.transaction_type == "deposit")
        .map(|t| t.amount)
        .sum();
    
    println!("\n📊 Summary (calculated in frontend):");
    println!("   Total Withdrawals: ${:.2}", withdrawals);
    println!("   Total Deposits: ${:.2}", deposits);
    println!("   Net Change: ${:.2}", deposits - withdrawals);
    
    Ok(())
}
