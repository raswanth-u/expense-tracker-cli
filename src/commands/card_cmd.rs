//! Credit card command handlers for the non-interactive CLI

use anyhow::{Result, Context};
use chrono::Local;
use serde_json::json;

use crate::api::ApiClient;
use crate::cli::CardCommands;
use crate::display::Display;
use crate::models::{CreditCardCreate, CreditCardPayment};

/// Handle all credit card subcommands
pub fn handle_card_command(api: &ApiClient, display: &Display, cmd: CardCommands, json_output: bool) -> Result<()> {
    match cmd {
        CardCommands::Add { name, last_four, limit, user_id, billing_day, tags } => {
            add_card(api, display, name, last_four, limit, user_id, billing_day, tags, json_output)
        }
        CardCommands::List { user_id, active } => list_cards(api, display, user_id, active, json_output),
        CardCommands::View { id } => view_card(api, display, id, json_output),
        CardCommands::Update { id, name, limit, billing_day, active } => {
            update_card(api, display, id, name, limit, billing_day, active, json_output)
        }
        CardCommands::Delete { id, force } => delete_card(api, id, force, json_output),
        CardCommands::Statement { id, month } => card_statement(api, display, id, month, json_output),
        CardCommands::Pay { id, amount, from_account, description } => {
            pay_card(api, display, id, amount, from_account, description, json_output)
        }
        CardCommands::Transactions { id, txn_type, from, to } => {
            card_transactions(api, display, id, txn_type, from, to, json_output)
        }
        CardCommands::Utilization { months } => card_utilization(api, display, months, json_output),
    }
}

fn add_card(
    api: &ApiClient,
    display: &Display,
    name: String,
    last_four: String,
    limit: f64,
    user_id: i32,
    billing_day: i32,
    tags: Option<String>,
    json_output: bool,
) -> Result<()> {
    // Validate last_four
    if last_four.len() != 4 || !last_four.chars().all(|c| c.is_ascii_digit()) {
        anyhow::bail!("Last four digits must be exactly 4 numeric characters");
    }
    
    // Validate billing day
    if billing_day < 1 || billing_day > 31 {
        anyhow::bail!("Billing day must be between 1 and 31");
    }
    
    let card_create = CreditCardCreate {
        user_id,
        card_name: name,
        last_four,
        credit_limit: limit,
        billing_day,
        tags,
    };
    
    let card = api.create_credit_card(&card_create)
        .context("Failed to create credit card")?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "id": card.id.unwrap_or(0),
            "card": card
        }))?);
        return Ok(());
    }
    
    println!("✓ Credit card created with ID: {}", card.id.unwrap_or(0));
    display.show("credit_cards", &vec![card])?;
    
    Ok(())
}

fn list_cards(api: &ApiClient, display: &Display, user_id: Option<i32>, _active: bool, json_output: bool) -> Result<()> {
    let cards = api.list_credit_cards(user_id)
        .context("Failed to fetch credit cards")?;
    
    if json_output {
        let total_limit: f64 = cards.iter().map(|c| c.credit_limit).sum();
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "count": cards.len(),
            "cards": cards,
            "summary": {
                "total_credit_limit": total_limit,
                "number_of_cards": cards.len()
            }
        }))?);
        return Ok(());
    }
    
    if cards.is_empty() {
        println!("No credit cards found.");
        return Ok(());
    }
    
    println!("Found {} credit card(s)", cards.len());
    display.show("credit_cards", &cards)?;
    
    // Calculate totals in frontend
    let total_limit: f64 = cards.iter().map(|c| c.credit_limit).sum();
    println!("\n📊 Summary (calculated in frontend):");
    println!("   Total Credit Limit: ${:.2}", total_limit);
    println!("   Number of Cards: {}", cards.len());
    
    Ok(())
}

fn view_card(api: &ApiClient, display: &Display, id: i32, json_output: bool) -> Result<()> {
    let card = api.get_credit_card(id)
        .context(format!("Failed to fetch credit card {}", id))?;
    
    if json_output {
        let mut stats = json!({
            "total_charges": 0.0,
            "total_payments": 0.0,
            "transaction_count": 0,
            "current_balance": 0.0,
            "credit_utilization": 0.0,
            "available_credit": card.credit_limit
        });
        
        if let Ok(transactions) = api.get_credit_card_transactions(id, None, None, None) {
            let charges: f64 = transactions.iter()
                .filter(|t| t.transaction_type == "charge")
                .map(|t| t.amount)
                .sum();
            let payments: f64 = transactions.iter()
                .filter(|t| t.transaction_type == "payment")
                .map(|t| t.amount)
                .sum();
            
            stats["total_charges"] = json!(charges);
            stats["total_payments"] = json!(payments);
            stats["transaction_count"] = json!(transactions.len());
            
            if let Some(last_txn) = transactions.last() {
                let current_balance = last_txn.balance_after;
                let utilization = (current_balance / card.credit_limit) * 100.0;
                stats["current_balance"] = json!(current_balance);
                stats["credit_utilization"] = json!(utilization);
                stats["available_credit"] = json!(card.credit_limit - current_balance);
            }
        }
        
        let filters = crate::models::ExpenseFilters {
            credit_card_id: Some(id),
            ..Default::default()
        };
        if let Ok(expenses) = api.list_expenses_filtered(&filters) {
            let total: f64 = expenses.iter().map(|e| e.amount).sum();
            stats["linked_expenses_count"] = json!(expenses.len());
            stats["linked_expenses_total"] = json!(total);
        }
        
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "card": card,
            "statistics": stats
        }))?);
        return Ok(());
    }
    
    display.show("credit_cards", &vec![card.clone()])?;
    
    // Get transactions and calculate stats in frontend
    println!("\n📊 Card Statistics (calculated in frontend):");
    
    // Get transactions for this card
    if let Ok(transactions) = api.get_credit_card_transactions(id, None, None, None) {
        let charges: f64 = transactions.iter()
            .filter(|t| t.transaction_type == "charge")
            .map(|t| t.amount)
            .sum();
        let payments: f64 = transactions.iter()
            .filter(|t| t.transaction_type == "payment")
            .map(|t| t.amount)
            .sum();
        
        println!("   Total Charges: ${:.2}", charges);
        println!("   Total Payments: ${:.2}", payments);
        println!("   Transaction Count: {}", transactions.len());
        
        // Get current balance (last transaction's balance_after)
        if let Some(last_txn) = transactions.last() {
            let current_balance = last_txn.balance_after;
            let utilization = (current_balance / card.credit_limit) * 100.0;
            println!("   Current Balance: ${:.2}", current_balance);
            println!("   Credit Utilization: {:.1}%", utilization);
            println!("   Available Credit: ${:.2}", card.credit_limit - current_balance);
        }
    }
    
    // Get expenses linked to this card
    let filters = crate::models::ExpenseFilters {
        credit_card_id: Some(id),
        ..Default::default()
    };
    if let Ok(expenses) = api.list_expenses_filtered(&filters) {
        let total: f64 = expenses.iter().map(|e| e.amount).sum();
        println!("   Linked Expenses: {} (Total: ${:.2})", expenses.len(), total);
    }
    
    Ok(())
}

fn update_card(
    api: &ApiClient,
    display: &Display,
    id: i32,
    name: Option<String>,
    limit: Option<f64>,
    billing_day: Option<i32>,
    active: Option<bool>,
    json_output: bool,
) -> Result<()> {
    let current = api.get_credit_card(id)
        .context(format!("Failed to fetch credit card {}", id))?;
    
    let card_update = CreditCardCreate {
        user_id: current.user_id,
        card_name: name.unwrap_or(current.card_name),
        last_four: current.last_four,
        credit_limit: limit.unwrap_or(current.credit_limit),
        billing_day: billing_day.unwrap_or(current.billing_day),
        tags: current.tags,
    };
    
    if active.is_some() && !json_output {
        println!("Note: Active status update not yet implemented in API");
    }
    
    let updated = api.update_credit_card(id, &card_update)
        .context(format!("Failed to update credit card {}", id))?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "id": id,
            "card": updated
        }))?);
        return Ok(());
    }
    
    println!("✓ Credit card {} updated successfully", id);
    display.show("credit_cards", &vec![updated])?;
    
    Ok(())
}

fn delete_card(api: &ApiClient, id: i32, force: bool, json_output: bool) -> Result<()> {
    if !force && !json_output {
        let card = api.get_credit_card(id)
            .context(format!("Failed to fetch credit card {}", id))?;
        
        println!("About to delete credit card:");
        println!("  ID: {}", id);
        println!("  Name: {}", card.card_name);
        println!("  Last Four: ****{}", card.last_four);
        println!("  Credit Limit: ${:.2}", card.credit_limit);
        println!("\nUse --force to skip this confirmation.");
        
        anyhow::bail!("Confirmation required. Use --force to delete.");
    }
    
    api.delete_credit_card(id)
        .context(format!("Failed to delete credit card {}", id))?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "id": id,
            "message": format!("Credit card {} deleted successfully", id)
        }))?);
        return Ok(());
    }
    
    println!("✓ Credit card {} deleted successfully", id);
    
    Ok(())
}

fn card_statement(api: &ApiClient, display: &Display, id: i32, month: String, json_output: bool) -> Result<()> {
    let statement = api.get_credit_card_statement(id, &month)
        .context(format!("Failed to fetch statement for card {}", id))?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "card_id": id,
            "month": month,
            "statement": statement
        }))?);
        return Ok(());
    }
    
    display.show("card_statement", &statement)?;
    
    Ok(())
}

fn pay_card(
    api: &ApiClient,
    _display: &Display,
    id: i32,
    amount: f64,
    from_account: Option<i32>,
    description: Option<String>,
    json_output: bool,
) -> Result<()> {
    let payment = CreditCardPayment {
        amount,
        date: Some(Local::now().format("%Y-%m-%d").to_string()),
        description,
        source_savings_account_id: from_account,
    };
    
    let transaction = api.make_credit_card_payment(id, &payment)
        .context(format!("Failed to make payment to card {}", id))?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "card_id": id,
            "amount": amount,
            "from_account": from_account,
            "new_balance": transaction.balance_after,
            "transaction": transaction
        }))?);
        return Ok(());
    }
    
    println!("✓ Payment of ${:.2} applied to card {}", amount, id);
    
    if let Some(account_id) = from_account {
        println!("   Funds withdrawn from savings account {}", account_id);
    }
    
    println!("   New Balance: ${:.2}", transaction.balance_after);
    
    Ok(())
}

fn card_transactions(
    api: &ApiClient,
    _display: &Display,
    id: i32,
    txn_type: Option<String>,
    from: Option<String>,
    to: Option<String>,
    json_output: bool,
) -> Result<()> {
    let transactions = api.get_credit_card_transactions(id, txn_type.as_deref(), from.as_deref(), to.as_deref())
        .context(format!("Failed to fetch transactions for card {}", id))?;
    
    // Calculate summary in frontend
    let charges: f64 = transactions.iter()
        .filter(|t| t.transaction_type == "charge")
        .map(|t| t.amount)
        .sum();
    let payments: f64 = transactions.iter()
        .filter(|t| t.transaction_type == "payment")
        .map(|t| t.amount)
        .sum();
    let refunds: f64 = transactions.iter()
        .filter(|t| t.transaction_type == "refund")
        .map(|t| t.amount)
        .sum();
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "card_id": id,
            "count": transactions.len(),
            "transactions": transactions,
            "summary": {
                "total_charges": charges,
                "total_payments": payments,
                "total_refunds": refunds,
                "net_change": charges - payments - refunds
            }
        }))?);
        return Ok(());
    }
    
    if transactions.is_empty() {
        println!("No transactions found for card {}", id);
        return Ok(());
    }
    
    println!("Found {} transaction(s) for card {}", transactions.len(), id);
    
    // Display transactions as table
    println!("\n{:<8} {:<12} {:>12} {:>12} {}", "ID", "Type", "Amount", "Balance", "Date");
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
    
    println!("\n📊 Summary (calculated in frontend):");
    println!("   Total Charges: ${:.2}", charges);
    println!("   Total Payments: ${:.2}", payments);
    println!("   Total Refunds: ${:.2}", refunds);
    println!("   Net Change: ${:.2}", charges - payments - refunds);
    
    Ok(())
}

fn card_utilization(api: &ApiClient, _display: &Display, _months: i32, json_output: bool) -> Result<()> {
    // Get all cards
    let cards = api.list_credit_cards(None)
        .context("Failed to fetch credit cards")?;
    
    if json_output {
        let mut utilization_data: Vec<serde_json::Value> = Vec::new();
        let mut total_balance = 0.0;
        let mut total_limit = 0.0;
        
        for card in &cards {
            if let Some(id) = card.id {
                let current_balance = if let Ok(transactions) = api.get_credit_card_transactions(id, None, None, None) {
                    transactions.last().map(|t| t.balance_after).unwrap_or(0.0)
                } else {
                    0.0
                };
                let utilization = (current_balance / card.credit_limit) * 100.0;
                let status = if utilization >= 90.0 {
                    "critical"
                } else if utilization >= 70.0 {
                    "warning"
                } else if utilization >= 30.0 {
                    "good"
                } else {
                    "low"
                };
                
                total_balance += current_balance;
                total_limit += card.credit_limit;
                
                utilization_data.push(json!({
                    "card_id": id,
                    "card_name": card.card_name,
                    "last_four": card.last_four,
                    "current_balance": current_balance,
                    "credit_limit": card.credit_limit,
                    "utilization_percent": utilization,
                    "available_credit": card.credit_limit - current_balance,
                    "status": status
                }));
            }
        }
        
        let overall_utilization = if total_limit > 0.0 { (total_balance / total_limit) * 100.0 } else { 0.0 };
        
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "count": cards.len(),
            "cards": utilization_data,
            "summary": {
                "total_balance": total_balance,
                "total_credit_limit": total_limit,
                "overall_utilization_percent": overall_utilization,
                "total_available_credit": total_limit - total_balance
            }
        }))?);
        return Ok(());
    }
    
    if cards.is_empty() {
        println!("No credit cards found.");
        return Ok(());
    }
    
    println!("📊 Credit Card Utilization Analysis (calculated in frontend)\n");
    
    for card in &cards {
        if let Some(id) = card.id {
            if let Ok(transactions) = api.get_credit_card_transactions(id, None, None, None) {
                let current_balance = transactions.last()
                    .map(|t| t.balance_after)
                    .unwrap_or(0.0);
                let utilization = (current_balance / card.credit_limit) * 100.0;
                
                let status = if utilization >= 90.0 {
                    "🔴"
                } else if utilization >= 70.0 {
                    "🟡"
                } else if utilization >= 30.0 {
                    "🟢"
                } else {
                    "⚪"
                };
                
                println!("{} {} (****{}):", status, card.card_name, card.last_four);
                println!("   Balance: ${:.2} / ${:.2} ({:.1}% utilization)",
                    current_balance, card.credit_limit, utilization);
                println!("   Available: ${:.2}", card.credit_limit - current_balance);
            }
        }
    }
    
    Ok(())
}
