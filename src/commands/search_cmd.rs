//! Search command handler

use anyhow::Result;
use serde_json::json;

use crate::api::ApiClient;
use crate::cli::SearchArgs;
use crate::display::Display;
use crate::models::ExpenseFilters;

pub fn handle_search_command(api: &ApiClient, display: &Display, args: SearchArgs, json_output: bool) -> Result<()> {
    let SearchArgs { query, entity, from, to, min, max } = args;
    
    match entity.as_deref() {
        Some("expenses") | None => search_expenses(api, display, &query, from, to, min, max, json_output)?,
        Some("users") => search_users(api, display, &query, json_output)?,
        Some("budgets") => search_budgets(api, display, &query, json_output)?,
        Some("cards") => search_cards(api, display, &query, json_output)?,
        Some("accounts") => search_accounts(api, display, &query, json_output)?,
        Some("goals") => search_goals(api, display, &query, json_output)?,
        Some("assets") => search_assets(api, display, &query, json_output)?,
        Some(other) => anyhow::bail!("Unknown entity type: {}. Valid: expenses, users, budgets, cards, accounts, goals, assets", other),
    }
    
    Ok(())
}

fn search_expenses(
    api: &ApiClient,
    display: &Display,
    query: &str,
    from: Option<String>,
    to: Option<String>,
    min: Option<f64>,
    max: Option<f64>,
    json_output: bool,
) -> Result<()> {
    let filters = ExpenseFilters {
        from_date: from,
        to_date: to,
        min_amount: min,
        max_amount: max,
        ..Default::default()
    };
    
    let expenses = api.list_expenses_filtered(&filters)?;
    let query_lower = query.to_lowercase();
    
    let matched: Vec<_> = expenses.into_iter()
        .filter(|e| {
            e.category.to_lowercase().contains(&query_lower)
                || e.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower))
                || e.tags.as_ref().map_or(false, |t| t.to_lowercase().contains(&query_lower))
                || e.payment_method.to_lowercase().contains(&query_lower)
        })
        .collect();
    
    let total: f64 = matched.iter().map(|e| e.amount).sum();
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "query": query,
            "entity": "expenses",
            "count": matched.len(),
            "results": matched,
            "summary": {
                "total": total,
                "count": matched.len()
            }
        }))?);
        return Ok(());
    }
    
    if matched.is_empty() {
        println!("No expenses found matching '{}'", query);
        return Ok(());
    }
    
    println!("Found {} expense(s) matching '{}'", matched.len(), query);
    display.show("expenses", &matched)?;
    println!("\n📊 Search Results: Total ${:.2} ({} items)", total, matched.len());
    
    Ok(())
}

fn search_users(api: &ApiClient, display: &Display, query: &str, json_output: bool) -> Result<()> {
    let users = api.list_users(None)?;
    let query_lower = query.to_lowercase();
    
    let matched: Vec<_> = users.into_iter()
        .filter(|u| {
            u.name.to_lowercase().contains(&query_lower)
                || u.email.to_lowercase().contains(&query_lower)
                || u.role.to_lowercase().contains(&query_lower)
        })
        .collect();
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "query": query,
            "entity": "users",
            "count": matched.len(),
            "results": matched
        }))?);
        return Ok(());
    }
    
    if matched.is_empty() {
        println!("No users found matching '{}'", query);
        return Ok(());
    }
    
    println!("Found {} user(s) matching '{}'", matched.len(), query);
    display.show("users", &matched)?;
    Ok(())
}

fn search_budgets(api: &ApiClient, display: &Display, query: &str, json_output: bool) -> Result<()> {
    let budgets = api.list_budgets(None, None, None)?;
    let query_lower = query.to_lowercase();
    
    let matched: Vec<_> = budgets.into_iter()
        .filter(|b| {
            b.category.to_lowercase().contains(&query_lower)
                || b.month.contains(query)
                || b.tags.as_ref().map_or(false, |t| t.to_lowercase().contains(&query_lower))
        })
        .collect();
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "query": query,
            "entity": "budgets",
            "count": matched.len(),
            "results": matched
        }))?);
        return Ok(());
    }
    
    if matched.is_empty() {
        println!("No budgets found matching '{}'", query);
        return Ok(());
    }
    
    println!("Found {} budget(s) matching '{}'", matched.len(), query);
    display.show("budgets", &matched)?;
    Ok(())
}

fn search_cards(api: &ApiClient, display: &Display, query: &str, json_output: bool) -> Result<()> {
    let cards = api.list_credit_cards(None)?;
    let query_lower = query.to_lowercase();
    
    let matched: Vec<_> = cards.into_iter()
        .filter(|c| {
            c.card_name.to_lowercase().contains(&query_lower)
                || c.last_four.contains(query)
                || c.tags.as_ref().map_or(false, |t| t.to_lowercase().contains(&query_lower))
        })
        .collect();
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "query": query,
            "entity": "cards",
            "count": matched.len(),
            "results": matched
        }))?);
        return Ok(());
    }
    
    if matched.is_empty() {
        println!("No credit cards found matching '{}'", query);
        return Ok(());
    }
    
    println!("Found {} card(s) matching '{}'", matched.len(), query);
    display.show("credit_cards", &matched)?;
    Ok(())
}

fn search_accounts(api: &ApiClient, display: &Display, query: &str, json_output: bool) -> Result<()> {
    let accounts = api.list_savings_accounts(None)?;
    let query_lower = query.to_lowercase();
    
    let matched: Vec<_> = accounts.into_iter()
        .filter(|a| {
            a.account_name.to_lowercase().contains(&query_lower)
                || a.bank_name.to_lowercase().contains(&query_lower)
                || a.account_number_last_four.contains(query)
                || a.tags.as_ref().map_or(false, |t| t.to_lowercase().contains(&query_lower))
        })
        .collect();
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "query": query,
            "entity": "accounts",
            "count": matched.len(),
            "results": matched
        }))?);
        return Ok(());
    }
    
    if matched.is_empty() {
        println!("No savings accounts found matching '{}'", query);
        return Ok(());
    }
    
    println!("Found {} account(s) matching '{}'", matched.len(), query);
    display.show("savings_accounts", &matched)?;
    Ok(())
}

fn search_goals(api: &ApiClient, display: &Display, query: &str, json_output: bool) -> Result<()> {
    let goals = api.list_savings_goals(None)?;
    let query_lower = query.to_lowercase();
    
    let matched: Vec<_> = goals.into_iter()
        .filter(|g| {
            g.name.to_lowercase().contains(&query_lower)
                || g.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower))
                || g.tags.as_ref().map_or(false, |t| t.to_lowercase().contains(&query_lower))
        })
        .collect();
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "query": query,
            "entity": "goals",
            "count": matched.len(),
            "results": matched
        }))?);
        return Ok(());
    }
    
    if matched.is_empty() {
        println!("No savings goals found matching '{}'", query);
        return Ok(());
    }
    
    println!("Found {} goal(s) matching '{}'", matched.len(), query);
    display.show("savings_goals", &matched)?;
    Ok(())
}

fn search_assets(api: &ApiClient, display: &Display, query: &str, json_output: bool) -> Result<()> {
    let assets = api.list_assets(None, None)?;
    let query_lower = query.to_lowercase();
    
    let matched: Vec<_> = assets.into_iter()
        .filter(|a| {
            a.name.to_lowercase().contains(&query_lower)
                || a.asset_type.to_lowercase().contains(&query_lower)
                || a.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower))
                || a.location.as_ref().map_or(false, |l| l.to_lowercase().contains(&query_lower))
                || a.tags.as_ref().map_or(false, |t| t.to_lowercase().contains(&query_lower))
        })
        .collect();
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "query": query,
            "entity": "assets",
            "count": matched.len(),
            "results": matched
        }))?);
        return Ok(());
    }
    
    if matched.is_empty() {
        println!("No assets found matching '{}'", query);
        return Ok(());
    }
    
    println!("Found {} asset(s) matching '{}'", matched.len(), query);
    display.show("assets", &matched)?;
    Ok(())
}
