use anyhow::Result;
use crate::api::ApiClient;
use crate::display::Display;
use crate::models::{SavingsGoalCreate, SavingsGoalUpdate};
use crate::ui::*;

pub fn handle_savings(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Savings Goals");
    
    let savings_options = vec![
        "Create New Goal".to_string(),
        "List Goals".to_string(),
        "View Goal Progress".to_string(),
        "Add Money to Goal".to_string(),
        "Withdraw from Goal".to_string(),
        "Update Goal".to_string(),
        "Delete Goal".to_string(),
        "Back to Main Menu".to_string(),
    ];
    
    loop {
        let selection = select_with_number("Savings Options", &savings_options)?;
        
        match selection {
            0 => create_goal(api, display)?,
            1 => list_goals(api, display)?,
            2 => view_progress(api, display)?,
            3 => add_money(api, display)?,
            4 => withdraw_money(api, display)?,
            5 => update_goal(api, display)?,
            6 => delete_goal(api, display)?,
            7 => break,
            _ => break,
        }
    }
    
    Ok(())
}

fn create_goal(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Create New Savings Goal");
    
    let name = prompt_string("Goal Name", None, false)?;
    let target_amount = prompt_float("Target Amount", None)?;
    let current_amount = prompt_float("Current Amount", Some(0.0))?;
    
    let deadline = select_date_preset()?;
    
    let description = prompt_string("Description (optional)", None, true)?;
    
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
    
    print_info("Creating savings goal...");
    
    let goal_create = SavingsGoalCreate {
        user_id,
        name,
        target_amount,
        current_amount,
        deadline,
        description: if description.is_empty() { None } else { Some(description) },
    };
    
    let goal = api.create_savings_goal(&goal_create)?;
    print_success(&format!("Savings goal created with ID: {}", goal.id.unwrap_or(0)));
    display.show("savings_goals", &vec![goal])?;
    
    Ok(())
}

fn list_goals(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Savings Goals");
    
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
    
    print_info("Fetching savings goals...");
    let goals = api.list_savings_goals(user_id)?;
    
    if goals.is_empty() {
        print_info("No savings goals found");
    } else {
        display.show("savings_goals", &goals)?;
    }
    
    Ok(())
}

fn view_progress(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("View Goal Progress");
    
    let goals = api.list_savings_goals(None)?;
    if goals.is_empty() {
        print_error("No savings goals found");
        return Ok(());
    }
    
    display.show("savings_goals", &goals)?;
    
    let goal_idx = select_from_list(
        &goals,
        "Select Goal",
        |g| format!("{} - ${:.2}/{:.2}", g.name, g.current_amount, g.target_amount),
    )?;
    
    let goal_id = goals[goal_idx].id.unwrap_or(0);
    
    print_info(&format!("Fetching progress for goal {}...", goal_id));
    let progress = api.get_savings_goal_progress(goal_id)?;
    display.show("savings_goal_progress", &progress)?;
    
    Ok(())
}

fn add_money(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Add Money to Goal");
    
    let goals = api.list_savings_goals(None)?;
    if goals.is_empty() {
        print_error("No savings goals found");
        return Ok(());
    }
    
    display.show("savings_goals", &goals)?;
    
    let goal_idx = select_from_list(
        &goals,
        "Select Goal",
        |g| format!("{} - ${:.2}/{:.2}", g.name, g.current_amount, g.target_amount),
    )?;
    
    let goal = &goals[goal_idx];
    let goal_id = goal.id.unwrap_or(0);
    
    let amount = prompt_float("Amount to Add", None)?;
    
    print_info("Adding money to goal...");
    
    let update = SavingsGoalUpdate { amount };
    let updated_goal = api.add_to_savings_goal(goal_id, &update)?;
    
    print_success(&format!("Added ${:.2} to goal", amount));
    display.show("savings_goals", &vec![updated_goal])?;
    
    Ok(())
}

fn withdraw_money(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Withdraw from Goal");
    
    let goals = api.list_savings_goals(None)?;
    if goals.is_empty() {
        print_error("No savings goals found");
        return Ok(());
    }
    
    display.show("savings_goals", &goals)?;
    
    let goal_idx = select_from_list(
        &goals,
        "Select Goal",
        |g| format!("{} - ${:.2}/{:.2}", g.name, g.current_amount, g.target_amount),
    )?;
    
    let goal = &goals[goal_idx];
    let goal_id = goal.id.unwrap_or(0);
    
    println!("\nAvailable: ${:.2}", goal.current_amount);
    let amount = prompt_float("Amount to Withdraw", None)?;
    
    if amount > goal.current_amount {
        print_error("Insufficient funds in goal");
        return Ok(());
    }
    
    print_info("Withdrawing from goal...");
    
    let update = SavingsGoalUpdate { amount };
    let updated_goal = api.withdraw_from_savings_goal(goal_id, &update)?;
    
    print_success(&format!("Withdrew ${:.2} from goal", amount));
    display.show("savings_goals", &vec![updated_goal])?;
    
    Ok(())
}

fn update_goal(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Update Savings Goal");
    
    let goals = api.list_savings_goals(None)?;
    if goals.is_empty() {
        print_error("No savings goals found");
        return Ok(());
    }
    
    display.show("savings_goals", &goals)?;
    
    let goal_idx = select_from_list(
        &goals,
        "Select Goal to Update",
        |g| format!("{} - ${:.2}/{:.2}", g.name, g.current_amount, g.target_amount),
    )?;
    
    let current = &goals[goal_idx];
    let goal_id = current.id.unwrap_or(0);
    
    println!();
    
    let name = prompt_string("Goal Name", Some(&current.name), false)?;
    let target_amount = prompt_float("Target Amount", Some(current.target_amount))?;
    let current_amount = prompt_float("Current Amount", Some(current.current_amount))?;
    let deadline = select_date_preset()?;
    let description = prompt_string(
        "Description (optional)",
        current.description.as_deref(),
        true,
    )?;
    
    // User selection
    print_info("Fetching available users...");
    let users = api.list_users(Some(true))?;
    
    if users.is_empty() {
        print_error("No users found");
        return Ok(());
    }
    
    display.show("users", &users)?;
    
    let _default_user_idx = users.iter()
        .position(|u| u.id.unwrap_or(0) == current.user_id)
        .unwrap_or(0);
    
    let user_idx = select_from_list(
        &users,
        "Select User",
        |u| format!("{} ({})", u.name, u.email),
    )?;
    
    let user_id = users[user_idx].id.unwrap_or(0);
    
    print_info("Updating savings goal...");
    
    let goal_update = SavingsGoalCreate {
        user_id,
        name,
        target_amount,
        current_amount,
        deadline,
        description: if description.is_empty() { None } else { Some(description) },
    };
    
    let goal = api.update_savings_goal(goal_id, &goal_update)?;
    print_success(&format!("Goal {} updated", goal_id));
    display.show("savings_goals", &vec![goal])?;
    
    Ok(())
}

fn delete_goal(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Delete Savings Goal");
    
    let goals = api.list_savings_goals(None)?;
    if goals.is_empty() {
        print_error("No savings goals found");
        return Ok(());
    }
    
    display.show("savings_goals", &goals)?;
    
    let goal_idx = select_from_list(
        &goals,
        "Select Goal to Delete",
        |g| format!("{} - ${:.2}/{:.2}", g.name, g.current_amount, g.target_amount),
    )?;
    
    let goal = &goals[goal_idx];
    let goal_id = goal.id.unwrap_or(0);
    
    if !prompt_confirm(&format!("Delete goal '{}'?", goal.name), false)? {
        print_info("Cancelled");
        return Ok(());
    }
    
    print_info(&format!("Deleting goal {}...", goal_id));
    api.delete_savings_goal(goal_id)?;
    print_success(&format!("Goal {} deleted", goal_id));
    
    Ok(())
}