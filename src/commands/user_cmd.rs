//! User command handlers for the non-interactive CLI

use anyhow::{Result, Context};
use chrono::Local;
use serde_json::json;

use crate::api::ApiClient;
use crate::cli::UserCommands;
use crate::display::Display;
use crate::models::UserCreate;

/// Handle all user subcommands
pub fn handle_user_command(api: &ApiClient, display: &Display, cmd: UserCommands, json_output: bool) -> Result<()> {
    match cmd {
        UserCommands::Add { name, email, role } => add_user(api, display, name, email, role, json_output),
        UserCommands::List { active } => list_users(api, display, active, json_output),
        UserCommands::View { id } => view_user(api, display, id, json_output),
        UserCommands::Update { id, name, email, role, active } => update_user(api, display, id, name, email, role, active, json_output),
        UserCommands::Delete { id, force } => delete_user(api, id, force, json_output),
        UserCommands::Stats { id, month } => user_stats(api, display, id, month, json_output),
    }
}

fn add_user(api: &ApiClient, display: &Display, name: String, email: String, role: String, json_output: bool) -> Result<()> {
    let user_create = UserCreate { name, email, role };
    
    let user = api.create_user(&user_create)
        .context("Failed to create user")?;
    
    if json_output {
        let user_id = user.id.unwrap_or(0);
        println!("{}", json!({
            "success": true,
            "id": user_id,
            "user": user
        }));
        return Ok(());
    }
    
    println!("✓ User created with ID: {}", user.id.unwrap_or(0));
    display.show("users", &vec![user])?;
    
    Ok(())
}

fn list_users(api: &ApiClient, display: &Display, active: bool, json_output: bool) -> Result<()> {
    let active_filter = if active { Some(true) } else { None };
    
    let users = api.list_users(active_filter)
        .context("Failed to fetch users")?;
    
    if json_output {
        println!("{}", json!({
            "success": true,
            "data": users
        }));
        return Ok(());
    }
    
    if users.is_empty() {
        println!("No users found.");
        return Ok(());
    }
    
    println!("Found {} user(s)", users.len());
    display.show("users", &users)?;
    
    Ok(())
}

fn view_user(api: &ApiClient, display: &Display, id: i32, json_output: bool) -> Result<()> {
    // Get user details
    let user = api.get_user(id)
        .context(format!("Failed to fetch user {}", id))?;
    
    if json_output {
        // Collect all stats for JSON output
        let mut stats = json!({
            "total_expenses": 0.0,
            "expense_count": 0,
            "active_budgets": 0,
            "credit_cards": 0,
            "total_credit_limit": 0.0,
            "savings_accounts": 0,
            "total_savings_balance": 0.0,
            "assets": 0,
            "total_asset_value": 0.0,
            "asset_net_change": 0.0,
            "savings_goals": 0,
            "savings_goals_progress": 0.0
        });
        
        let filters = crate::models::ExpenseFilters {
            user_id: Some(id),
            ..Default::default()
        };
        if let Ok(expenses) = api.list_expenses_filtered(&filters) {
            let total: f64 = expenses.iter().map(|e| e.amount).sum();
            stats["total_expenses"] = json!(total);
            stats["expense_count"] = json!(expenses.len());
        }
        
        if let Ok(budgets) = api.list_budgets(None, Some(id), None) {
            stats["active_budgets"] = json!(budgets.len());
        }
        
        if let Ok(cards) = api.list_credit_cards(Some(id)) {
            let total_limit: f64 = cards.iter().map(|c| c.credit_limit).sum();
            stats["credit_cards"] = json!(cards.len());
            stats["total_credit_limit"] = json!(total_limit);
        }
        
        if let Ok(accounts) = api.list_savings_accounts(Some(id)) {
            let total_balance: f64 = accounts.iter().map(|a| a.current_balance).sum();
            stats["savings_accounts"] = json!(accounts.len());
            stats["total_savings_balance"] = json!(total_balance);
        }
        
        if let Ok(assets) = api.list_assets(Some(id), None) {
            let total_value: f64 = assets.iter().map(|a| a.current_value).sum();
            let total_purchase: f64 = assets.iter().map(|a| a.purchase_value).sum();
            stats["assets"] = json!(assets.len());
            stats["total_asset_value"] = json!(total_value);
            stats["asset_net_change"] = json!(total_value - total_purchase);
        }
        
        if let Ok(goals) = api.list_savings_goals(Some(id)) {
            let total_target: f64 = goals.iter().map(|g| g.target_amount).sum();
            let total_current: f64 = goals.iter().map(|g| g.current_amount).sum();
            let progress = if total_target > 0.0 { (total_current / total_target) * 100.0 } else { 0.0 };
            stats["savings_goals"] = json!(goals.len());
            stats["savings_goals_progress"] = json!(progress);
        }
        
        println!("{}", json!({
            "success": true,
            "data": user,
            "stats": stats
        }));
        return Ok(());
    }
    
    display.show("users", &vec![user])?;
    
    // Get related data and calculate stats in frontend
    println!("\n📊 User Statistics:");
    
    // Get user's expenses
    let filters = crate::models::ExpenseFilters {
        user_id: Some(id),
        ..Default::default()
    };
    if let Ok(expenses) = api.list_expenses_filtered(&filters) {
        let total: f64 = expenses.iter().map(|e| e.amount).sum();
        println!("   Total Expenses: ${:.2} ({} transactions)", total, expenses.len());
    }
    
    // Get user's budgets (month, user_id, category)
    if let Ok(budgets) = api.list_budgets(None, Some(id), None) {
        println!("   Active Budgets: {}", budgets.len());
    }
    
    // Get user's credit cards
    if let Ok(cards) = api.list_credit_cards(Some(id)) {
        let total_limit: f64 = cards.iter().map(|c| c.credit_limit).sum();
        println!("   Credit Cards: {} (Total Limit: ${:.2})", cards.len(), total_limit);
    }
    
    // Get user's savings accounts
    if let Ok(accounts) = api.list_savings_accounts(Some(id)) {
        let total_balance: f64 = accounts.iter().map(|a| a.current_balance).sum();
        println!("   Savings Accounts: {} (Total Balance: ${:.2})", accounts.len(), total_balance);
    }
    
    // Get user's assets (user_id, asset_type)
    if let Ok(assets) = api.list_assets(Some(id), None) {
        let total_value: f64 = assets.iter().map(|a| a.current_value).sum();
        let total_purchase: f64 = assets.iter().map(|a| a.purchase_value).sum();
        let net_change = total_value - total_purchase;
        let change_str = if net_change >= 0.0 {
            format!("+${:.2}", net_change)
        } else {
            format!("-${:.2}", net_change.abs())
        };
        println!("   Assets: {} (Current Value: ${:.2}, Change: {})", assets.len(), total_value, change_str);
    }
    
    // Get user's savings goals (user_id only)
    if let Ok(goals) = api.list_savings_goals(Some(id)) {
        let total_target: f64 = goals.iter().map(|g| g.target_amount).sum();
        let total_current: f64 = goals.iter().map(|g| g.current_amount).sum();
        let progress = if total_target > 0.0 { (total_current / total_target) * 100.0 } else { 0.0 };
        println!("   Savings Goals: {} ({:.1}% overall progress)", goals.len(), progress);
    }
    
    Ok(())
}

fn update_user(
    api: &ApiClient,
    display: &Display,
    id: i32,
    name: Option<String>,
    email: Option<String>,
    role: Option<String>,
    active: Option<bool>,
    json_output: bool,
) -> Result<()> {
    // Get current user
    let current = api.get_user(id)
        .context(format!("Failed to fetch user {}", id))?;
    
    // Build update
    let user_update = UserCreate {
        name: name.unwrap_or(current.name),
        email: email.unwrap_or(current.email),
        role: role.unwrap_or(current.role),
    };
    
    // Note: active status update would need a separate endpoint or field
    if active.is_some() && !json_output {
        println!("Note: Active status update not yet implemented in API");
    }
    
    let updated = api.update_user(id, &user_update)
        .context(format!("Failed to update user {}", id))?;
    
    if json_output {
        println!("{}", json!({
            "success": true,
            "id": id,
            "user": updated
        }));
        return Ok(());
    }
    
    println!("✓ User {} updated successfully", id);
    display.show("users", &vec![updated])?;
    
    Ok(())
}

fn delete_user(api: &ApiClient, id: i32, force: bool, json_output: bool) -> Result<()> {
    if !force {
        let user = api.get_user(id)
            .context(format!("Failed to fetch user {}", id))?;
        
        if json_output {
            println!("{}", json!({
                "success": false,
                "id": id,
                "message": "Confirmation required. Use --force to delete.",
                "user": user
            }));
            return Ok(());
        }
        
        println!("About to delete user:");
        println!("  ID: {}", id);
        println!("  Name: {}", user.name);
        println!("  Email: {}", user.email);
        println!("\nUse --force to skip this confirmation.");
        
        anyhow::bail!("Confirmation required. Use --force to delete.");
    }
    
    api.delete_user(id)
        .context(format!("Failed to delete user {}", id))?;
    
    if json_output {
        println!("{}", json!({
            "success": true,
            "id": id,
            "message": format!("User {} deleted successfully", id)
        }));
        return Ok(());
    }
    
    println!("✓ User {} deleted successfully", id);
    
    Ok(())
}

fn user_stats(api: &ApiClient, display: &Display, id: i32, month: Option<String>, json_output: bool) -> Result<()> {
    let month = month.unwrap_or_else(|| Local::now().format("%Y-%m").to_string());
    
    let stats = api.get_user_stats(id, Some(&month))
        .context(format!("Failed to fetch stats for user {}", id))?;
    
    if json_output {
        println!("{}", json!({
            "success": true,
            "data": stats
        }));
        return Ok(());
    }
    
    display.show("user_stats", &stats)?;
    
    Ok(())
}
