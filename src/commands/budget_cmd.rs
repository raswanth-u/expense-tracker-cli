//! Budget command handlers for the non-interactive CLI

use anyhow::{Result, Context};
use chrono::Local;
use serde_json::json;

use crate::api::ApiClient;
use crate::cli::BudgetCommands;
use crate::display::Display;
use crate::models::BudgetCreate;

/// Handle all budget subcommands
pub fn handle_budget_command(api: &ApiClient, display: &Display, cmd: BudgetCommands, json_output: bool) -> Result<()> {
    match cmd {
        BudgetCommands::Add { category, amount, month, user_id, period, tags } => {
            add_budget(api, display, category, amount, month, user_id, period, tags, json_output)
        }
        BudgetCommands::List { user_id, month, category, active } => {
            list_budgets(api, display, user_id, month, category, active, json_output)
        }
        BudgetCommands::View { id } => view_budget(api, display, id, json_output),
        BudgetCommands::Update { id, amount, category, active } => {
            update_budget(api, display, id, amount, category, active, json_output)
        }
        BudgetCommands::Delete { id, force } => delete_budget(api, id, force, json_output),
        BudgetCommands::Status { month, user_id } => budget_status(api, display, month, user_id, json_output),
        BudgetCommands::Compare { month1, month2, user_id } => {
            compare_budgets(api, display, month1, month2, user_id, json_output)
        }
    }
}

fn add_budget(
    api: &ApiClient,
    display: &Display,
    category: String,
    amount: f64,
    month: String,
    user_id: Option<i32>,
    period: String,
    tags: Option<String>,
    json_output: bool,
) -> Result<()> {
    let budget_create = BudgetCreate {
        user_id,
        category,
        amount,
        month,
        period: Some(period),
        tags,
    };
    
    let budget = api.create_budget(&budget_create)
        .context("Failed to create budget")?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "id": budget.id.unwrap_or(0),
            "budget": {
                "id": budget.id,
                "user_id": budget.user_id,
                "category": budget.category,
                "amount": budget.amount,
                "month": budget.month,
                "period": budget.period,
                "tags": budget.tags
            }
        }))?);
        return Ok(());
    }
    
    println!("✓ Budget created with ID: {}", budget.id.unwrap_or(0));
    display.show("budgets", &vec![budget])?;
    
    Ok(())
}

fn list_budgets(
    api: &ApiClient,
    display: &Display,
    user_id: Option<i32>,
    month: Option<String>,
    category: Option<String>,
    _active: bool,
    json_output: bool,
) -> Result<()> {
    // API signature: list_budgets(month, user_id, category)
    let budgets = api.list_budgets(month.as_deref(), user_id, category.as_deref())
        .context("Failed to fetch budgets")?;
    
    let total_budget: f64 = budgets.iter().map(|b| b.amount).sum();
    
    if json_output {
        let budget_list: Vec<_> = budgets.iter().map(|b| json!({
            "id": b.id,
            "user_id": b.user_id,
            "category": b.category,
            "amount": b.amount,
            "month": b.month,
            "period": b.period,
            "tags": b.tags
        })).collect();
        
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "count": budgets.len(),
            "budgets": budget_list,
            "summary": {
                "total_budgeted": total_budget,
                "categories": budgets.len()
            }
        }))?);
        return Ok(());
    }
    
    if budgets.is_empty() {
        println!("No budgets found.");
        return Ok(());
    }
    
    println!("Found {} budget(s)", budgets.len());
    display.show("budgets", &budgets)?;
    
    // Calculate totals in frontend
    println!("\n📊 Budget Summary (calculated in frontend):");
    println!("   Total Budgeted: ${:.2}", total_budget);
    println!("   Categories: {}", budgets.len());
    
    Ok(())
}

fn view_budget(api: &ApiClient, display: &Display, id: i32, json_output: bool) -> Result<()> {
    // Get budget details
    let budget = api.get_budget(id)
        .context(format!("Failed to fetch budget {}", id))?;
    
    // Get expenses for this category and month
    let filters = crate::models::ExpenseFilters {
        user_id: budget.user_id,
        category: Some(budget.category.clone()),
        from_date: Some(format!("{}-01", budget.month)),
        to_date: Some(format!("{}-31", budget.month)),
        ..Default::default()
    };
    
    let (spent, remaining, usage_pct, status) = if let Ok(expenses) = api.list_expenses_filtered(&filters) {
        let spent: f64 = expenses.iter().map(|e| e.amount).sum();
        let remaining = budget.amount - spent;
        let usage_pct = (spent / budget.amount) * 100.0;
        let status = if usage_pct >= 100.0 {
            "over_budget"
        } else if usage_pct >= 80.0 {
            "warning"
        } else if usage_pct >= 50.0 {
            "half_used"
        } else {
            "within_budget"
        };
        (spent, remaining, usage_pct, status)
    } else {
        (0.0, budget.amount, 0.0, "unknown")
    };
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "budget": {
                "id": budget.id,
                "user_id": budget.user_id,
                "category": budget.category,
                "amount": budget.amount,
                "month": budget.month,
                "period": budget.period,
                "tags": budget.tags,
                "spent": spent,
                "remaining": remaining,
                "usage_percent": usage_pct,
                "status": status
            }
        }))?);
        return Ok(());
    }
    
    display.show("budgets", &vec![budget.clone()])?;
    
    // Calculate spending status in frontend
    println!("\n📊 Budget Status (calculated in frontend):");
    println!("   Budgeted: ${:.2}", budget.amount);
    println!("   Spent: ${:.2}", spent);
    println!("   Remaining: ${:.2}", remaining);
    println!("   Usage: {:.1}%", usage_pct);
    
    // Alert levels
    if usage_pct >= 100.0 {
        println!("   ⚠️  OVER BUDGET!");
    } else if usage_pct >= 80.0 {
        println!("   ⚠️  Warning: Near budget limit");
    } else if usage_pct >= 50.0 {
        println!("   ℹ️  Half of budget used");
    } else {
        println!("   ✓ Within budget");
    }
    
    Ok(())
}

fn update_budget(
    api: &ApiClient,
    display: &Display,
    id: i32,
    amount: Option<f64>,
    category: Option<String>,
    active: Option<bool>,
    json_output: bool,
) -> Result<()> {
    let current = api.get_budget(id)
        .context(format!("Failed to fetch budget {}", id))?;
    
    let budget_update = BudgetCreate {
        user_id: current.user_id,
        category: category.unwrap_or(current.category),
        amount: amount.unwrap_or(current.amount),
        month: current.month,
        period: current.period,
        tags: current.tags,
    };
    
    if active.is_some() && !json_output {
        println!("Note: Active status update not yet implemented in API");
    }
    
    let updated = api.update_budget(id, &budget_update)
        .context(format!("Failed to update budget {}", id))?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "id": id,
            "budget": {
                "id": updated.id,
                "user_id": updated.user_id,
                "category": updated.category,
                "amount": updated.amount,
                "month": updated.month,
                "period": updated.period,
                "tags": updated.tags
            }
        }))?);
        return Ok(());
    }
    
    println!("✓ Budget {} updated successfully", id);
    display.show("budgets", &vec![updated])?;
    
    Ok(())
}

fn delete_budget(api: &ApiClient, id: i32, force: bool, json_output: bool) -> Result<()> {
    if !force && !json_output {
        let budget = api.get_budget(id)
            .context(format!("Failed to fetch budget {}", id))?;
        
        println!("About to delete budget:");
        println!("  ID: {}", id);
        println!("  Category: {}", budget.category);
        println!("  Amount: ${:.2}", budget.amount);
        println!("  Month: {}", budget.month);
        println!("\nUse --force to skip this confirmation.");
        
        anyhow::bail!("Confirmation required. Use --force to delete.");
    }
    
    if !force && json_output {
        // For JSON output without force, return error in JSON format
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": false,
            "error": "Confirmation required. Use --force to delete.",
            "id": id
        }))?);
        return Ok(());
    }
    
    api.delete_budget(id)
        .context(format!("Failed to delete budget {}", id))?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "id": id,
            "message": format!("Budget {} deleted successfully", id)
        }))?);
        return Ok(());
    }
    
    println!("✓ Budget {} deleted successfully", id);
    
    Ok(())
}

fn budget_status(api: &ApiClient, _display: &Display, month: Option<String>, user_id: Option<i32>, json_output: bool) -> Result<()> {
    let month = month.unwrap_or_else(|| Local::now().format("%Y-%m").to_string());
    
    // Get all budgets for the month - API: list_budgets(month, user_id, category)
    let budgets = api.list_budgets(Some(&month), user_id, None)
        .context("Failed to fetch budgets")?;
    
    if budgets.is_empty() {
        if json_output {
            println!("{}", serde_json::to_string_pretty(&json!({
                "success": true,
                "month": month,
                "status": {
                    "budgets": [],
                    "total_budget": 0.0,
                    "total_spent": 0.0,
                    "total_remaining": 0.0,
                    "over_budget_count": 0,
                    "warning_count": 0
                }
            }))?);
            return Ok(());
        }
        println!("No budgets found for {}", month);
        return Ok(());
    }
    
    let mut total_budget = 0.0;
    let mut total_spent = 0.0;
    let mut over_budget_count = 0;
    let mut warning_count = 0;
    let mut budget_statuses = Vec::new();
    
    for budget in &budgets {
        // Get expenses for this category
        let filters = crate::models::ExpenseFilters {
            user_id: budget.user_id,
            category: Some(budget.category.clone()),
            from_date: Some(format!("{}-01", month)),
            to_date: Some(format!("{}-31", month)),
            ..Default::default()
        };
        
        if let Ok(expenses) = api.list_expenses_filtered(&filters) {
            let spent: f64 = expenses.iter().map(|e| e.amount).sum();
            let remaining = budget.amount - spent;
            let usage_pct = (spent / budget.amount) * 100.0;
            
            total_budget += budget.amount;
            total_spent += spent;
            
            let status = if usage_pct >= 100.0 {
                over_budget_count += 1;
                "over_budget"
            } else if usage_pct >= 80.0 {
                warning_count += 1;
                "warning"
            } else {
                "ok"
            };
            
            budget_statuses.push(json!({
                "id": budget.id,
                "category": budget.category,
                "budget_amount": budget.amount,
                "spent": spent,
                "remaining": remaining,
                "usage_percent": usage_pct,
                "status": status
            }));
            
            if !json_output {
                let status_icon = if usage_pct >= 100.0 {
                    "🔴 OVER"
                } else if usage_pct >= 80.0 {
                    "🟡 WARN"
                } else {
                    "🟢 OK"
                };
                println!("{} {} - Budget: ${:.2} | Spent: ${:.2} | Remaining: ${:.2} ({:.0}%)",
                    status_icon, budget.category, budget.amount, spent, remaining, usage_pct);
            }
        }
    }
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "month": month,
            "status": {
                "budgets": budget_statuses,
                "total_budget": total_budget,
                "total_spent": total_spent,
                "total_remaining": total_budget - total_spent,
                "over_budget_count": over_budget_count,
                "warning_count": warning_count
            }
        }))?);
        return Ok(());
    }
    
    println!("📊 Budget Status for {} (calculated in frontend)\n", month);
    println!("\n--- Summary ---");
    println!("Total Budget: ${:.2}", total_budget);
    println!("Total Spent: ${:.2}", total_spent);
    println!("Total Remaining: ${:.2}", total_budget - total_spent);
    println!("Over Budget: {}", over_budget_count);
    println!("Warnings: {}", warning_count);
    
    Ok(())
}

fn compare_budgets(
    api: &ApiClient,
    display: &Display,
    month1: String,
    month2: String,
    user_id: Option<i32>,
    json_output: bool,
) -> Result<()> {
    let comparison = api.compare_budgets(&month1, &month2, user_id)
        .context("Failed to compare budgets")?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "comparison": comparison
        }))?);
        return Ok(());
    }
    
    display.show("budget_comparison", &comparison)?;
    
    Ok(())
}
