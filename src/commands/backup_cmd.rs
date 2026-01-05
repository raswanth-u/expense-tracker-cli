//! Backup command handler

use anyhow::{Result, Context};
use std::fs;
use std::path::PathBuf;
use chrono::Local;
use serde_json::json;

use crate::api::ApiClient;
use crate::cli::BackupCommands;
use crate::display::Display;
use crate::models::ExpenseFilters;

pub fn handle_backup_command(api: &ApiClient, _display: &Display, cmd: BackupCommands, json_output: bool) -> Result<()> {
    match cmd {
        BackupCommands::Create { output } => create_backup(api, output, json_output)?,
        BackupCommands::Restore { file, force: _ } => restore_backup(api, &file, json_output)?,
        BackupCommands::List => list_backups(json_output)?,
    }
    Ok(())
}

fn get_backup_dir() -> PathBuf {
    // Use XDG_CONFIG_HOME or fallback to ~/.config
    std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(|h| PathBuf::from(h).join(".config"))
                .unwrap_or_else(|_| PathBuf::from("."))
        })
        .join("expense-cli")
        .join("backups")
}

fn create_backup(api: &ApiClient, output: Option<String>, json_output: bool) -> Result<()> {
    if !json_output {
        println!("📦 Creating backup...");
    }
    
    // Collect all data using correct API signatures
    let users = api.list_users(None)?;
    let expenses = api.list_expenses_filtered(&ExpenseFilters::default())?;
    let budgets = api.list_budgets(None, None, None)?;
    let credit_cards = api.list_credit_cards(None)?;
    let debit_cards = api.list_debit_cards(None)?;
    let savings_accounts = api.list_savings_accounts(None)?;
    let savings_goals = api.list_savings_goals(None)?;
    let assets = api.list_assets(None, None)?;
    let recurring = api.list_recurring_templates(None, None)?;
    
    // Create backup structure
    let backup = serde_json::json!({
        "backup_version": "1.0",
        "created_at": Local::now().to_rfc3339(),
        "data": {
            "users": users,
            "expenses": expenses,
            "budgets": budgets,
            "credit_cards": credit_cards,
            "debit_cards": debit_cards,
            "savings_accounts": savings_accounts,
            "savings_goals": savings_goals,
            "assets": assets,
            "recurring_expenses": recurring
        },
        "counts": {
            "users": users.len(),
            "expenses": expenses.len(),
            "budgets": budgets.len(),
            "credit_cards": credit_cards.len(),
            "debit_cards": debit_cards.len(),
            "savings_accounts": savings_accounts.len(),
            "savings_goals": savings_goals.len(),
            "assets": assets.len(),
            "recurring_expenses": recurring.len()
        }
    });
    
    // Determine output path
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = output.unwrap_or_else(|| format!("backup_{}.json", timestamp));
    
    // Ensure backups directory exists
    let backup_dir = get_backup_dir();
    
    fs::create_dir_all(&backup_dir)?;
    
    let filepath = if filename.contains('/') || filename.contains('\\') {
        PathBuf::from(&filename)
    } else {
        backup_dir.join(&filename)
    };
    
    // Write backup
    let json_str = serde_json::to_string_pretty(&backup)?;
    fs::write(&filepath, &json_str)?;
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "filepath": filepath.to_string_lossy(),
            "counts": {
                "users": users.len(),
                "expenses": expenses.len(),
                "budgets": budgets.len(),
                "credit_cards": credit_cards.len(),
                "debit_cards": debit_cards.len(),
                "savings_accounts": savings_accounts.len(),
                "savings_goals": savings_goals.len(),
                "assets": assets.len(),
                "recurring_expenses": recurring.len()
            }
        }))?);
        return Ok(());
    }
    
    println!("✅ Backup created: {}", filepath.display());
    println!("\n📊 Backup Summary:");
    println!("   Users: {}", users.len());
    println!("   Expenses: {}", expenses.len());
    println!("   Budgets: {}", budgets.len());
    println!("   Credit Cards: {}", credit_cards.len());
    println!("   Debit Cards: {}", debit_cards.len());
    println!("   Savings Accounts: {}", savings_accounts.len());
    println!("   Savings Goals: {}", savings_goals.len());
    println!("   Assets: {}", assets.len());
    println!("   Recurring Expenses: {}", recurring.len());
    
    Ok(())
}

fn restore_backup(api: &ApiClient, file: &str, json_output: bool) -> Result<()> {
    if !json_output {
        println!("📦 Restoring from backup: {}", file);
    }
    
    let content = fs::read_to_string(file)
        .context(format!("Failed to read backup file: {}", file))?;
    
    let backup: serde_json::Value = serde_json::from_str(&content)
        .context("Failed to parse backup file")?;
    
    // Validate backup structure
    let _data = backup.get("data")
        .context("Invalid backup: missing 'data' field")?;
    
    let version = backup.get("backup_version")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    
    let created_at = backup.get("created_at").and_then(|v| v.as_str()).unwrap_or("unknown");
    let counts = backup.get("counts").cloned().unwrap_or(json!({}));
    
    if json_output {
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "file": file,
            "version": version,
            "created_at": created_at,
            "counts": counts,
            "message": "Backup file is valid. Full restore not yet implemented."
        }))?);
        return Ok(());
    }
    
    println!("   Backup version: {}", version);
    println!("   Created: {}", created_at);
    
    // Show what will be restored
    if let Some(counts) = backup.get("counts") {
        println!("\n📊 Items to restore:");
        if let Some(c) = counts.get("users").and_then(|v| v.as_i64()) { println!("   Users: {}", c); }
        if let Some(c) = counts.get("expenses").and_then(|v| v.as_i64()) { println!("   Expenses: {}", c); }
        if let Some(c) = counts.get("budgets").and_then(|v| v.as_i64()) { println!("   Budgets: {}", c); }
        if let Some(c) = counts.get("credit_cards").and_then(|v| v.as_i64()) { println!("   Credit Cards: {}", c); }
        if let Some(c) = counts.get("debit_cards").and_then(|v| v.as_i64()) { println!("   Debit Cards: {}", c); }
        if let Some(c) = counts.get("savings_accounts").and_then(|v| v.as_i64()) { println!("   Savings Accounts: {}", c); }
        if let Some(c) = counts.get("savings_goals").and_then(|v| v.as_i64()) { println!("   Savings Goals: {}", c); }
        if let Some(c) = counts.get("assets").and_then(|v| v.as_i64()) { println!("   Assets: {}", c); }
        if let Some(c) = counts.get("recurring_expenses").and_then(|v| v.as_i64()) { println!("   Recurring: {}", c); }
    }
    
    println!("\n⚠️  Full restore not yet implemented.");
    println!("   Backup file is valid and can be used for manual restore.");
    println!("   Use the API directly to import specific items.");
    
    // Suppress unused warning for api
    let _ = api;
    
    Ok(())
}

fn list_backups(json_output: bool) -> Result<()> {
    let backup_dir = get_backup_dir();
    
    if !json_output {
        println!("📁 Backup directory: {}", backup_dir.display());
    }
    
    if !backup_dir.exists() {
        if json_output {
            println!("{}", serde_json::to_string_pretty(&json!({
                "success": true,
                "directory": backup_dir.to_string_lossy(),
                "count": 0,
                "backups": []
            }))?);
            return Ok(());
        }
        println!("   No backups found.");
        return Ok(());
    }
    
    let entries: Vec<_> = fs::read_dir(&backup_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "json"))
        .collect();
    
    if entries.is_empty() {
        if json_output {
            println!("{}", serde_json::to_string_pretty(&json!({
                "success": true,
                "directory": backup_dir.to_string_lossy(),
                "count": 0,
                "backups": []
            }))?);
            return Ok(());
        }
        println!("   No backups found.");
        return Ok(());
    }
    
    if json_output {
        let mut backups = Vec::new();
        for entry in &entries {
            let path = entry.path();
            let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            let metadata = entry.metadata().ok();
            let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
            
            let mut backup_info = json!({
                "filename": filename,
                "path": path.to_string_lossy(),
                "size": size
            });
            
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(backup) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(counts) = backup.get("counts") {
                        backup_info["counts"] = counts.clone();
                    }
                    if let Some(created) = backup.get("created_at") {
                        backup_info["created_at"] = created.clone();
                    }
                }
            }
            backups.push(backup_info);
        }
        
        println!("{}", serde_json::to_string_pretty(&json!({
            "success": true,
            "directory": backup_dir.to_string_lossy(),
            "count": backups.len(),
            "backups": backups
        }))?);
        return Ok(());
    }
    
    println!("\n📦 Available backups:\n");
    
    for entry in entries {
        let path = entry.path();
        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        let metadata = entry.metadata().ok();
        let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
        let modified = metadata
            .and_then(|m| m.modified().ok())
            .map(|t| {
                let dt: chrono::DateTime<Local> = t.into();
                dt.format("%Y-%m-%d %H:%M:%S").to_string()
            })
            .unwrap_or_else(|| "unknown".to_string());
        
        // Try to read backup metadata
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(backup) = serde_json::from_str::<serde_json::Value>(&content) {
                let counts = backup.get("counts");
                let expenses = counts.and_then(|c| c.get("expenses")).and_then(|v| v.as_i64()).unwrap_or(0);
                let users = counts.and_then(|c| c.get("users")).and_then(|v| v.as_i64()).unwrap_or(0);
                
                println!("   📄 {}", filename);
                println!("      Modified: {} | Size: {} bytes", modified, size);
                println!("      Contains: {} users, {} expenses", users, expenses);
                println!();
            } else {
                println!("   📄 {} (invalid format)", filename);
            }
        } else {
            println!("   📄 {}", filename);
            println!("      Modified: {} | Size: {} bytes", modified, size);
            println!();
        }
    }
    
    Ok(())
}
