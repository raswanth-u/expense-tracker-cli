use anyhow::Result;
use chrono::{Local, Datelike};
use crate::api::ApiClient;
use crate::display::Display;
use crate::models::ExpenseFilters;
use crate::ui::*;

pub fn show_dashboard(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Dashboard - Financial Overview");
    
    print_info("Loading dashboard data...");
    
    // Get current date info
    let today = Local::now();
    let today_str = today.format("%Y-%m-%d").to_string();
    let current_month = today.format("%Y-%m").to_string();
    
    // Calculate week range (last 7 days)
    let week_ago = today - chrono::Duration::days(7);
    let week_ago_str = week_ago.format("%Y-%m-%d").to_string();
    
    // Calculate last week range (for comparison)
    let two_weeks_ago = today - chrono::Duration::days(14);
    let two_weeks_ago_str = two_weeks_ago.format("%Y-%m-%d").to_string();
    
    // Fetch all required data
    let dashboard_data = fetch_dashboard_data(
        api,
        &today_str,
        &week_ago_str,
        &two_weeks_ago_str,
        &current_month,
    )?;
    
    // Display the dashboard
    display.show("dashboard", &dashboard_data)?;
    
    Ok(())
}

fn fetch_dashboard_data(
    api: &ApiClient,
    today: &str,
    week_ago: &str,
    two_weeks_ago: &str,
    current_month: &str,
) -> Result<serde_json::Value> {
    // 1. Today's expenses
    let today_filters = ExpenseFilters {
        from_date: Some(today.to_string()),
        to_date: Some(today.to_string()),
        ..Default::default()
    };
    let today_expenses = api.list_expenses_filtered(&today_filters)?;
    let today_total: f64 = today_expenses.iter().map(|e| e.amount).sum();
    let today_count = today_expenses.len();
    
    // 2. This week's expenses
    let week_filters = ExpenseFilters {
        from_date: Some(week_ago.to_string()),
        to_date: Some(today.to_string()),
        ..Default::default()
    };
    let week_expenses = api.list_expenses_filtered(&week_filters)?;
    let week_total: f64 = week_expenses.iter().map(|e| e.amount).sum();
    
    // 3. Last week's expenses (for comparison)
    let last_week_filters = ExpenseFilters {
        from_date: Some(two_weeks_ago.to_string()),
        to_date: Some(week_ago.to_string()),
        ..Default::default()
    };
    let last_week_expenses = api.list_expenses_filtered(&last_week_filters)?;
    let last_week_total: f64 = last_week_expenses.iter().map(|e| e.amount).sum();
    
    // Calculate week comparison
    let week_change = week_total - last_week_total;
    let week_change_pct = if last_week_total > 0.0 {
        (week_change / last_week_total) * 100.0
    } else {
        0.0
    };
    
    // 4. This month's data - Get all expenses for the month
    let month_start = format!("{}-01", current_month);
    let month_end = format!("{}-31", current_month); // Simplified, works for most months
    
    let month_filters = ExpenseFilters {
        from_date: Some(month_start.clone()),
        to_date: Some(month_end.clone()),
        ..Default::default()
    };
    let month_expenses = api.list_expenses_filtered(&month_filters)?;
    let month_total_spent: f64 = month_expenses.iter().map(|e| e.amount).sum();
    
    // 5. Budget status
    let budget_status = api.get_budget_status(current_month, None)?;
    
    // Extract budget info
    let total_budget = budget_status["total_budget"].as_f64().unwrap_or(0.0);
    let alerts_count = budget_status["alerts_count"].as_u64().unwrap_or(0);
    
    // Use the actual month total spent (from expenses) instead of budget API
    // because budget API only shows spending against budgeted categories
    let actual_month_spent = month_total_spent;
    let actual_remaining = if total_budget > 0.0 {
        total_budget - actual_month_spent
    } else {
        0.0
    };
    let actual_percentage = if total_budget > 0.0 {
        (actual_month_spent / total_budget) * 100.0
    } else {
        0.0
    };
    
    // 6. Recent transactions (last 5)
    let recent_filters = ExpenseFilters {
        from_date: Some(week_ago.to_string()),
        to_date: Some(today.to_string()),
        ..Default::default()
    };
    let mut recent_expenses = api.list_expenses_filtered(&recent_filters)?;
    recent_expenses.sort_by(|a, b| b.date.cmp(&a.date)); // Sort by date descending
    let recent_transactions: Vec<_> = recent_expenses.iter().take(5).collect();
    
    // 7. Credit card summary
    let card_summary = api.get_all_cards_summary(None, Some(current_month))?;
    
    // Extract card info
    let total_cards = card_summary["total_cards"].as_u64().unwrap_or(0);
    let total_credit_limit = card_summary["total_credit_limit"].as_f64().unwrap_or(0.0);
    let total_card_spent = card_summary["total_spent"].as_f64().unwrap_or(0.0);
    let overall_utilization = card_summary["overall_utilization"].as_f64().unwrap_or(0.0);
    
    // 8. Calculate average daily spending (this month)
    let days_in_month = Local::now().day();
    let avg_daily = if days_in_month > 0 {
        actual_month_spent / days_in_month as f64
    } else {
        0.0
    };
    
    // 9. Find largest expense today
    let largest_today = today_expenses.iter()
        .map(|e| e.amount)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);
    
    // Build dashboard JSON
    let dashboard = serde_json::json!({
        "generated_at": Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        "today": {
            "date": today,
            "total": today_total,
            "count": today_count,
            "largest": largest_today,
            "transactions": today_expenses.iter().map(|e| {
                serde_json::json!({
                    "amount": e.amount,
                    "category": e.category,
                    "description": e.description,
                    "payment_method": e.payment_method,
                })
            }).collect::<Vec<_>>(),
        },
        "week": {
            "total": week_total,
            "last_week_total": last_week_total,
            "change": week_change,
            "change_percentage": week_change_pct,
        },
        "month": {
            "month": current_month,
            "total_spent": actual_month_spent,
            "total_budget": total_budget,
            "remaining": actual_remaining,
            "percentage": actual_percentage,
            "avg_daily": avg_daily,
            "days_in_month": days_in_month,
        },
        "budget_alerts": {
            "count": alerts_count,
            "budgets": budget_status["budgets"].clone(),
        },
        "credit_cards": {
            "total_cards": total_cards,
            "total_limit": total_credit_limit,
            "total_spent": total_card_spent,
            "utilization": overall_utilization,
        },
        "recent_transactions": recent_transactions.iter().map(|e| {
            serde_json::json!({
                "id": e.id,
                "date": e.date,
                "amount": e.amount,
                "category": e.category,
                "description": e.description,
                "payment_method": e.payment_method,
            })
        }).collect::<Vec<_>>(),
        "quick_stats": {
            "avg_daily_spending": avg_daily,
            "largest_expense_today": largest_today,
            "days_elapsed": days_in_month,
        }
    });
    
    Ok(dashboard)
}