use anyhow::Result;
use chrono::Local;
use crate::api::ApiClient;
use crate::display::Display;
use crate::models::RecurringExpenseTemplateCreate;
use crate::ui::*;
use crate::constants::{RECURRING_FREQUENCIES, DAYS_OF_WEEK, PAYMENT_METHODS};

pub fn handle_recurring(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Recurring Expense Management");
    
    let recurring_options = vec![
        "Create Recurring Template".to_string(),
        "List Templates".to_string(),
        "View Template Details".to_string(),
        "Update Template".to_string(),
        "Delete Template".to_string(),
        "View Upcoming Expenses".to_string(),
        "Generate Due Expenses Now".to_string(),
        "Skip Next Occurrence".to_string(),
        "Back to Main Menu".to_string(),
    ];
    
    loop {
        let selection = select_with_number("Recurring Expense Options", &recurring_options)?;
        
        match selection {
            0 => create_template(api, display)?,
            1 => list_templates(api, display)?,
            2 => view_template(api, display)?,
            3 => update_template(api, display)?,
            4 => delete_template(api, display)?,
            5 => view_upcoming(api, display)?,
            6 => generate_due(api, display)?,
            7 => skip_occurrence(api, display)?,
            8 => break,
            _ => break,
        }
    }
    
    Ok(())
}

fn create_template(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Create Recurring Expense Template");
    
    let amount = prompt_float("Amount", None)?;
    let category = crate::commands::expenses::select_category()?;
    let description = prompt_string("Description (optional)", None, true)?;
    
    // Payment method
    let payment_options: Vec<String> = PAYMENT_METHODS.iter().map(|s| s.to_string()).collect();
    let payment_idx = select_with_number("Payment Method", &payment_options)?;
    let payment_method = PAYMENT_METHODS[payment_idx].to_string();
    
    // Credit card (if applicable)
    let mut credit_card_id = None;
    if payment_method == "credit_card" {
        print_info("Fetching available credit cards...");
        let cards = api.list_credit_cards(None)?;
        
        if !cards.is_empty() {
            display.show("credit_cards", &cards)?;
            let card_idx = select_from_list(
                &cards,
                "Select Credit Card",
                |c| format!("{} (****{})", c.card_name, c.last_four),
            )?;
            credit_card_id = cards[card_idx].id;
        }
    }
    
    // User selection
    print_info("Fetching available users...");
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
    
    let user_id = users[user_idx].id.unwrap_or(0);
    
    // Frequency selection
    let freq_options: Vec<String> = RECURRING_FREQUENCIES.iter().map(|s| s.to_string()).collect();
    let freq_idx = select_with_number("Frequency", &freq_options)?;
    let frequency = RECURRING_FREQUENCIES[freq_idx].to_string();
    
    // Interval (for custom frequency)
    let interval = if frequency == "custom" {
        prompt_int("Interval (number of days)", Some(1), Some(1), Some(365))?
    } else {
        1
    };
    
    // Day of week (for weekly)
    let day_of_week = if frequency == "weekly" {
        let dow_options: Vec<String> = DAYS_OF_WEEK.iter().map(|s| s.to_string()).collect();
        let dow_idx = select_with_number("Day of Week", &dow_options)?;
        Some(dow_idx as i32)
    } else {
        None
    };
    
    // Day of month (for monthly/yearly)
    let day_of_month = if frequency == "monthly" || frequency == "yearly" {
        Some(prompt_int("Day of Month (1-31)", Some(1), Some(1), Some(31))?)
    } else {
        None
    };
    
    // Month of year (for yearly)
    let month_of_year = if frequency == "yearly" {
        Some(prompt_int("Month (1-12)", Some(1), Some(1), Some(12))?)
    } else {
        None
    };
    
    // Start date
    println!("\nStart Date:");
    let start_date = select_date_preset()?;
    
    // End date (optional)
    let end_date = if prompt_confirm("Set end date?", false)? {
        println!("\nEnd Date:");
        Some(select_date_preset()?)
    } else {
        None
    };
    
    // Tags
    let tags = prompt_string("Tags (comma-separated, optional)", None, true)?;
    
    print_info("Creating recurring template...");
    
    let template_create = RecurringExpenseTemplateCreate {
        user_id,
        amount,
        category,
        description: if description.is_empty() { None } else { Some(description) },
        payment_method,
        credit_card_id,
        frequency,
        interval,
        day_of_week,
        day_of_month,
        month_of_year,
        start_date,
        end_date,
        tags: if tags.is_empty() { None } else { Some(tags) },
    };
    
    let template = api.create_recurring_template(&template_create)?;
    print_success(&format!("Recurring template created with ID: {}", template.id.unwrap_or(0)));
    display.show("recurring_templates", &vec![template])?;
    
    Ok(())
}

fn list_templates(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Recurring Expense Templates");
    
    let filter_user = prompt_confirm("Filter by user?", false)?;
    let user_id = if filter_user {
        let users = api.list_users(Some(true))?;
        if users.is_empty() {
            None
        } else {
            display.show("users", &users)?;
            let user_idx = select_from_list(
                &users,
                "Select User",
                |u| format!("{} ({})", u.name, u.email),
            )?;
            users[user_idx].id
        }
    } else {
        None
    };
    
    print_info("Fetching recurring templates...");
    let templates = api.list_recurring_templates(user_id, None)?;
    
    if templates.is_empty() {
        print_info("No recurring templates found");
    } else {
        display.show("recurring_templates", &templates)?;
    }
    
    Ok(())
}

fn view_template(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("View Template Details");
    
    let templates = api.list_recurring_templates(None, None)?;
    if templates.is_empty() {
        print_error("No recurring templates found");
        return Ok(());
    }
    
    display.show("recurring_templates", &templates)?;
    
    let template_idx = select_from_list(
        &templates,
        "Select Template",
        |t| format!("{} - ${:.2} ({})", t.description.as_ref().unwrap_or(&t.category), t.amount, t.frequency),
    )?;
    
    let template_id = templates[template_idx].id.unwrap_or(0);
    
    print_info(&format!("Fetching template {}...", template_id));
    let template = api.get_recurring_template(template_id)?;
    display.show("recurring_templates", &vec![template])?;
    
    Ok(())
}

fn update_template(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Update Recurring Template");
    
    let templates = api.list_recurring_templates(None, None)?;
    if templates.is_empty() {
        print_error("No recurring templates found");
        return Ok(());
    }
    
    display.show("recurring_templates", &templates)?;
    
    let template_idx = select_from_list(
        &templates,
        "Select Template to Update",
        |t| format!("{} - ${:.2} ({})", t.description.as_ref().unwrap_or(&t.category), t.amount, t.frequency),
    )?;
    
    let current = &templates[template_idx];
    let template_id = current.id.unwrap_or(0);
    
    println!();
    
    // Similar to create but with defaults from current
    let amount = prompt_float("Amount", Some(current.amount))?;
    let category = crate::commands::expenses::select_category()?;
    let description = prompt_string(
        "Description (optional)",
        current.description.as_deref(),
        true,
    )?;
    
    // Payment method
    let current_pm_idx = PAYMENT_METHODS.iter()
        .position(|&p| p == current.payment_method)
        .unwrap_or(0);
    
    println!("\nCurrent payment method: {}", PAYMENT_METHODS[current_pm_idx]);
    let payment_options: Vec<String> = PAYMENT_METHODS.iter().map(|s| s.to_string()).collect();
    let payment_idx = select_with_number("Payment Method", &payment_options)?;
    let payment_method = PAYMENT_METHODS[payment_idx].to_string();
    
    // Credit card
    let mut credit_card_id = None;
    if payment_method == "credit_card" {
        print_info("Fetching available credit cards...");
        let cards = api.list_credit_cards(None)?;
        
        if !cards.is_empty() {
            display.show("credit_cards", &cards)?;
            let card_idx = select_from_list(
                &cards,
                "Select Credit Card",
                |c| format!("{} (****{})", c.card_name, c.last_four),
            )?;
            credit_card_id = cards[card_idx].id;
        }
    }
    
    // User
    print_info("Fetching available users...");
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
    
    let user_id = users[user_idx].id.unwrap_or(0);
    
    // Frequency
    let current_freq_idx = RECURRING_FREQUENCIES.iter()
        .position(|&f| f == current.frequency)
        .unwrap_or(0);
    
    println!("\nCurrent frequency: {}", RECURRING_FREQUENCIES[current_freq_idx]);
    let freq_options: Vec<String> = RECURRING_FREQUENCIES.iter().map(|s| s.to_string()).collect();
    let freq_idx = select_with_number("Frequency", &freq_options)?;
    let frequency = RECURRING_FREQUENCIES[freq_idx].to_string();
    
    let interval = if frequency == "custom" {
        prompt_int("Interval (number of days)", Some(current.interval), Some(1), Some(365))?
    } else {
        1
    };
    
    let day_of_week = if frequency == "weekly" {
        let dow_options: Vec<String> = DAYS_OF_WEEK.iter().map(|s| s.to_string()).collect();
        let default_dow = current.day_of_week.unwrap_or(0) as usize;
        let dow_idx = select_with_number("Day of Week", &dow_options)?;
        Some(dow_idx as i32)
    } else {
        None
    };
    
    let day_of_month = if frequency == "monthly" || frequency == "yearly" {
        Some(prompt_int(
            "Day of Month (1-31)",
            current.day_of_month,
            Some(1),
            Some(31),
        )?)
    } else {
        None
    };
    
    let month_of_year = if frequency == "yearly" {
        Some(prompt_int("Month (1-12)", current.month_of_year, Some(1), Some(12))?)
    } else {
        None
    };
    
    println!("\nStart Date:");
    let start_date = select_date_preset()?;
    
    let end_date = if prompt_confirm("Set end date?", current.end_date.is_some())? {
        println!("\nEnd Date:");
        Some(select_date_preset()?)
    } else {
        None
    };
    
    let tags = prompt_string(
        "Tags (comma-separated, optional)",
        current.tags.as_deref(),
        true,
    )?;
    
    print_info("Updating recurring template...");
    
    let template_update = RecurringExpenseTemplateCreate {
        user_id,
        amount,
        category,
        description: if description.is_empty() { None } else { Some(description) },
        payment_method,
        credit_card_id,
        frequency,
        interval,
        day_of_week,
        day_of_month,
        month_of_year,
        start_date,
        end_date,
        tags: if tags.is_empty() { None } else { Some(tags) },
    };
    
    let template = api.update_recurring_template(template_id, &template_update)?;
    print_success(&format!("Template {} updated", template_id));
    display.show("recurring_templates", &vec![template])?;
    
    Ok(())
}

fn delete_template(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Delete Recurring Template");
    
    let templates = api.list_recurring_templates(None, None)?;
    if templates.is_empty() {
        print_error("No recurring templates found");
        return Ok(());
    }
    
    display.show("recurring_templates", &templates)?;
    
    let template_idx = select_from_list(
        &templates,
        "Select Template to Delete",
        |t| format!("{} - ${:.2} ({})", t.description.as_ref().unwrap_or(&t.category), t.amount, t.frequency),
    )?;
    
    let template = &templates[template_idx];
    let template_id = template.id.unwrap_or(0);
    
    if !prompt_confirm(&format!("Delete template '{}'?", template.description.as_ref().unwrap_or(&template.category)), false)? {
        print_info("Cancelled");
        return Ok(());
    }
    
    print_info(&format!("Deleting template {}...", template_id));
    api.delete_recurring_template(template_id)?;
    print_success(&format!("Template {} deleted", template_id));
    
    Ok(())
}

fn view_upcoming(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Upcoming Recurring Expenses");
    
    let days = prompt_int("Show upcoming for how many days?", Some(7), Some(1), Some(90))?;
    
    let filter_user = prompt_confirm("Filter by user?", false)?;
    let user_id = if filter_user {
        let users = api.list_users(Some(true))?;
        if users.is_empty() {
            None
        } else {
            display.show("users", &users)?;
            let user_idx = select_from_list(
                &users,
                "Select User",
                |u| format!("{} ({})", u.name, u.email),
            )?;
            users[user_idx].id
        }
    } else {
        None
    };
    
    print_info(&format!("Fetching upcoming expenses for next {} days...", days));
    let upcoming = api.get_upcoming_recurring_expenses(days, user_id)?;
    display.show("upcoming_recurring", &upcoming)?;
    
    Ok(())
}

fn generate_due(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Generate Due Recurring Expenses");
    
    print_info("Checking for due recurring expenses...");
    
    // First, show what's due
    let upcoming = api.get_upcoming_recurring_expenses(0, None)?;
    let due_count = upcoming["count"].as_u64().unwrap_or(0);
    
    if due_count == 0 {
        print_info("No recurring expenses are due today");
        return Ok(());
    }
    
    display.show("upcoming_recurring", &upcoming)?;
    
    if !prompt_confirm(&format!("Generate {} due expense(s)?", due_count), true)? {
        print_info("Cancelled");
        return Ok(());
    }
    
    print_info("Generating expenses...");
    let result = api.generate_due_recurring_expenses()?;
    
    let generated = result["generated_count"].as_u64().unwrap_or(0);
    let errors = result["error_count"].as_u64().unwrap_or(0);
    
    print_success(&format!("Generated {} expense(s)", generated));
    
    if errors > 0 {
        print_error(&format!("{} error(s) occurred", errors));
    }
    
    // Show generated expenses
    if let Some(generated_list) = result["generated"].as_array() {
        if !generated_list.is_empty() {
            println!("\n📋 Generated Expenses:");
            for (i, expense) in generated_list.iter().enumerate() {
                println!("  {}. {} - ${:.2} ({})", 
                    i + 1,
                    expense["category"].as_str().unwrap_or("Unknown"),
                    expense["amount"].as_f64().unwrap_or(0.0),
                    expense["date"].as_str().unwrap_or("")
                );
            }
        }
    }
    
    Ok(())
}

fn skip_occurrence(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Skip Next Occurrence");
    
    let templates = api.list_recurring_templates(None, None)?;
    if templates.is_empty() {
        print_error("No recurring templates found");
        return Ok(());
    }
    
    display.show("recurring_templates", &templates)?;
    
    let template_idx = select_from_list(
        &templates,
        "Select Template",
        |t| format!("{} - Next: {}", t.description.as_ref().unwrap_or(&t.category), t.next_occurrence),
    )?;
    
    let template = &templates[template_idx];
    let template_id = template.id.unwrap_or(0);
    
    println!("\nCurrent next occurrence: {}", template.next_occurrence);
    
    if !prompt_confirm("Skip this occurrence?", false)? {
        print_info("Cancelled");
        return Ok(());
    }
    
    print_info("Skipping occurrence...");
    let result = api.skip_recurring_occurrence(template_id)?;
    
    println!("\n✅ Occurrence skipped");
    println!("  Old next: {}", result["old_next_occurrence"].as_str().unwrap_or(""));
    println!("  New next: {}", result["new_next_occurrence"].as_str().unwrap_or(""));
    
    Ok(())
}

// Function to check and prompt for due recurring expenses on startup
pub fn check_recurring_on_startup(api: &ApiClient) -> Result<()> {
    match api.get_upcoming_recurring_expenses(0, None) {
        Ok(upcoming) => {
            let due_count = upcoming["count"].as_u64().unwrap_or(0);
            
            if due_count > 0 {
                println!("\n🔔 [bold yellow]Recurring Expenses Due![/bold yellow]");
                println!("   You have {} recurring expense(s) due today", due_count);
                
                if let Some(expenses) = upcoming["upcoming_expenses"].as_array() {
                    println!("\n   Due today:");
                    for expense in expenses.iter().take(3) {
                        println!("   • {} - ${:.2}",
                            expense["description"].as_str()
                                .or(expense["category"].as_str())
                                .unwrap_or("Unknown"),
                            expense["amount"].as_f64().unwrap_or(0.0)
                        );
                    }
                    if expenses.len() > 3 {
                        println!("   ... and {} more", expenses.len() - 3);
                    }
                }
                
                println!("\n   💡 Use './expense -q r' to manage recurring expenses\n");
            }
        }
        Err(_) => {
            // Silently fail - don't interrupt startup
        }
    }
    
    Ok(())
}