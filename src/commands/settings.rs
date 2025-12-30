use anyhow::Result;
use crate::api::ApiClient;
use crate::config::Config;
use crate::display::Display;
use crate::ui::*;

pub fn handle_settings(api: &ApiClient, _display: &Display, config: &Config) -> Result<()> {
    loop {
        match show_settings_menu()? {
            SettingsMenuOption::View => view_settings(config)?,
            SettingsMenuOption::ChangeDefaultUser => change_default_user(api, config)?,
            SettingsMenuOption::ChangeDefaultCategory => change_default_category(config)?,
            SettingsMenuOption::ToggleRecentValues => toggle_recent_values(config)?,
            SettingsMenuOption::Back => break,
        }
    }
    Ok(())
}

fn view_settings(config: &Config) -> Result<()> {
    print_section_header("Current Settings");
    
    println!("API Configuration:");
    println!("  Base URL: {}", config.api.base_url);
    println!("  SSL Verify: {}", !config.api.skip_ssl_verify);
    println!();
    
    println!("Display Configuration:");
    println!("  Python Path: {}", if config.display.python_path.is_empty() { 
        "system default" 
    } else { 
        &config.display.python_path 
    });
    println!("  Display Module: {}", config.display.display_module);
    println!();
    
    println!("Defaults:");
    println!("  Default User ID: {}", config.defaults.default_user_id);
    println!("  Date Format: {}", config.defaults.date_format);
    println!();
    
    print_info("Settings are stored in config/config.toml");
    
    Ok(())
}

fn change_default_user(api: &ApiClient, config: &Config) -> Result<()> {
    print_section_header("Change Default User");
    
    print_info("Fetching available users...");
    let users = api.list_users(Some(true))?;
    
    if users.is_empty() {
        print_error("No users found");
        return Ok(());
    }
    
    let display = Display::new(config);
    display.show("users", &users)?;
    
    let user_idx = select_from_list(
        &users,
        "Select Default User",
        |u| format!("{} ({})", u.name, u.email),
    )?;
    
    let user_id = users[user_idx].id.unwrap_or(0);
    
    print_info(&format!("To change default user to ID {}, edit config/config.toml:", user_id));
    println!("\n[defaults]");
    println!("default_user_id = {}", user_id);
    println!();
    
    Ok(())
}

fn change_default_category(_config: &Config) -> Result<()> {
    print_section_header("Change Default Category");
    
    let category = crate::commands::expenses::select_category()?;
    
    print_info(&format!("To set '{}' as default category, add to config/config.toml:", category));
    println!("\n[defaults]");
    println!("default_category = \"{}\"", category);
    println!();
    
    Ok(())
}

fn toggle_recent_values(_config: &Config) -> Result<()> {
    print_section_header("Toggle Recent Values");
    
    print_info("To enable/disable recent values feature, add to config/config.toml:");
    println!("\n[ui]");
    println!("show_recent_values = true  # or false");
    println!("recent_values_count = 5");
    println!();
    
    Ok(())
}