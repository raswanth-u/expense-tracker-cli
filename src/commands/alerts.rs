use anyhow::Result;
use chrono::Local;
use crate::api::ApiClient;
use crate::display::Display;
use crate::ui::*;

pub fn handle_alerts(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Budget Alerts");
    
    let alert_options = vec![
        "View Current Alerts".to_string(),
        "View All Budget Status".to_string(),
        "Configure Alert Thresholds".to_string(),
        "Check Alerts for Specific Month".to_string(),
        "Back to Main Menu".to_string(),
    ];
    
    loop {
        let selection = select_with_number("Alert Options", &alert_options)?;
        
        match selection {
            0 => view_current_alerts(api, display)?,
            1 => view_all_budget_status(api, display)?,
            2 => configure_thresholds()?,
            3 => check_specific_month(api, display)?,
            4 => break,
            _ => break,
        }
    }
    
    Ok(())
}

fn view_current_alerts(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Current Budget Alerts");
    
    let current_month = Local::now().format("%Y-%m").to_string();
    
    print_info(&format!("Checking alerts for {}...", current_month));
    
    let alerts = api.get_budget_alerts(&current_month, None)?;
    
    let alert_count = alerts["alert_count"].as_u64().unwrap_or(0);
    
    if alert_count == 0 {
        println!("\n✅ [bold green]No budget alerts![/bold green]");
        println!("All your budgets are within safe limits.\n");
        return Ok(());
    }
    
    // Display alerts
    display.show("budget_alerts", &alerts)?;
    
    // Provide recommendations
    println!("\n📋 Recommendations:");
    
    // Fix: Create a binding for the empty vec to extend its lifetime
    let empty_vec = vec![];
    let alert_list = alerts["alerts"].as_array().unwrap_or(&empty_vec);
    
    for (i, alert) in alert_list.iter().enumerate() {
        let category = alert["category"].as_str().unwrap_or("Unknown");
        let status = alert["status"].as_str().unwrap_or("ok");
        let percentage = alert["percentage"].as_f64().unwrap_or(0.0);
        let remaining = alert["remaining"].as_f64().unwrap_or(0.0);
        
        println!("\n{}. {} - {}", i + 1, category, 
            if status == "exceeded" { "EXCEEDED" } else { "WARNING" });
        
        if status == "exceeded" {
            println!("   ⚠️  You've exceeded your budget by ${:.2}", remaining.abs());
            println!("   💡 Consider reviewing your spending in this category");
            println!("   💡 You may want to adjust your budget for next month");
        } else {
            println!("   ⚠️  You've used {:.1}% of your budget", percentage);
            println!("   💡 You have ${:.2} remaining", remaining);
            println!("   💡 Try to limit spending in this category for the rest of the month");
        }
    }
    
    // Ask if user wants to take action
    println!();
    if prompt_confirm("Would you like to view detailed budget status?", false)? {
        view_all_budget_status(api, display)?;
    }
    
    Ok(())
}

fn view_all_budget_status(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Budget Status - All Categories");
    
    let current_month = Local::now().format("%Y-%m").to_string();
    
    let filter_user = prompt_confirm("Filter by specific user?", false)?;
    let user_id = if filter_user {
        let users = api.list_users(Some(true))?;
        if users.is_empty() {
            print_error("No users found");
            return Ok(());
        }
        display.show("users", &users)?;
        let user_idx = select_from_list(
            &users,
            "Select User",
            |u| format!("{} ({})", u.name, u.email),
        )?;
        users[user_idx].id
    } else {
        None
    };
    
    print_info(&format!("Fetching budget status for {}...", current_month));
    let status = api.get_budget_status(&current_month, user_id)?;
    
    // Check if there are any budgets
    let budgets = status["budgets"].as_array();
    let has_budgets = budgets.map(|b| !b.is_empty()).unwrap_or(false);
    
    if !has_budgets {
        println!("\n⚠️  No budgets found for {}", current_month);
        println!("💡 Tip: Create budgets to track your spending limits");
        println!("   Use 'Budget Management' → 'Create New Budget' from the main menu\n");
        
        // Show actual spending even without budgets
        let month_start = format!("{}-01", current_month);
        let month_end = format!("{}-31", current_month);
        
        let month_filters = crate::models::ExpenseFilters {
            from_date: Some(month_start),
            to_date: Some(month_end),
            ..Default::default()
        };
        
        match api.list_expenses_filtered(&month_filters) {
            Ok(expenses) => {
                let total_spent: f64 = expenses.iter().map(|e| e.amount).sum();
                if total_spent > 0.0 {
                    println!("📊 Spending Summary for {} (without budgets):", current_month);
                    println!("   Total Spent: ${:.2}", total_spent);
                    println!("   Transactions: {}", expenses.len());
                    
                    // Group by category
                    let mut by_category: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
                    for expense in &expenses {
                        *by_category.entry(expense.category.clone()).or_insert(0.0) += expense.amount;
                    }
                    
                    println!("\n   By Category:");
                    let mut categories: Vec<_> = by_category.iter().collect();
                    categories.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
                    
                    for (category, amount) in categories {
                        println!("     • {}: ${:.2}", category, amount);
                    }
                }
            }
            Err(_) => {}
        }
        
        return Ok(());
    }
    
    display.show("budget_status", &status)?;
    
    // Show summary and insights
    let total_budget = status["total_budget"].as_f64().unwrap_or(0.0);
    let total_spent = status["total_spent"].as_f64().unwrap_or(0.0);
    let overall_pct = status["overall_percentage"].as_f64().unwrap_or(0.0);
    
    println!("\n📊 Overall Insights:");
    println!("   Budget: ${:.2}", total_budget);
    println!("   Spent: ${:.2}", total_spent);
    println!("   Usage: {:.1}%", overall_pct);
    
    if overall_pct < 50.0 {
        println!("   ✅ Great! You're well within your budget.");
    } else if overall_pct < 80.0 {
        println!("   ⚠️  You're over halfway through your budget.");
    } else if overall_pct < 100.0 {
        println!("   🚨 Warning! You're approaching your budget limit.");
    } else {
        println!("   🚨 Alert! You've exceeded your total budget.");
    }
    
    Ok(())
}

fn configure_thresholds() -> Result<()> {
    print_section_header("Configure Alert Thresholds");
    
    println!("\n⚙️  Alert Configuration");
    println!("Current thresholds:");
    println!("  - Warning: 80% of budget");
    println!("  - Critical: 100% of budget (exceeded)");
    println!();
    
    println!("💡 Note: Alert thresholds are currently hardcoded in the backend.");
    println!("To customize thresholds, you would need to:");
    println!("  1. Add threshold configuration to config.toml");
    println!("  2. Update the backend API to support custom thresholds");
    println!("  3. Pass thresholds when checking budget status");
    println!();
    
    println!("Suggested config.toml format:");
    println!("  [alerts]");
    println!("  warning_threshold = 80");
    println!("  critical_threshold = 100");
    println!("  enable_notifications = true");
    println!();
    
    if prompt_confirm("Would you like to see how to enable email alerts?", false)? {
        println!("\n📧 Email Alerts (Future Feature):");
        println!("  - Daily budget summaries");
        println!("  - Instant alerts when threshold exceeded");
        println!("  - Weekly spending reports");
        println!("  - Monthly budget reviews");
    }
    
    Ok(())
}

fn check_specific_month(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Check Alerts for Specific Month");
    
    let month = select_month_preset()?;
    
    let filter_user = prompt_confirm("Filter by specific user?", false)?;
    let user_id = if filter_user {
        let users = api.list_users(Some(true))?;
        if users.is_empty() {
            print_error("No users found");
            return Ok(());
        }
        display.show("users", &users)?;
        let user_idx = select_from_list(
            &users,
            "Select User",
            |u| format!("{} ({})", u.name, u.email),
        )?;
        users[user_idx].id
    } else {
        None
    };
    
    print_info(&format!("Checking alerts for {}...", month));
    let alerts = api.get_budget_alerts(&month, user_id)?;
    
    let alert_count = alerts["alert_count"].as_u64().unwrap_or(0);
    
    if alert_count == 0 {
        println!("\n✅ No alerts for {}", month);
    } else {
        display.show("budget_alerts", &alerts)?;
    }
    
    Ok(())
}

// Function to check alerts on startup (can be called from main)
pub fn check_alerts_on_startup(api: &ApiClient) -> Result<()> {
    let current_month = Local::now().format("%Y-%m").to_string();
    
    match api.get_budget_alerts(&current_month, None) {
        Ok(alerts) => {
            let alert_count = alerts["alert_count"].as_u64().unwrap_or(0);
            
            if alert_count > 0 {
                println!("\n🔔 [bold yellow]Budget Alert![/bold yellow]");
                println!("   You have {} budget alert(s) for {}", alert_count, current_month);
                println!("   Use './expense alerts' or select 'Budget Alerts' from menu to view details.\n");
            }
        }
        Err(_) => {
            // Silently fail - don't interrupt startup
        }
    }
    
    Ok(())
}