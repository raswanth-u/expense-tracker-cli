//! Dashboard command handler

use anyhow::Result;
use chrono::Local;
use serde_json::json;

use crate::api::ApiClient;
use crate::cli::DashboardArgs;
use crate::display::Display;
use crate::models::ExpenseFilters;

pub fn handle_dashboard_command(api: &ApiClient, _display: &Display, args: DashboardArgs, json_output: bool) -> Result<()> {
    let user_id = args.user_id;
    let month = args.month.unwrap_or_else(|| Local::now().format("%Y-%m").to_string());
    
    if json_output {
        return print_dashboard_json(api, user_id, &month);
    }
    
    println!("📊 Dashboard for {}\n", month);
    
    // 1. Expense Summary
    print_expense_summary(api, user_id, &month)?;
    
    // 2. Budget Status
    print_budget_status(api, user_id, &month)?;
    
    // 3. Card Status
    print_card_status(api, user_id)?;
    
    // 4. Savings Overview
    print_savings_overview(api, user_id)?;
    
    // 5. Asset Summary
    print_asset_summary(api, user_id)?;
    
    // 6. Upcoming Recurring
    print_upcoming_recurring(api)?;
    
    // 7. Recent Activity
    print_recent_activity(api, user_id)?;
    
    Ok(())
}

fn print_dashboard_json(api: &ApiClient, user_id: Option<i32>, month: &str) -> Result<()> {
    let filters = ExpenseFilters { user_id, ..Default::default() };
    let expenses = api.list_expenses_filtered(&filters)?;
    let month_expenses: Vec<_> = expenses.iter().filter(|e| e.date.starts_with(month)).cloned().collect();
    let total_expenses: f64 = month_expenses.iter().map(|e| e.amount).sum();
    
    let budgets = api.list_budgets(Some(month), user_id, None)?;
    let cards = api.list_credit_cards(user_id)?;
    let debit_cards = api.list_debit_cards(user_id)?;
    let accounts = api.list_savings_accounts(user_id)?;
    let goals = api.list_savings_goals(user_id)?;
    let assets = api.list_assets(user_id, None)?;
    let upcoming = api.get_upcoming_recurring_expenses(7, user_id).unwrap_or_default();
    
    let total_credit_limit: f64 = cards.iter().map(|c| c.credit_limit).sum();
    let total_balance: f64 = accounts.iter().map(|a| a.current_balance).sum();
    let total_assets_current: f64 = assets.iter().map(|a| a.current_value).sum();
    let total_assets_purchase: f64 = assets.iter().map(|a| a.purchase_value).sum();
    
    println!("{}", serde_json::to_string_pretty(&json!({
        "success": true,
        "month": month,
        "expenses": {
            "total": total_expenses,
            "count": month_expenses.len(),
            "items": month_expenses
        },
        "budgets": {
            "count": budgets.len(),
            "items": budgets
        },
        "credit_cards": {
            "count": cards.len(),
            "total_limit": total_credit_limit,
            "items": cards
        },
        "debit_cards": {
            "count": debit_cards.len(),
            "items": debit_cards
        },
        "savings_accounts": {
            "count": accounts.len(),
            "total_balance": total_balance,
            "items": accounts
        },
        "savings_goals": {
            "count": goals.len(),
            "items": goals
        },
        "assets": {
            "count": assets.len(),
            "total_current_value": total_assets_current,
            "total_purchase_value": total_assets_purchase,
            "net_change": total_assets_current - total_assets_purchase,
            "items": assets
        },
        "upcoming_recurring": {
            "count": upcoming.as_array().map(|a| a.len()).unwrap_or(0),
            "items": upcoming
        }
    }))?);
    
    Ok(())
}

fn print_expense_summary(api: &ApiClient, user_id: Option<i32>, month: &str) -> Result<()> {
    let filters = ExpenseFilters {
        user_id,
        ..Default::default()
    };
    let expenses = api.list_expenses_filtered(&filters)?;
    
    // Filter expenses by month
    let month_expenses: Vec<_> = expenses.iter()
        .filter(|e| e.date.starts_with(month))
        .collect();
    
    let total: f64 = month_expenses.iter().map(|e| e.amount).sum();
    let count = month_expenses.len();
    
    // Get previous month for comparison
    let parts: Vec<&str> = month.split('-').collect();
    if let (Some(year), Some(mon)) = (parts.get(0), parts.get(1)) {
        if let (Ok(y), Ok(m)) = (year.parse::<i32>(), mon.parse::<u32>()) {
            let prev_month = if m == 1 {
                format!("{:04}-{:02}", y - 1, 12)
            } else {
                format!("{:04}-{:02}", y, m - 1)
            };
            
            let prev_expenses: Vec<_> = expenses.iter()
                .filter(|e| e.date.starts_with(&prev_month))
                .collect();
            let prev_total: f64 = prev_expenses.iter().map(|e| e.amount).sum();
            
            let change = if prev_total > 0.0 {
                ((total - prev_total) / prev_total) * 100.0
            } else {
                0.0
            };
            
            let trend = if change > 0.0 { "📈" } else if change < 0.0 { "📉" } else { "➡️" };
            
            println!("💰 Expenses");
            println!("   This month: ${:.2} ({} transactions)", total, count);
            println!("   Last month: ${:.2} ({} trend: {:.1}%)", prev_total, trend, change.abs());
        }
    }
    
    // Category breakdown
    let mut by_category: std::collections::HashMap<&str, f64> = std::collections::HashMap::new();
    for exp in &month_expenses {
        *by_category.entry(exp.category.as_str()).or_insert(0.0) += exp.amount;
    }
    
    if !by_category.is_empty() {
        let mut sorted: Vec<_> = by_category.into_iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        println!("\n   Top Categories:");
        for (cat, amt) in sorted.iter().take(5) {
            let pct = if total > 0.0 { (amt / total) * 100.0 } else { 0.0 };
            println!("     • {}: ${:.2} ({:.1}%)", cat, amt, pct);
        }
    }
    
    println!();
    Ok(())
}

fn print_budget_status(api: &ApiClient, user_id: Option<i32>, month: &str) -> Result<()> {
    // API: list_budgets(month, user_id, category)
    let budgets = api.list_budgets(Some(month), user_id, None)?;
    
    if budgets.is_empty() {
        println!("📋 Budgets: No budgets set for {}", month);
        println!();
        return Ok(());
    }
    
    // Get expenses for calculation
    let filters = ExpenseFilters {
        user_id,
        from_date: Some(format!("{}-01", month)),
        to_date: Some(format!("{}-31", month)),
        ..Default::default()
    };
    let expenses = api.list_expenses_filtered(&filters)?;
    
    println!("📋 Budget Status");
    
    for budget in &budgets {
        // Calculate spent for this category (budget.amount is the limit)
        let spent: f64 = expenses.iter()
            .filter(|e| e.category == budget.category)
            .filter(|e| user_id.map_or(true, |uid| e.user_id == uid))
            .map(|e| e.amount)
            .sum();
        
        let remaining = budget.amount - spent;
        let pct = (spent / budget.amount) * 100.0;
        
        let status = if pct >= 100.0 {
            "🔴 OVER"
        } else if pct >= 80.0 {
            "🟡 WARNING"
        } else {
            "🟢 OK"
        };
        
        println!("   {} {}: ${:.2}/${:.2} ({:.1}%) {}",
            status, budget.category, spent, budget.amount, pct,
            if remaining > 0.0 { format!("(${:.2} left)", remaining) } else { format!("(${:.2} over)", -remaining) }
        );
    }
    
    println!();
    Ok(())
}

fn print_card_status(api: &ApiClient, user_id: Option<i32>) -> Result<()> {
    let cards = api.list_credit_cards(user_id)?;
    
    if cards.is_empty() {
        println!("💳 Cards: No credit cards");
        println!();
        return Ok(());
    }
    
    println!("💳 Credit Cards");
    
    // Note: CreditCard model doesn't have current_balance, need to calculate from transactions
    // For now just show credit limits
    let mut total_limit = 0.0;
    
    for card in &cards {
        println!("   🟢 {} (****{}): Limit ${:.2}",
            card.card_name, card.last_four, card.credit_limit
        );
        total_limit += card.credit_limit;
    }
    
    println!("   ────────────────────────────────");
    println!("   Total Credit Limit: ${:.2}", total_limit);
    
    println!();
    Ok(())
}

fn print_savings_overview(api: &ApiClient, user_id: Option<i32>) -> Result<()> {
    let accounts = api.list_savings_accounts(user_id)?;
    let goals = api.list_savings_goals(user_id)?;
    
    if accounts.is_empty() && goals.is_empty() {
        println!("🏦 Savings: No accounts or goals");
        println!();
        return Ok(());
    }
    
    println!("🏦 Savings Overview");
    
    if !accounts.is_empty() {
        let total: f64 = accounts.iter().map(|a| a.current_balance).sum();
        println!("   Accounts: {} (Total: ${:.2})", accounts.len(), total);
        
        for acc in accounts.iter().take(3) {
            println!("     • {} @ {}: ${:.2}", acc.account_name, acc.bank_name, acc.current_balance);
        }
        if accounts.len() > 3 {
            println!("     ... and {} more", accounts.len() - 3);
        }
    }
    
    if !goals.is_empty() {
        println!("\n   Active Goals:");
        let active_goals: Vec<_> = goals.iter()
            .filter(|g| g.is_active.unwrap_or(true))
            .take(3)
            .collect();
        
        for goal in active_goals {
            let progress = (goal.current_amount / goal.target_amount) * 100.0;
            let bar_width = 20;
            let filled = ((progress / 100.0) * bar_width as f64) as usize;
            let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(bar_width - filled));
            
            println!("     • {}: ${:.2}/${:.2} {} {:.1}%",
                goal.name, goal.current_amount, goal.target_amount, bar, progress);
        }
    }
    
    println!();
    Ok(())
}

fn print_asset_summary(api: &ApiClient, user_id: Option<i32>) -> Result<()> {
    let assets = api.list_assets(user_id, None)?;
    
    if assets.is_empty() {
        return Ok(());
    }
    
    println!("🏠 Assets");
    
    let total_purchase: f64 = assets.iter().map(|a| a.purchase_value).sum();
    let total_current: f64 = assets.iter().map(|a| a.current_value).sum();
    let change = total_current - total_purchase;
    let pct = if total_purchase > 0.0 { (change / total_purchase) * 100.0 } else { 0.0 };
    
    let trend = if change > 0.0 { "📈" } else if change < 0.0 { "📉" } else { "➡️" };
    
    println!("   {} assets: ${:.2} → ${:.2} ({} {:.1}%)",
        assets.len(), total_purchase, total_current, trend, pct.abs());
    
    // By type
    let mut by_type: std::collections::HashMap<&str, f64> = std::collections::HashMap::new();
    for asset in &assets {
        *by_type.entry(asset.asset_type.as_str()).or_insert(0.0) += asset.current_value;
    }
    
    let mut sorted: Vec<_> = by_type.into_iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    for (typ, val) in sorted.iter().take(3) {
        println!("     • {}: ${:.2}", typ, val);
    }
    
    println!();
    Ok(())
}

fn print_upcoming_recurring(api: &ApiClient) -> Result<()> {
    let upcoming = api.get_upcoming_recurring_expenses(7, None)?;
    
    println!("🔄 Upcoming Recurring Expenses (next 7 days)");
    
    if let Some(arr) = upcoming.as_array() {
        if arr.is_empty() {
            println!("   No upcoming recurring expenses in the next 7 days");
        } else {
            for item in arr.iter().take(5) {
                let category = item.get("category").and_then(|v| v.as_str()).unwrap_or("-");
                let amount = item.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let next_date = item.get("next_occurrence").and_then(|v| v.as_str()).unwrap_or("-");
                println!("   • {} - ${:.2} - {}", category, amount, next_date);
            }
        }
    } else {
        println!("   No upcoming recurring expenses");
    }
    
    println!();
    Ok(())
}

fn print_recent_activity(api: &ApiClient, user_id: Option<i32>) -> Result<()> {
    let filters = ExpenseFilters {
        user_id,
        ..Default::default()
    };
    let expenses = api.list_expenses_filtered(&filters)?;
    
    if expenses.is_empty() {
        return Ok(());
    }
    
    println!("📝 Recent Activity (last 5 transactions)");
    
    let mut sorted = expenses.clone();
    sorted.sort_by(|a, b| b.date.cmp(&a.date));
    
    let recent: Vec<_> = sorted.into_iter().take(5).collect();
    
    for exp in &recent {
        let desc = exp.description.as_deref().unwrap_or("-");
        println!("   {} | ${:>8.2} | {:15} | {}",
            exp.date,
            exp.amount,
            exp.category,
            desc
        );
    }
    
    println!();
    Ok(())
}
