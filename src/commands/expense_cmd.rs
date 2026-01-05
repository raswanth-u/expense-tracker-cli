//! Expense command handlers for the non-interactive CLI

use anyhow::{Result, Context};
use chrono::Local;
use serde_json::json;

use crate::api::ApiClient;
use crate::cli::ExpenseCommands;
use crate::display::Display;
use crate::models::{ExpenseCreate, ExpenseFilters};

/// Handle all expense subcommands
pub fn handle_expense_command(api: &ApiClient, display: &Display, cmd: ExpenseCommands, json_output: bool) -> Result<()> {
    match cmd {
        ExpenseCommands::Add {
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
        } => add_expense(api, display, amount, category, user_id, description, date, payment, card_id, debit_id, account_id, tags, recurring, json_output),
        
        ExpenseCommands::List {
            user_id,
            category,
            payment,
            min,
            max,
            from,
            to,
            tags,
            limit,
        } => list_expenses(api, display, user_id, category, payment, min, max, from, to, tags, limit, json_output),
        
        ExpenseCommands::View { id } => view_expense(api, display, id, json_output),
        
        ExpenseCommands::Update {
            id,
            amount,
            category,
            description,
            date,
            payment,
            tags,
        } => update_expense(api, display, id, amount, category, description, date, payment, tags, json_output),
        
        ExpenseCommands::Delete { id, force } => delete_expense(api, display, id, force, json_output),
        
        ExpenseCommands::Summary { user_id, month } => expense_summary(api, display, user_id, month, json_output),
    }
}

fn add_expense(
    api: &ApiClient,
    display: &Display,
    amount: f64,
    category: String,
    user_id: i32,
    description: Option<String>,
    date: Option<String>,
    payment: String,
    card_id: Option<i32>,
    debit_id: Option<i32>,
    account_id: Option<i32>,
    tags: Option<String>,
    recurring: bool,
    json_output: bool,
) -> Result<()> {
    // Default date to today if not provided
    let date = date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());
    
    // Determine credit_card_id and savings_account_id based on payment method
    let mut credit_card_id = card_id;
    let mut savings_account_id = account_id;
    
    // If debit card is used, get the linked savings account and convert to savings_account payment
    // The API treats debit_card as a savings_account withdrawal
    let mut effective_payment = payment.clone();
    if payment == "debit_card" {
        if let Some(debit_id) = debit_id {
            // Get debit card details to find linked account
            let debit_card = api.get_debit_card(debit_id)?;
            savings_account_id = Some(debit_card.savings_account_id);
            // API requires payment_method to be "savings_account" when savings_account_id is set
            effective_payment = "savings_account".to_string();
            if !json_output {
                println!("✓ Using debit card linked to savings account ID: {}", debit_card.savings_account_id);
            }
        } else {
            anyhow::bail!("Debit card ID is required when payment method is debit_card");
        }
    }
    
    // Validate payment method requirements
    match payment.as_str() {
        "credit_card" if credit_card_id.is_none() => {
            anyhow::bail!("Credit card ID (--card-id) is required when payment method is credit_card");
        }
        "savings_account" if savings_account_id.is_none() => {
            anyhow::bail!("Savings account ID (--account-id) is required when payment method is savings_account");
        }
        _ => {}
    }
    
    let expense_create = ExpenseCreate {
        user_id,
        amount,
        category,
        description,
        date,
        payment_method: effective_payment,
        credit_card_id,
        is_recurring: Some(recurring),
        tags,
        savings_account_id,
    };
    
    let expense = api.create_expense(&expense_create)
        .context("Failed to create expense")?;
    
    if json_output {
        let details = api.get_expense_details(expense.id.unwrap_or(0))?;
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "id": expense.id,
            "expense": details
        }))?);
        return Ok(());
    }
    
    println!("✓ Expense created with ID: {}", expense.id.unwrap_or(0));
    
    // Show the created expense with full details
    let details = api.get_expense_details(expense.id.unwrap_or(0))?;
    display.show("expense_details", &details)?;
    
    Ok(())
}

fn list_expenses(
    api: &ApiClient,
    display: &Display,
    user_id: Option<i32>,
    category: Option<String>,
    payment: Option<String>,
    min: Option<f64>,
    max: Option<f64>,
    from: Option<String>,
    to: Option<String>,
    tags: Option<String>,
    limit: Option<i32>,
    json_output: bool,
) -> Result<()> {
    let filters = ExpenseFilters {
        user_id,
        category,
        payment_method: payment,
        credit_card_id: None,
        min_amount: min,
        max_amount: max,
        from_date: from,
        to_date: to,
        is_recurring: None,
        tags,
    };
    
    let mut expenses = api.list_expenses_filtered(&filters)
        .context("Failed to fetch expenses")?;
    
    // Apply limit if specified
    if let Some(lim) = limit {
        expenses.truncate(lim as usize);
    }
    
    // Calculate summary in frontend
    let total: f64 = expenses.iter().map(|e| e.amount).sum();
    let avg = if !expenses.is_empty() { total / expenses.len() as f64 } else { 0.0 };
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "count": expenses.len(),
            "expenses": expenses,
            "summary": {
                "total": total,
                "average": avg,
                "count": expenses.len()
            }
        }))?);
        return Ok(());
    }
    
    if expenses.is_empty() {
        println!("No expenses found matching the criteria.");
        return Ok(());
    }
    
    println!("Found {} expense(s)", expenses.len());
    display.show("expenses", &expenses)?;
    
    println!("\n📊 Summary (calculated in frontend):");
    println!("   Total: ${:.2}", total);
    println!("   Average: ${:.2}", avg);
    println!("   Count: {}", expenses.len());
    
    Ok(())
}

fn view_expense(api: &ApiClient, display: &Display, id: i32, json_output: bool) -> Result<()> {
    let details = api.get_expense_details(id)
        .context(format!("Failed to fetch expense {}", id))?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "expense": details
        }))?);
        return Ok(());
    }
    
    display.show("expense_details", &details)?;
    
    Ok(())
}

fn update_expense(
    api: &ApiClient,
    display: &Display,
    id: i32,
    amount: Option<f64>,
    category: Option<String>,
    description: Option<String>,
    date: Option<String>,
    payment: Option<String>,
    tags: Option<String>,
    json_output: bool,
) -> Result<()> {
    // First get the current expense
    let current = api.get_expense(id)
        .context(format!("Failed to fetch expense {}", id))?;
    
    // Build update with new values or keep existing
    let expense_update = ExpenseCreate {
        user_id: current.user_id,
        amount: amount.unwrap_or(current.amount),
        category: category.unwrap_or(current.category),
        description: description.or(current.description),
        date: date.unwrap_or(current.date),
        payment_method: payment.unwrap_or(current.payment_method),
        credit_card_id: current.credit_card_id,
        is_recurring: current.is_recurring,
        tags: tags.or(current.tags),
        savings_account_id: current.savings_account_id,
    };
    
    let updated = api.update_expense(id, &expense_update)
        .context(format!("Failed to update expense {}", id))?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "id": id,
            "expense": updated
        }))?);
        return Ok(());
    }
    
    println!("✓ Expense {} updated successfully", id);
    display.show("expenses", &vec![updated])?;
    
    Ok(())
}

fn delete_expense(api: &ApiClient, _display: &Display, id: i32, force: bool, json_output: bool) -> Result<()> {
    if !force && !json_output {
        // Show expense before deleting
        let expense = api.get_expense(id)
            .context(format!("Failed to fetch expense {}", id))?;
        
        println!("About to delete expense:");
        println!("  ID: {}", id);
        println!("  Amount: ${:.2}", expense.amount);
        println!("  Category: {}", expense.category);
        println!("  Date: {}", expense.date);
        println!("\nUse --force to skip this confirmation.");
        
        // In non-interactive mode, require --force
        anyhow::bail!("Confirmation required. Use --force to delete.");
    }
    
    api.delete_expense(id)
        .context(format!("Failed to delete expense {}", id))?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "id": id,
            "message": format!("Expense {} deleted successfully", id)
        }))?);
        return Ok(());
    }
    
    println!("✓ Expense {} deleted successfully", id);
    
    Ok(())
}

fn expense_summary(
    api: &ApiClient,
    display: &Display,
    user_id: Option<i32>,
    month: Option<String>,
    json_output: bool,
) -> Result<()> {
    let month = month.unwrap_or_else(|| Local::now().format("%Y-%m").to_string());
    
    // API signature: get_expense_summary(from_date, to_date, user_id)
    let from_date = format!("{}-01", month);
    let to_date = format!("{}-31", month);
    
    let summary = api.get_expense_summary(Some(&from_date), Some(&to_date), user_id)
        .context("Failed to fetch expense summary")?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "month": month,
            "summary": summary
        }))?);
        return Ok(());
    }
    
    display.show("expense_summary", &summary)?;
    
    Ok(())
}
