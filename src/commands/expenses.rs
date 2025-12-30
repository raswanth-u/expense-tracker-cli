use anyhow::Result;
use crate::api::ApiClient;
use crate::config::Config;
use crate::display::Display;
use crate::models::{ExpenseCreate, ExpenseFilters};
use crate::ui::*;
use crate::constants::{CATEGORIES, PAYMENT_METHODS, COMMON_TAGS, suggest_category};

pub fn handle_expenses(api: &ApiClient, display: &Display, _config: &Config) -> Result<()> {
    loop {
        match show_expense_menu()? {
            ExpenseMenuOption::Create => create_expense(api, display, _config)?,
            ExpenseMenuOption::List => list_expenses(api, display)?,
            ExpenseMenuOption::View => view_expense(api, display)?,
            ExpenseMenuOption::Update => update_expense(api, display)?,
            ExpenseMenuOption::Delete => delete_expense(api, display)?,
            ExpenseMenuOption::SummaryByCategory => summary_by_category(api, display)?,
            ExpenseMenuOption::SummaryByPayment => summary_by_payment(api, display)?,
            ExpenseMenuOption::Back => break,
        }
    }
    Ok(())
}

fn create_expense(api: &ApiClient, display: &Display, _config: &Config) -> Result<()> {
    print_section_header("Creating New Expense");
    
    // Amount
    let amount = prompt_float("Amount", None)?;
    
    // Description (optional)
    let description = prompt_string("Description (optional)", None, true)?;
    
    // Smart category suggestion
    let suggested_category = if !description.is_empty() {
        suggest_category(&description)
    } else {
        None
    };
    
    // Category selection
    let category = if let Some(suggested) = suggested_category {
        if prompt_confirm(&format!("Use suggested category '{}'?", suggested), true)? {
            suggested.to_string()
        } else {
            select_category()?
        }
    } else {
        select_category()?
    };
    
    // Date
    let date = select_date_preset()?;
    
    // Payment Method
    let payment_options: Vec<String> = PAYMENT_METHODS.iter().map(|s| s.to_string()).collect();
    let payment_idx = select_with_number("Payment Method", &payment_options)?;
    let payment_method = PAYMENT_METHODS[payment_idx].to_string();
    
    // Credit Card (if applicable)
    let mut credit_card_id = None;
    if payment_method == "credit_card" {
        print_info("Fetching available credit cards...");
        let cards = api.list_credit_cards(None)?;
        
        if cards.is_empty() {
            print_error("No credit cards found. Please create one first.");
            return Ok(());
        }
        
        display.show("credit_cards", &cards)?;
        
        let card_idx = select_from_list(
            &cards,
            "Select Credit Card",
            |c| format!("{} (****{})", c.card_name, c.last_four),
        )?;
        
        credit_card_id = cards[card_idx].id;
    }
    
    // User Selection
    print_info("Fetching available users...");
    let users = api.list_users(Some(true))?;
    
    if users.is_empty() {
        print_error("No users found. Please create one first.");
        return Ok(());
    }
    
    display.show("users", &users)?;
    
    let user_idx = select_from_list(
        &users,
        "Select User",
        |u| format!("{} ({})", u.name, u.email),
    )?;
    
    let user_id = users[user_idx].id.unwrap_or(0);
    
    // Recurring
    let recurring = prompt_confirm("Is this a recurring expense?", false)?;
    
    // Tags
    let tags = select_tags()?;
    
    // Create expense
    print_info("Creating expense...");
    
    let expense_create = ExpenseCreate {
        user_id,
        amount,
        category,
        description: if description.is_empty() { None } else { Some(description) },
        date,
        payment_method,
        credit_card_id,
        is_recurring: Some(recurring),
        tags: if tags.is_empty() { None } else { Some(tags) },
    };
    
    let expense = api.create_expense(&expense_create)?;
    print_success(&format!("Expense created with ID: {}", expense.id.unwrap_or(0)));
    display.show("expenses", &vec![expense])?;
    
    Ok(())
}

pub fn select_category() -> Result<String> {
    let category_options: Vec<String> = CATEGORIES.iter().map(|s| s.to_string()).collect();
    let category_idx = select_with_number("Category", &category_options)?;
    
    let category = CATEGORIES[category_idx];
    
    if category == "Other (Custom)" {
        Ok(prompt_string("Enter custom category", None, false)?)
    } else {
        Ok(category.to_string())
    }
}

fn select_tags() -> Result<String> {
    if !prompt_confirm("Add tags?", false)? {
        return Ok(String::new());
    }
    
    let options = vec![
        "Select from common tags".to_string(),
        "Enter custom tags".to_string(),
    ];
    
    let choice = select_with_number("Tag selection method", &options)?;
    
    if choice == 0 {
        let selected = select_multiple_from_list(
            &COMMON_TAGS.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            "Select tags",
        )?;
        
        let tags: Vec<String> = selected.iter()
            .map(|&i| COMMON_TAGS[i].to_string())
            .collect();
        
        Ok(tags.join(","))
    } else {
        Ok(prompt_string("Enter tags (comma-separated)", None, true)?)
    }
}

fn list_expenses(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Listing Expenses");
    
    // Ask for filters
    let use_filters = prompt_confirm("Apply filters?", false)?;
    
    let filters = if use_filters {
        build_expense_filters(api)?
    } else {
        ExpenseFilters::default()
    };
    
    print_info("Fetching expenses...");
    let expenses = api.list_expenses_filtered(&filters)?;
    
    if expenses.is_empty() {
        print_info("No expenses found");
    } else {
        display.show("expenses", &expenses)?;
    }
    
    Ok(())
}

fn build_expense_filters(api: &ApiClient) -> Result<ExpenseFilters> {
    let filter_options = vec![
        "User".to_string(),
        "Date Range".to_string(),
        "Category".to_string(),
        "Payment Method".to_string(),
        "Amount Range".to_string(),
    ];
    
    let selected = select_multiple_from_list(
        &filter_options,
        "Select filters",
    )?;
    
    let mut filters = ExpenseFilters::default();
    
    for &idx in &selected {
        match idx {
            0 => {
                // User filter
                let users = api.list_users(Some(true))?;
                if !users.is_empty() {
                    let config = crate::config::Config::load()?;
                    let display = crate::display::Display::new(&config);
                    display.show("users", &users)?;
                    let user_idx = select_from_list(
                        &users,
                        "Select User",
                        |u| format!("{} ({})", u.name, u.email),
                    )?;
                    filters.user_id = users[user_idx].id;
                }
            }
            1 => {
                // Date range
                println!("\nFrom Date:");
                filters.from_date = Some(select_date_preset()?);
                println!("\nTo Date:");
                filters.to_date = Some(select_date_preset()?);
            }
            2 => {
                // Category
                filters.category = Some(select_category()?);
            }
            3 => {
                // Payment method
                let pm_options: Vec<String> = PAYMENT_METHODS.iter().map(|s| s.to_string()).collect();
                let pm_idx = select_with_number("Payment Method", &pm_options)?;
                filters.payment_method = Some(PAYMENT_METHODS[pm_idx].to_string());
            }
            4 => {
                // Amount range
                filters.min_amount = Some(prompt_float("Minimum amount", None)?);
                filters.max_amount = Some(prompt_float("Maximum amount", None)?);
            }
            _ => {}
        }
    }
    
    Ok(filters)
}

fn view_expense(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("View Expense Details");
    
    let expense_id = prompt_int("Expense ID", None, Some(1), None)?;
    
    print_info(&format!("Fetching expense {}...", expense_id));
    let expense = api.get_expense(expense_id)?;
    display.show("expenses", &vec![expense])?;
    
    Ok(())
}

fn update_expense(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Update Expense");
    
    let expense_id = prompt_int("Expense ID to update", None, Some(1), None)?;
    
    print_info(&format!("Fetching expense {}...", expense_id));
    let current = api.get_expense(expense_id)?;
    
    display.show("expenses", &vec![current.clone()])?;
    println!();
    
    // Amount
    let amount = prompt_float("Amount", Some(current.amount))?;
    
    // Category
    let category = select_category()?;
    
    // Description
    let description = prompt_string(
        "Description (optional)",
        current.description.as_deref(),
        true,
    )?;
    
    // Date
    let date = select_date_preset()?;
    
    // Payment Method
    let current_pm_idx = PAYMENT_METHODS.iter()
        .position(|&p| p == current.payment_method)
        .unwrap_or(0);
    
    println!("\nCurrent payment method: {}", PAYMENT_METHODS[current_pm_idx]);
    let payment_options: Vec<String> = PAYMENT_METHODS.iter().map(|s| s.to_string()).collect();
    let payment_idx = select_with_number("Payment Method", &payment_options)?;
    let payment_method = PAYMENT_METHODS[payment_idx].to_string();
    
    // Credit Card
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
        print_error("No users found.");
        return Ok(());
    }
    
    display.show("users", &users)?;
    
    let user_idx = select_from_list(
        &users,
        "Select User",
        |u| format!("{} ({})", u.name, u.email),
    )?;
    
    let user_id = users[user_idx].id.unwrap_or(0);
    
    // Tags
    let tags = prompt_string(
        "Tags (comma-separated, optional)",
        current.tags.as_deref(),
        true,
    )?;
    
    print_info("Updating expense...");
    
    let expense_update = ExpenseCreate {
        user_id,
        amount,
        category,
        description: if description.is_empty() { None } else { Some(description) },
        date,
        payment_method,
        credit_card_id,
        is_recurring: current.is_recurring,
        tags: if tags.is_empty() { None } else { Some(tags) },
    };
    
    let expense = api.update_expense(expense_id, &expense_update)?;
    print_success(&format!("Expense {} updated", expense_id));
    display.show("expenses", &vec![expense])?;
    
    Ok(())
}

fn delete_expense(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Delete Expense");
    
    let expense_id = prompt_int("Expense ID to delete", None, Some(1), None)?;
    
    print_info(&format!("Fetching expense {}...", expense_id));
    let expense = api.get_expense(expense_id)?;
    display.show("expenses", &vec![expense])?;
    
    if !prompt_confirm(&format!("Delete expense {}?", expense_id), false)? {
        print_info("Cancelled");
        return Ok(());
    }
    
    print_info(&format!("Deleting expense {}...", expense_id));
    api.delete_expense(expense_id)?;
    print_success(&format!("Expense {} deleted", expense_id));
    
    Ok(())
}

fn summary_by_category(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Expense Summary by Category");
    
    let use_date_filter = prompt_confirm("Filter by date range?", false)?;
    
    let (from_date, to_date) = if use_date_filter {
        println!("\nFrom Date:");
        let from = select_date_preset()?;
        println!("\nTo Date:");
        let to = select_date_preset()?;
        (Some(from), Some(to))
    } else {
        (None, None)
    };
    
    let use_user_filter = prompt_confirm("Filter by user?", false)?;
    let user_id = if use_user_filter {
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
    
    print_info("Fetching summary...");
    let summary = api.get_expense_summary(from_date.as_deref(), to_date.as_deref(), user_id)?;
    display.show("expense_summary", &summary)?;
    
    Ok(())
}

fn summary_by_payment(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Expense Summary by Payment Method");
    
    let use_date_filter = prompt_confirm("Filter by date range?", false)?;
    
    let (from_date, to_date) = if use_date_filter {
        println!("\nFrom Date:");
        let from = select_date_preset()?;
        println!("\nTo Date:");
        let to = select_date_preset()?;
        (Some(from), Some(to))
    } else {
        (None, None)
    };
    
    let use_user_filter = prompt_confirm("Filter by user?", false)?;
    let user_id = if use_user_filter {
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
    
    print_info("Fetching summary...");
    let summary = api.get_payment_summary(from_date.as_deref(), to_date.as_deref(), user_id)?;
    display.show("payment_summary", &summary)?;
    
    Ok(())
}