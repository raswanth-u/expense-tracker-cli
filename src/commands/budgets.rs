use anyhow::Result;
use crate::api::ApiClient;
use crate::config::Config;
use crate::display::Display;
use crate::models::BudgetCreate;
use crate::ui::*;
use crate::constants::BUDGET_PERIODS;

pub fn handle_budgets(api: &ApiClient, display: &Display, _config: &Config) -> Result<()> {
    loop {
        match show_budget_menu()? {
            BudgetMenuOption::Create => create_budget(api, display)?,
            BudgetMenuOption::List => list_budgets(api, display)?,
            BudgetMenuOption::View => view_budget(api, display)?,
            BudgetMenuOption::Update => update_budget(api, display)?,
            BudgetMenuOption::Delete => delete_budget(api, display)?,
            BudgetMenuOption::Status => budget_status(api, display)?,
            BudgetMenuOption::Alerts => budget_alerts(api, display)?,
            BudgetMenuOption::Compare => compare_budgets(api, display)?,
            BudgetMenuOption::Back => break,
        }
    }
    Ok(())
}

fn create_budget(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Creating New Budget");
    
    let category = crate::commands::expenses::select_category()?;
    let amount = prompt_float("Budget Amount", None)?;
    let month = select_month_preset()?;
    
    let is_family_budget = prompt_confirm("Is this a family-wide budget?", false)?;
    
    let user_id = if is_family_budget {
        None
    } else {
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
        
        Some(users[user_idx].id.unwrap_or(0))
    };
    
    let period_options: Vec<String> = BUDGET_PERIODS.iter().map(|s| s.to_string()).collect();
    let period_idx = select_with_number("Period", &period_options)?;
    let period = BUDGET_PERIODS[period_idx].to_string();
    
    print_info("Creating budget...");
    
    let budget_create = BudgetCreate {
        user_id,
        category,
        amount,
        month,
        period: Some(period),
    };
    
    let budget = api.create_budget(&budget_create)?;
    print_success(&format!("Budget created with ID: {}", budget.id.unwrap_or(0)));
    display.show("budgets", &vec![budget])?;
    
    Ok(())
}

fn list_budgets(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Listing Budgets");
    
    let use_filters = prompt_confirm("Apply filters?", false)?;
    
    let (month, user_id, category) = if use_filters {
        let month = if prompt_confirm("Filter by month?", false)? {
            Some(select_month_preset()?)
        } else {
            None
        };
        
        let user_id = if prompt_confirm("Filter by user?", false)? {
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
        
        let category = if prompt_confirm("Filter by category?", false)? {
            Some(crate::commands::expenses::select_category()?)
        } else {
            None
        };
        
        (month, user_id, category)
    } else {
        (None, None, None)
    };
    
    print_info("Fetching budgets...");
    let budgets = api.list_budgets(month.as_deref(), user_id, category.as_deref())?;
    
    if budgets.is_empty() {
        print_info("No budgets found");
    } else {
        display.show("budgets", &budgets)?;
    }
    
    Ok(())
}

fn view_budget(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("View Budget Details");
    
    let budget_id = prompt_int("Budget ID", None, Some(1), None)?;
    
    print_info(&format!("Fetching budget {}...", budget_id));
    let budget = api.get_budget(budget_id)?;
    display.show("budgets", &vec![budget])?;
    
    Ok(())
}

fn update_budget(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Update Budget");
    
    let budget_id = prompt_int("Budget ID to update", None, Some(1), None)?;
    
    print_info(&format!("Fetching budget {}...", budget_id));
    let current = api.get_budget(budget_id)?;
    
    display.show("budgets", &vec![current.clone()])?;
    println!();
    
    let category = crate::commands::expenses::select_category()?;
    let amount = prompt_float("Budget Amount", Some(current.amount))?;
    let month = select_month_preset()?;
    
    let is_family_budget = prompt_confirm("Is this a family-wide budget?", current.user_id.is_none())?;
    
    let user_id = if is_family_budget {
        None
    } else {
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
        
        Some(users[user_idx].id.unwrap_or(0))
    };
    
    let current_period_idx = BUDGET_PERIODS.iter()
        .position(|&p| Some(p.to_string()) == current.period)
        .unwrap_or(0);
    
    println!("\nCurrent period: {}", BUDGET_PERIODS[current_period_idx]);
    let period_options: Vec<String> = BUDGET_PERIODS.iter().map(|s| s.to_string()).collect();
    let period_idx = select_with_number("Period", &period_options)?;
    let period = BUDGET_PERIODS[period_idx].to_string();
    
    print_info("Updating budget...");
    
    let budget_update = BudgetCreate {
        user_id,
        category,
        amount,
        month,
        period: Some(period),
    };
    
    let budget = api.update_budget(budget_id, &budget_update)?;
    print_success(&format!("Budget {} updated", budget_id));
    display.show("budgets", &vec![budget])?;
    
    Ok(())
}

fn delete_budget(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Delete Budget");
    
    let budget_id = prompt_int("Budget ID to delete", None, Some(1), None)?;
    
    print_info(&format!("Fetching budget {}...", budget_id));
    let budget = api.get_budget(budget_id)?;
    display.show("budgets", &vec![budget])?;
    
    if !prompt_confirm(&format!("Delete budget {}?", budget_id), false)? {
        print_info("Cancelled");
        return Ok(());
    }
    
    print_info(&format!("Deleting budget {}...", budget_id));
    api.delete_budget(budget_id)?;
    print_success(&format!("Budget {} deleted", budget_id));
    
    Ok(())
}

fn budget_status(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Budget Status");
    
    let month = select_month_preset()?;
    
    let user_id = if prompt_confirm("Filter by user?", false)? {
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
    
    print_info(&format!("Fetching budget status for {}...", month));
    let status = api.get_budget_status(&month, user_id)?;
    display.show("budget_status", &status)?;
    
    Ok(())
}

fn budget_alerts(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Budget Alerts");
    
    let month = select_month_preset()?;
    
    let user_id = if prompt_confirm("Filter by user?", false)? {
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
    
    print_info(&format!("Fetching budget alerts for {}...", month));
    let alerts = api.get_budget_alerts(&month, user_id)?;
    display.show("budget_alerts", &alerts)?;
    
    Ok(())
}

fn compare_budgets(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Compare Budgets");
    
    println!("\nFirst Month:");
    let month1 = select_month_preset()?;
    
    println!("\nSecond Month:");
    let month2 = select_month_preset()?;
    
    let user_id = if prompt_confirm("Filter by user?", false)? {
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
    
    print_info(&format!("Comparing budgets: {} vs {}...", month1, month2));
    let comparison = api.compare_budgets(&month1, &month2, user_id)?;
    display.show("budget_comparison", &comparison)?;
    
    Ok(())
}