use anyhow::Result;
use crate::api::ApiClient;
use crate::display::Display;
use crate::models::ExpenseFilters;
use crate::ui::*;
use crate::constants::PAYMENT_METHODS;  // Remove CATEGORIES from import

pub fn handle_search(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Search Expenses");
    
    // Ask for search type
    let search_options = vec![
        "Quick Search (by description/category)".to_string(),
        "Advanced Search (multiple filters)".to_string(),
    ];
    
    let search_type = select_with_number("Search Type", &search_options)?;
    
    match search_type {
        0 => quick_search(api, display)?,
        1 => advanced_search(api, display)?,
        _ => quick_search(api, display)?,
    }
    
    Ok(())
}

fn quick_search(api: &ApiClient, display: &Display) -> Result<()> {
    println!("\n[Quick Search]");
    println!("Enter keywords to search in description and category");
    println!("Examples: 'groceries', 'uber', 'restaurant'\n");
    
    let query = prompt_string("Search query", None, false)?;
    
    if query.trim().is_empty() {
        print_error("Search query cannot be empty");
        return Ok(());
    }
    
    print_info(&format!("Searching for '{}'...", query));
    
    // Fetch all expenses (we'll filter client-side for fuzzy matching)
    let all_expenses = api.list_expenses_filtered(&ExpenseFilters::default())?;
    
    // Filter by query (case-insensitive, matches description or category)
    let query_lower = query.to_lowercase();
    let results: Vec<_> = all_expenses.iter()
        .filter(|e| {
            let desc = e.description.as_ref().map(|s| s.to_lowercase()).unwrap_or_default();
            let category = e.category.to_lowercase();
            let tags = e.tags.as_ref().map(|s| s.to_lowercase()).unwrap_or_default();
            
            desc.contains(&query_lower) || 
            category.contains(&query_lower) ||
            tags.contains(&query_lower)
        })
        .cloned()
        .collect();
    
    display_search_results(&results, &query, display)?;
    
    Ok(())
}

fn advanced_search(api: &ApiClient, display: &Display) -> Result<()> {
    println!("\n[Advanced Search]");
    println!("Select filters to apply:\n");
    
    let mut filters = ExpenseFilters::default();
    
    // 1. Text search (description/category)
    if prompt_confirm("Search by keywords?", false)? {
        let query = prompt_string("Keywords", None, false)?;
        // We'll use this for client-side filtering later
        filters.tags = Some(query); // Temporarily store query in tags field
    }
    
    // 2. Date range
    if prompt_confirm("Filter by date range?", false)? {
        println!("\nFrom Date:");
        filters.from_date = Some(select_date_preset()?);
        println!("\nTo Date:");
        filters.to_date = Some(select_date_preset()?);
    }
    
    // 3. Amount range
    if prompt_confirm("Filter by amount range?", false)? {
        filters.min_amount = Some(prompt_float("Minimum amount", None)?);
        filters.max_amount = Some(prompt_float("Maximum amount", None)?);
    }
    
    // 4. Category
    if prompt_confirm("Filter by category?", false)? {
        filters.category = Some(crate::commands::expenses::select_category()?);
    }
    
    // 5. Payment method
    if prompt_confirm("Filter by payment method?", false)? {
        let pm_options: Vec<String> = PAYMENT_METHODS.iter().map(|s| s.to_string()).collect();
        let pm_idx = select_with_number("Payment Method", &pm_options)?;
        filters.payment_method = Some(PAYMENT_METHODS[pm_idx].to_string());
    }
    
    // 6. User
    if prompt_confirm("Filter by user?", false)? {
        let users = api.list_users(Some(true))?;
        if !users.is_empty() {
            display.show("users", &users)?;
            let user_idx = select_from_list(
                &users,
                "Select User",
                |u| format!("{} ({})", u.name, u.email),
            )?;
            filters.user_id = users[user_idx].id;
        }
    }
    
    // 7. Recurring
    if prompt_confirm("Filter by recurring status?", false)? {
        let recurring = prompt_confirm("Show only recurring expenses?", false)?;
        filters.is_recurring = Some(recurring);
    }
    
    print_info("Searching with filters...");
    
    // Extract keyword query if present
    let keyword_query = filters.tags.clone();
    filters.tags = None; // Clear the temporary storage
    
    // Fetch filtered expenses
    let mut results = api.list_expenses_filtered(&filters)?;
    
    // Apply keyword filtering if provided
    if let Some(query) = keyword_query {
        let query_lower = query.to_lowercase();
        results.retain(|e| {
            let desc = e.description.as_ref().map(|s| s.to_lowercase()).unwrap_or_default();
            let category = e.category.to_lowercase();
            let tags = e.tags.as_ref().map(|s| s.to_lowercase()).unwrap_or_default();
            
            desc.contains(&query_lower) || 
            category.contains(&query_lower) ||
            tags.contains(&query_lower)
        });
    }
    
    display_search_results(&results, "advanced search", display)?;
    
    Ok(())
}

fn display_search_results(
    results: &[crate::models::Expense],
    query: &str,
    display: &Display,
) -> Result<()> {
    if results.is_empty() {
        print_info(&format!("No results found for '{}'", query));
        return Ok(());
    }
    
    // Calculate summary
    let total_amount: f64 = results.iter().map(|e| e.amount).sum();
    let count = results.len();
    
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║                    Search Results                         ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    println!("  Found: {} transaction(s)", count);
    println!("  Total Amount: ${:.2}", total_amount);
    println!("  Average: ${:.2}", if count > 0 { total_amount / count as f64 } else { 0.0 });
    println!();
    
    // Display results - Convert slice to Vec for display
    display.show("expenses", &results.to_vec())?;
    
    // Ask for actions
    println!("\nWhat would you like to do?");
    let actions = vec![
        "View details of an expense".to_string(),
        "Export results to CSV".to_string(),
        "Return to menu".to_string(),
    ];
    
    let action = select_with_number("Select action", &actions)?;
    
    match action {
        0 => {
            let expense_id = prompt_int("Enter Expense ID", None, Some(1), None)?;
            if let Some(expense) = results.iter().find(|e| e.id == Some(expense_id)) {
                display.show("expenses", &vec![expense.clone()])?;
            } else {
                print_error("Expense not found in search results");
            }
        }
        1 => {
            export_search_results(results)?;
        }
        _ => {}
    }
    
    Ok(())
}

fn export_search_results(results: &[crate::models::Expense]) -> Result<()> {
    let filename = prompt_string(
        "Output filename",
        Some("search_results.csv"),
        false,
    )?;
    
    // Create CSV content
    let mut csv_content = String::from("ID,Date,User ID,Amount,Category,Description,Payment Method,Recurring,Tags\n");
    
    for expense in results {
        csv_content.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            expense.id.unwrap_or(0),
            expense.date,
            expense.user_id,
            expense.amount,
            expense.category,
            expense.description.as_ref().unwrap_or(&String::new()),
            expense.payment_method,
            expense.is_recurring.unwrap_or(false),
            expense.tags.as_ref().unwrap_or(&String::new()),
        ));
    }
    
    std::fs::write(&filename, csv_content)?;
    print_success(&format!("Exported {} results to {}", results.len(), filename));
    
    Ok(())
}