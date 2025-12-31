use anyhow::Result;
use chrono::Local;
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::Path;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use crate::api::ApiClient;
use crate::display::Display;
use crate::ui::*;

pub fn handle_backup(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Backup & Restore");
    
    let backup_options = vec![
        "Create Backup".to_string(),
        "Restore from Backup".to_string(),
        "List Available Backups".to_string(),
        "Delete Old Backups".to_string(),
        "Back to Main Menu".to_string(),
    ];
    
    loop {
        let selection = select_with_number("Backup Options", &backup_options)?;
        
        match selection {
            0 => create_backup(api, display)?,
            1 => restore_backup(api, display)?,
            2 => list_backups()?,
            3 => delete_backups()?,
            4 => break,
            _ => break,
        }
    }
    
    Ok(())
}

fn create_backup(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Create Backup");
    
    // Ask for backup options
    let include_users = prompt_confirm("Include users?", true)?;
    let include_expenses = prompt_confirm("Include expenses?", true)?;
    let include_budgets = prompt_confirm("Include budgets?", true)?;
    let include_cards = prompt_confirm("Include credit cards?", true)?;
    let include_savings = prompt_confirm("Include savings goals?", true)?;
    let include_assets = prompt_confirm("Include assets?", true)?;
    let include_recurring = prompt_confirm("Include recurring templates?", true)?;
    
    let compress = prompt_confirm("Compress backup?", true)?;
    
    // Generate filename
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let default_filename = format!("backup_{}.json", timestamp);
    
    let filename = prompt_string(
        "Backup filename",
        Some(&default_filename),
        false,
    )?;
    
    print_info("Creating backup...");
    
    // Fetch all data
    let mut backup_data = serde_json::json!({
        "metadata": {
            "created_at": Local::now().to_rfc3339(),
            "version": "2.0.0",
            "cli_version": env!("CARGO_PKG_VERSION"),
        }
    });
    
    let mut record_counts = std::collections::HashMap::new();
    
    // Fetch users
    if include_users {
        print_info("Backing up users...");
        match api.list_users(None) {
            Ok(users) => {
                record_counts.insert("users".to_string(), users.len());
                backup_data["users"] = serde_json::to_value(&users)?;
            }
            Err(e) => print_error(&format!("Failed to fetch users: {}", e)),
        }
    }
    
    // Fetch expenses
    if include_expenses {
        print_info("Backing up expenses...");
        match api.list_expenses_filtered(&crate::models::ExpenseFilters::default()) {
            Ok(expenses) => {
                record_counts.insert("expenses".to_string(), expenses.len());
                backup_data["expenses"] = serde_json::to_value(&expenses)?;
            }
            Err(e) => print_error(&format!("Failed to fetch expenses: {}", e)),
        }
    }
    
    // Fetch budgets
    if include_budgets {
        print_info("Backing up budgets...");
        match api.list_budgets(None, None, None) {
            Ok(budgets) => {
                record_counts.insert("budgets".to_string(), budgets.len());
                backup_data["budgets"] = serde_json::to_value(&budgets)?;
            }
            Err(e) => print_error(&format!("Failed to fetch budgets: {}", e)),
        }
    }
    
    // Fetch credit cards
    if include_cards {
        print_info("Backing up credit cards...");
        match api.list_credit_cards(None) {
            Ok(cards) => {
                record_counts.insert("credit_cards".to_string(), cards.len());
                backup_data["credit_cards"] = serde_json::to_value(&cards)?;
            }
            Err(e) => print_error(&format!("Failed to fetch credit cards: {}", e)),
        }
    }
    
    // Fetch savings goals
    if include_savings {
        print_info("Backing up savings goals...");
        match api.list_savings_goals(None) {
            Ok(goals) => {
                record_counts.insert("savings_goals".to_string(), goals.len());
                backup_data["savings_goals"] = serde_json::to_value(&goals)?;
            }
            Err(e) => print_error(&format!("Failed to fetch savings goals: {}", e)),
        }
    }
    
    // Fetch assets
    if include_assets {
        print_info("Backing up assets...");
        match api.list_assets(None, None) {
            Ok(assets) => {
                record_counts.insert("assets".to_string(), assets.len());
                backup_data["assets"] = serde_json::to_value(&assets)?;
            }
            Err(e) => print_error(&format!("Failed to fetch assets: {}", e)),
        }
    }
    
    // Fetch recurring templates
    if include_recurring {
        print_info("Backing up recurring templates...");
        match api.list_recurring_templates(None, None) {
            Ok(templates) => {
                record_counts.insert("recurring_templates".to_string(), templates.len());
                backup_data["recurring_templates"] = serde_json::to_value(&templates)?;
            }
            Err(e) => print_error(&format!("Failed to fetch recurring templates: {}", e)),
        }
    }
    
    // Add record counts to metadata
    backup_data["metadata"]["record_counts"] = serde_json::to_value(&record_counts)?;
    
    // Serialize to JSON
    let json_data = serde_json::to_string_pretty(&backup_data)?;
    
    // Create backups directory if it doesn't exist
    fs::create_dir_all("backups")?;
    
    let backup_path = Path::new("backups").join(&filename);
    
    // Write to file (compressed or uncompressed)
    if compress {
        let compressed_filename = if filename.ends_with(".json") {
            filename.replace(".json", ".json.gz")
        } else {
            format!("{}.gz", filename)
        };
        
        let compressed_path = Path::new("backups").join(&compressed_filename);
        let file = File::create(&compressed_path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(json_data.as_bytes())?;
        encoder.finish()?;
        
        let file_size = fs::metadata(&compressed_path)?.len();
        
        print_success(&format!("Backup created: backups/{}", compressed_filename));
        println!("  Size: {} bytes (compressed)", file_size);
    } else {
        fs::write(&backup_path, json_data)?;
        let file_size = fs::metadata(&backup_path)?.len();
        
        print_success(&format!("Backup created: backups/{}", filename));
        println!("  Size: {} bytes", file_size);
    }
    
    // Display summary
    println!("\n📊 Backup Summary:");
    let mut sorted_counts: Vec<_> = record_counts.iter().collect();
    sorted_counts.sort_by_key(|(k, _)| k.as_str());
    
    for (entity, count) in sorted_counts {
        println!("  • {}: {} records", entity, count);
    }
    
    Ok(())
}

fn restore_backup(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Restore from Backup");
    
    // List available backups
    let backups = get_available_backups()?;
    
    if backups.is_empty() {
        print_error("No backups found in 'backups/' directory");
        return Ok(());
    }
    
    println!("\nAvailable Backups:");
    for (i, backup) in backups.iter().enumerate() {
        let backup_path = Path::new("backups").join(backup);
        let metadata = fs::metadata(&backup_path)?;
        let size = metadata.len();
        let modified = metadata.modified()?;
        let modified_time: chrono::DateTime<Local> = modified.into();
        
        println!("  {}. {}", i + 1, backup);
        println!("     Size: {} bytes | Modified: {}", size, modified_time.format("%Y-%m-%d %H:%M:%S"));
    }
    
    let backup_idx = select_from_list(
        &backups,
        "\nSelect Backup to Restore",
        |b| b.clone(),
    )?;
    
    let backup_file = &backups[backup_idx];
    let backup_path = Path::new("backups").join(backup_file);
    
    // Read backup file
    print_info(&format!("Reading backup: {}...", backup_file));
    
    let json_data = if backup_file.ends_with(".gz") {
        // Decompress
        let file = File::open(&backup_path)?;
        let mut decoder = GzDecoder::new(file);
        let mut json_string = String::new();
        decoder.read_to_string(&mut json_string)?;
        json_string
    } else {
        fs::read_to_string(&backup_path)?
    };
    
    let backup_data: serde_json::Value = serde_json::from_str(&json_data)?;
    
    // Show backup metadata
    if let Some(metadata) = backup_data.get("metadata") {
        println!("\n📋 Backup Information:");
        if let Some(created) = metadata.get("created_at") {
            println!("  Created: {}", created.as_str().unwrap_or("Unknown"));
        }
        if let Some(version) = metadata.get("version") {
            println!("  Version: {}", version.as_str().unwrap_or("Unknown"));
        }
        if let Some(cli_version) = metadata.get("cli_version") {
            println!("  CLI Version: {}", cli_version.as_str().unwrap_or("Unknown"));
        }
        if let Some(counts) = metadata.get("record_counts") {
            println!("\n  📊 Records in Backup:");
            if let Some(obj) = counts.as_object() {
                for (key, value) in obj {
                    println!("    • {}: {}", key, value.as_u64().unwrap_or(0));
                }
            }
        }
    }
    
    println!("\n⚠️  WARNING: This will create new records in the database.");
    println!("⚠️  Existing records will NOT be deleted or modified.");
    println!("⚠️  Duplicate records may be created if data already exists.\n");
    
    if !prompt_confirm("Continue with restore?", false)? {
        print_info("Restore cancelled");
        return Ok(());
    }
    
    // Ask what to restore
    let restore_users = backup_data.get("users").is_some() && 
        prompt_confirm("Restore users?", true)?;
    let restore_expenses = backup_data.get("expenses").is_some() && 
        prompt_confirm("Restore expenses?", true)?;
    let restore_budgets = backup_data.get("budgets").is_some() && 
        prompt_confirm("Restore budgets?", true)?;
    let restore_cards = backup_data.get("credit_cards").is_some() && 
        prompt_confirm("Restore credit cards?", true)?;
    let restore_savings = backup_data.get("savings_goals").is_some() && 
        prompt_confirm("Restore savings goals?", true)?;
    let restore_assets = backup_data.get("assets").is_some() && 
        prompt_confirm("Restore assets?", true)?;
    let restore_recurring = backup_data.get("recurring_templates").is_some() && 
        prompt_confirm("Restore recurring templates?", true)?;
    
    let mut restore_summary = std::collections::HashMap::new();
    
    // Restore users
    if restore_users {
        if let Some(users) = backup_data.get("users") {
            print_info("Restoring users...");
            let users: Vec<crate::models::User> = serde_json::from_value(users.clone())?;
            let mut success_count = 0;
            let mut error_count = 0;
            
            for user in users {
                let user_create = crate::models::UserCreate {
                    name: user.name,
                    email: user.email,
                    role: user.role,
                };
                
                match api.create_user(&user_create) {
                    Ok(_) => {
                        success_count += 1;
                        print!(".");
                    }
                    Err(_) => {
                        error_count += 1;
                        print!("x");
                    }
                }
            }
            println!();
            
            restore_summary.insert("users".to_string(), (success_count, error_count));
        }
    }
    
    // Restore expenses
    if restore_expenses {
        if let Some(expenses) = backup_data.get("expenses") {
            print_info("Restoring expenses...");
            let expenses: Vec<crate::models::Expense> = serde_json::from_value(expenses.clone())?;
            let mut success_count = 0;
            let mut error_count = 0;
            
            for expense in expenses {
                let expense_create = crate::models::ExpenseCreate {
                    user_id: expense.user_id,
                    amount: expense.amount,
                    category: expense.category,
                    description: expense.description,
                    date: expense.date,
                    payment_method: expense.payment_method,
                    credit_card_id: expense.credit_card_id,
                    is_recurring: expense.is_recurring,
                    tags: expense.tags,
                };
                
                match api.create_expense(&expense_create) {
                    Ok(_) => {
                        success_count += 1;
                        print!(".");
                    }
                    Err(_) => {
                        error_count += 1;
                        print!("x");
                    }
                }
            }
            println!();
            
            restore_summary.insert("expenses".to_string(), (success_count, error_count));
        }
    }
    
    // Restore budgets
    if restore_budgets {
        if let Some(budgets) = backup_data.get("budgets") {
            print_info("Restoring budgets...");
            let budgets: Vec<crate::models::Budget> = serde_json::from_value(budgets.clone())?;
            let mut success_count = 0;
            let mut error_count = 0;
            
            for budget in budgets {
                let budget_create = crate::models::BudgetCreate {
                    user_id: budget.user_id,
                    category: budget.category,
                    amount: budget.amount,
                    month: budget.month,
                    period: budget.period,
                };
                
                match api.create_budget(&budget_create) {
                    Ok(_) => {
                        success_count += 1;
                        print!(".");
                    }
                    Err(_) => {
                        error_count += 1;
                        print!("x");
                    }
                }
            }
            println!();
            
            restore_summary.insert("budgets".to_string(), (success_count, error_count));
        }
    }
    
    // Restore credit cards
    if restore_cards {
        if let Some(cards) = backup_data.get("credit_cards") {
            print_info("Restoring credit cards...");
            let cards: Vec<crate::models::CreditCard> = serde_json::from_value(cards.clone())?;
            let mut success_count = 0;
            let mut error_count = 0;
            
            for card in cards {
                let card_create = crate::models::CreditCardCreate {
                    user_id: card.user_id,
                    card_name: card.card_name,
                    last_four: card.last_four,
                    credit_limit: card.credit_limit,
                    billing_day: card.billing_day,
                };
                
                match api.create_credit_card(&card_create) {
                    Ok(_) => {
                        success_count += 1;
                        print!(".");
                    }
                    Err(_) => {
                        error_count += 1;
                        print!("x");
                    }
                }
            }
            println!();
            
            restore_summary.insert("credit_cards".to_string(), (success_count, error_count));
        }
    }
    
    // Restore savings goals
    if restore_savings {
        if let Some(goals) = backup_data.get("savings_goals") {
            print_info("Restoring savings goals...");
            let goals: Vec<crate::models::SavingsGoal> = serde_json::from_value(goals.clone())?;
            let mut success_count = 0;
            let mut error_count = 0;
            
            for goal in goals {
                let goal_create = crate::models::SavingsGoalCreate {
                    user_id: goal.user_id,
                    name: goal.name,
                    target_amount: goal.target_amount,
                    current_amount: goal.current_amount,
                    deadline: goal.deadline,
                    description: goal.description,
                };
                
                match api.create_savings_goal(&goal_create) {
                    Ok(_) => {
                        success_count += 1;
                        print!(".");
                    }
                    Err(_) => {
                        error_count += 1;
                        print!("x");
                    }
                }
            }
            println!();
            
            restore_summary.insert("savings_goals".to_string(), (success_count, error_count));
        }
    }
    
    // Restore assets
    if restore_assets {
        if let Some(assets) = backup_data.get("assets") {
            print_info("Restoring assets...");
            let assets: Vec<crate::models::Asset> = serde_json::from_value(assets.clone())?;
            let mut success_count = 0;
            let mut error_count = 0;
            
            for asset in assets {
                let asset_create = crate::models::AssetCreate {
                    user_id: asset.user_id,
                    name: asset.name,
                    asset_type: asset.asset_type,
                    purchase_value: asset.purchase_value,
                    current_value: asset.current_value,
                    purchase_date: asset.purchase_date,
                    description: asset.description,
                    location: asset.location,
                };
                
                match api.create_asset(&asset_create) {
                    Ok(_) => {
                        success_count += 1;
                        print!(".");
                    }
                    Err(_) => {
                        error_count += 1;
                        print!("x");
                    }
                }
            }
            println!();
            
            restore_summary.insert("assets".to_string(), (success_count, error_count));
        }
    }
    
    // Restore recurring templates
    if restore_recurring {
        if let Some(templates) = backup_data.get("recurring_templates") {
            print_info("Restoring recurring templates...");
            let templates: Vec<crate::models::RecurringExpenseTemplate> = serde_json::from_value(templates.clone())?;
            let mut success_count = 0;
            let mut error_count = 0;
            
            for template in templates {
                let template_create = crate::models::RecurringExpenseTemplateCreate {
                    user_id: template.user_id,
                    amount: template.amount,
                    category: template.category,
                    description: template.description,
                    payment_method: template.payment_method,
                    credit_card_id: template.credit_card_id,
                    frequency: template.frequency,
                    interval: template.interval,
                    day_of_week: template.day_of_week,
                    day_of_month: template.day_of_month,
                    month_of_year: template.month_of_year,
                    start_date: template.start_date,
                    end_date: template.end_date,
                    tags: template.tags,
                };
                
                match api.create_recurring_template(&template_create) {
                    Ok(_) => {
                        success_count += 1;
                        print!(".");
                    }
                    Err(_) => {
                        error_count += 1;
                        print!("x");
                    }
                }
            }
            println!();
            
            restore_summary.insert("recurring_templates".to_string(), (success_count, error_count));
        }
    }
    
    // Display restore summary
    println!("\n✅ Restore Complete!");
    println!("\n📊 Restore Summary:");
    
    let mut sorted_summary: Vec<_> = restore_summary.iter().collect();
    sorted_summary.sort_by_key(|(k, _)| k.as_str());
    
    let mut total_success = 0;
    let mut total_errors = 0;
    
    for (entity, (success, errors)) in sorted_summary {
        println!("  • {}: {} restored, {} errors", entity, success, errors);
        total_success += success;
        total_errors += errors;
    }
    
    println!("\n[bold]Total:[/bold] {} records restored, {} errors", total_success, total_errors);
    
    if total_errors > 0 {
        println!("\n💡 Note: Errors may occur due to:");
        println!("  • Duplicate records (same email, card, etc.)");
        println!("  • Invalid foreign keys (user_id not found)");
        println!("  • Data validation failures");
    }
    
    Ok(())
}

fn list_backups() -> Result<()> {
    print_section_header("Available Backups");
    
    let backups = get_available_backups()?;
    
    if backups.is_empty() {
        print_info("No backups found in 'backups/' directory");
        println!("\n💡 Tip: Create a backup using 'Create Backup' option");
        return Ok(());
    }
    
    println!("\n📦 Backups in 'backups/' directory:\n");
    
    let mut total_size = 0u64;
    
    for (i, backup) in backups.iter().enumerate() {
        let backup_path = Path::new("backups").join(backup);
        let metadata = fs::metadata(&backup_path)?;
        let size = metadata.len();
        let modified = metadata.modified()?;
        let modified_time: chrono::DateTime<Local> = modified.into();
        
        total_size += size;
        
        println!("{}. {}", i + 1, backup);
        println!("   Size: {} bytes ({:.2} KB)", size, size as f64 / 1024.0);
        println!("   Modified: {}", modified_time.format("%Y-%m-%d %H:%M:%S"));
        
        // Try to read metadata
        if let Some(json_data) = if backup.ends_with(".gz") {
            let file = File::open(&backup_path).ok();
            if let Some(f) = file {
                let mut decoder = GzDecoder::new(f);
                let mut json_string = String::new();
                decoder.read_to_string(&mut json_string).ok();
                Some(json_string)
            } else {
                None
            }
        } else {
            fs::read_to_string(&backup_path).ok()
        } {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&json_data) {
                if let Some(metadata) = data.get("metadata") {
                    if let Some(counts) = metadata.get("record_counts") {
                        if let Some(obj) = counts.as_object() {
                            let total_records: u64 = obj.values()
                                .filter_map(|v| v.as_u64())
                                .sum();
                            println!("   Records: {} total", total_records);
                        }
                    }
                }
            }
        }
        
        println!();
    }
    
    println!("Total: {} backup(s), {:.2} MB", backups.len(), total_size as f64 / 1024.0 / 1024.0);
    
    Ok(())
}

fn delete_backups() -> Result<()> {
    print_section_header("Delete Old Backups");
    
    let backups = get_available_backups()?;
    
    if backups.is_empty() {
        print_info("No backups found");
        return Ok(());
    }
    
    println!("\nAvailable Backups:");
    for (i, backup) in backups.iter().enumerate() {
        let backup_path = Path::new("backups").join(backup);
        let metadata = fs::metadata(&backup_path)?;
        let size = metadata.len();
        println!("  {}. {} ({:.2} KB)", i + 1, backup, size as f64 / 1024.0);
    }
    
    let delete_options = vec![
        "Delete Single Backup".to_string(),
        "Delete Multiple Backups".to_string(),
        "Delete All Backups".to_string(),
        "Cancel".to_string(),
    ];
    
    let delete_choice = select_with_number("Delete Options", &delete_options)?;
    
    match delete_choice {
        0 => delete_single_backup(&backups)?,
        1 => delete_multiple_backups(&backups)?,
        2 => delete_all_backups(&backups)?,
        _ => print_info("Cancelled"),
    }
    
    Ok(())
}

fn delete_single_backup(backups: &[String]) -> Result<()> {
    let backup_idx = select_from_list(
        backups,
        "\nSelect Backup to Delete",
        |b| b.clone(),
    )?;
    
    let backup_file = &backups[backup_idx];
    
    if !prompt_confirm(&format!("Delete backup '{}'?", backup_file), false)? {
        print_info("Cancelled");
        return Ok(());
    }
    
    let backup_path = Path::new("backups").join(backup_file);
    fs::remove_file(&backup_path)?;
    
    print_success(&format!("Deleted backup: {}", backup_file));
    
    Ok(())
}

fn delete_multiple_backups(backups: &[String]) -> Result<()> {
    let selected = select_multiple_from_list(
        &backups.to_vec(),
        "Select Backups to Delete",
    )?;
    
    if selected.is_empty() {
        print_info("No backups selected");
        return Ok(());
    }
    
    println!("\nYou selected {} backup(s) to delete:", selected.len());
    for &idx in &selected {
        println!("  • {}", backups[idx]);
    }
    
    if !prompt_confirm(&format!("Delete {} backup(s)?", selected.len()), false)? {
        print_info("Cancelled");
        return Ok(());
    }
    
    let mut deleted_count = 0;
    for &idx in &selected {
        let backup_file = &backups[idx];
        let backup_path = Path::new("backups").join(backup_file);
        
        match fs::remove_file(&backup_path) {
            Ok(_) => {
                deleted_count += 1;
                print!(".");
            }
            Err(_) => print!("x"),
        }
    }
    println!();
    
    print_success(&format!("Deleted {} backup(s)", deleted_count));
    
    Ok(())
}

fn delete_all_backups(backups: &[String]) -> Result<()> {
    println!("\n⚠️  WARNING: This will delete ALL {} backup(s)!", backups.len());
    
    if !prompt_confirm("Are you absolutely sure?", false)? {
        print_info("Cancelled");
        return Ok(());
    }
    
    // Double confirmation
    if !prompt_confirm("Type 'yes' to confirm deletion of all backups", false)? {
        print_info("Cancelled");
        return Ok(());
    }
    
    let mut deleted_count = 0;
    for backup_file in backups {
        let backup_path = Path::new("backups").join(backup_file);
        
        match fs::remove_file(&backup_path) {
            Ok(_) => {
                deleted_count += 1;
                print!(".");
            }
            Err(_) => print!("x"),
        }
    }
    println!();
    
    print_success(&format!("Deleted {} backup(s)", deleted_count));
    
    Ok(())
}

fn get_available_backups() -> Result<Vec<String>> {
    let backups_dir = Path::new("backups");
    
    if !backups_dir.exists() {
        return Ok(vec![]);
    }
    
    let mut backups = vec![];
    
    for entry in fs::read_dir(backups_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(filename) = path.file_name() {
                if let Some(name) = filename.to_str() {
                    if name.ends_with(".json") || name.ends_with(".json.gz") {
                        backups.push(name.to_string());
                    }
                }
            }
        }
    }
    
    // Sort by name (which includes timestamp)
    backups.sort();
    backups.reverse(); // Most recent first
    
    Ok(backups)
}