use anyhow::Result;
use crate::api::ApiClient;
use crate::config::Config;
use crate::display::Display;
use crate::models::UserCreate;
use crate::ui::*;
use crate::constants::USER_ROLES;

pub fn handle_users(api: &ApiClient, display: &Display, _config: &Config) -> Result<()> {
    loop {
        match show_user_menu()? {
            UserMenuOption::Create => create_user(api, display)?,
            UserMenuOption::List => list_users(api, display)?,
            UserMenuOption::View => view_user(api, display)?,
            UserMenuOption::Update => update_user(api, display)?,
            UserMenuOption::Deactivate => deactivate_user(api, display)?,
            UserMenuOption::Stats => view_user_stats(api, display)?,
            UserMenuOption::Back => break,
        }
    }
    Ok(())
}

fn create_user(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Creating New User");
    
    let name = prompt_string("Name", None, false)?;
    let email = prompt_email("Email", None)?;
    
    let role_options: Vec<String> = USER_ROLES.iter().map(|s| s.to_string()).collect();
    let role_idx = select_with_number("Role", &role_options)?;
    let role = USER_ROLES[role_idx].to_string();
    
    print_info("Creating user...");
    
    let user_create = UserCreate { name, email, role };
    let user = api.create_user(&user_create)?;
    
    print_success(&format!("User created with ID: {}", user.id.unwrap_or(0)));
    display.show("users", &vec![user])?;
    
    Ok(())
}

fn list_users(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Listing Users");
    
    let filter_active = prompt_confirm("Show only active users?", true)?;
    
    print_info("Fetching users...");
    let users = api.list_users(Some(filter_active))?;
    
    if users.is_empty() {
        print_info("No users found");
    } else {
        display.show("users", &users)?;
    }
    
    Ok(())
}

fn view_user(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("View User Details");
    
    // Fetch all users for selection
    let users = api.list_users(None)?;
    if users.is_empty() {
        print_error("No users found");
        return Ok(());
    }
    
    let user_idx = select_from_list(
        &users,
        "Select User",
        |u| format!("{} ({})", u.name, u.email),
    )?;
    
    let user_id = users[user_idx].id.unwrap_or(0);
    
    print_info(&format!("Fetching user {}...", user_id));
    let user = api.get_user(user_id)?;
    display.show("users", &vec![user])?;
    
    Ok(())
}

fn update_user(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Update User");
    
    // Fetch all users for selection
    let users = api.list_users(None)?;
    if users.is_empty() {
        print_error("No users found");
        return Ok(());
    }
    
    let user_idx = select_from_list(
        &users,
        "Select User to Update",
        |u| format!("{} ({})", u.name, u.email),
    )?;
    
    let current = &users[user_idx];
    let user_id = current.id.unwrap_or(0);
    
    display.show("users", &vec![current.clone()])?;
    println!();
    
    let name = prompt_string("Name", Some(&current.name), false)?;
    let email = prompt_email("Email", Some(&current.email))?;
    
    let current_role_idx = USER_ROLES.iter().position(|&r| r == current.role).unwrap_or(0);
    let role_options: Vec<String> = USER_ROLES.iter().map(|s| s.to_string()).collect();
    
    println!("\nCurrent role: {}", USER_ROLES[current_role_idx]);
    let role_idx = select_with_number("Role", &role_options)?;
    let role = USER_ROLES[role_idx].to_string();
    
    print_info("Updating user...");
    
    let user_update = UserCreate { name, email, role };
    let user = api.update_user(user_id, &user_update)?;
    
    print_success(&format!("User {} updated", user_id));
    display.show("users", &vec![user])?;
    
    Ok(())
}

fn deactivate_user(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Deactivate User");
    
    // Fetch active users for selection
    let users = api.list_users(Some(true))?;
    if users.is_empty() {
        print_error("No active users found");
        return Ok(());
    }
    
    let user_idx = select_from_list(
        &users,
        "Select User to Deactivate",
        |u| format!("{} ({})", u.name, u.email),
    )?;
    
    let user = &users[user_idx];
    let user_id = user.id.unwrap_or(0);
    
    display.show("users", &vec![user.clone()])?;
    
    if !prompt_confirm(&format!("Deactivate user '{}'?", user.name), false)? {
        print_info("Cancelled");
        return Ok(());
    }
    
    print_info(&format!("Deactivating user {}...", user_id));
    api.delete_user(user_id)?;
    print_success(&format!("User {} deactivated", user_id));
    
    Ok(())
}

fn view_user_stats(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("User Statistics");
    
    // Fetch all users for selection
    let users = api.list_users(Some(true))?;
    if users.is_empty() {
        print_error("No users found");
        return Ok(());
    }
    
    let user_idx = select_from_list(
        &users,
        "Select User",
        |u| format!("{} ({})", u.name, u.email),
    )?;
    
    let user_id = users[user_idx].id.unwrap_or(0);
    
    let month = if prompt_confirm("Filter by specific month?", false)? {
        Some(select_month_preset()?)
    } else {
        None
    };
    
    print_info(&format!("Fetching statistics for user {}...", user_id));
    let stats = api.get_user_stats(user_id, month.as_deref())?;
    display.show("user_stats", &stats)?;
    
    Ok(())
}