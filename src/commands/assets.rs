use anyhow::Result;
use crate::api::ApiClient;
use crate::display::Display;
use crate::models::{AssetCreate, AssetValueUpdate};
use crate::ui::*;
use crate::constants::ASSET_TYPES;

pub fn handle_assets(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Asset Management");
    
    let asset_options = vec![
        "Create New Asset".to_string(),
        "List Assets".to_string(),
        "View Asset Details".to_string(),
        "Update Asset".to_string(),
        "Update Asset Value".to_string(),
        "Delete Asset".to_string(),
        "View Assets Summary".to_string(),
        "View Depreciation Analysis".to_string(),
        "Back to Main Menu".to_string(),
    ];
    
    loop {
        let selection = select_with_number("Asset Options", &asset_options)?;
        
        match selection {
            0 => create_asset(api, display)?,
            1 => list_assets(api, display)?,
            2 => view_asset(api, display)?,
            3 => update_asset(api, display)?,
            4 => update_asset_value(api, display)?,
            5 => delete_asset(api, display)?,
            6 => view_summary(api, display)?,
            7 => view_depreciation(api, display)?,
            8 => break,
            _ => break,
        }
    }
    
    Ok(())
}

fn create_asset(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Create New Asset");
    
    let name = prompt_string("Asset Name", None, false)?;
    
    let asset_type_options: Vec<String> = ASSET_TYPES.iter().map(|s| s.to_string()).collect();
    let type_idx = select_with_number("Asset Type", &asset_type_options)?;
    let asset_type = ASSET_TYPES[type_idx].to_string();
    
    let purchase_value = prompt_float("Purchase Value", None)?;
    let current_value = prompt_float("Current Value", Some(purchase_value))?;
    let purchase_date = select_date_preset()?;
    
    let description = prompt_string("Description (optional)", None, true)?;
    let location = prompt_string("Location (optional)", None, true)?;
    
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
    
    print_info("Creating asset...");
    
    let asset_create = AssetCreate {
        user_id,
        name,
        asset_type,
        purchase_value,
        current_value,
        purchase_date,
        description: if description.is_empty() { None } else { Some(description) },
        location: if location.is_empty() { None } else { Some(location) },
    };
    
    let asset = api.create_asset(&asset_create)?;
    print_success(&format!("Asset created with ID: {}", asset.id.unwrap_or(0)));
    display.show("assets", &vec![asset])?;
    
    Ok(())
}

fn list_assets(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Assets");
    
    let use_filters = prompt_confirm("Apply filters?", false)?;
    
    let (user_id, asset_type) = if use_filters {
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
        
        let asset_type = if prompt_confirm("Filter by asset type?", false)? {
            let type_options: Vec<String> = ASSET_TYPES.iter().map(|s| s.to_string()).collect();
            let type_idx = select_with_number("Asset Type", &type_options)?;
            Some(ASSET_TYPES[type_idx].to_string())
        } else {
            None
        };
        
        (user_id, asset_type)
    } else {
        (None, None)
    };
    
    print_info("Fetching assets...");
    let assets = api.list_assets(user_id, asset_type.as_deref())?;
    
    if assets.is_empty() {
        print_info("No assets found");
    } else {
        display.show("assets", &assets)?;
    }
    
    Ok(())
}

fn view_asset(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("View Asset Details");
    
    let assets = api.list_assets(None, None)?;
    if assets.is_empty() {
        print_error("No assets found");
        return Ok(());
    }
    
    display.show("assets", &assets)?;
    
    let asset_idx = select_from_list(
        &assets,
        "Select Asset",
        |a| format!("{} ({}) - ${:.2}", a.name, a.asset_type, a.current_value),
    )?;
    
    let asset_id = assets[asset_idx].id.unwrap_or(0);
    
    print_info(&format!("Fetching asset {}...", asset_id));
    let asset = api.get_asset(asset_id)?;
    display.show("assets", &vec![asset])?;
    
    Ok(())
}

fn update_asset(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Update Asset");
    
    let assets = api.list_assets(None, None)?;
    if assets.is_empty() {
        print_error("No assets found");
        return Ok(());
    }
    
    display.show("assets", &assets)?;
    
    let asset_idx = select_from_list(
        &assets,
        "Select Asset to Update",
        |a| format!("{} ({}) - ${:.2}", a.name, a.asset_type, a.current_value),
    )?;
    
    let current = &assets[asset_idx];
    let asset_id = current.id.unwrap_or(0);
    
    println!();
    
    let name = prompt_string("Asset Name", Some(&current.name), false)?;
    
    let current_type_idx = ASSET_TYPES.iter()
        .position(|&t| t == current.asset_type)
        .unwrap_or(0);
    
    println!("\nCurrent type: {}", ASSET_TYPES[current_type_idx]);
    let asset_type_options: Vec<String> = ASSET_TYPES.iter().map(|s| s.to_string()).collect();
    let type_idx = select_with_number("Asset Type", &asset_type_options)?;
    let asset_type = ASSET_TYPES[type_idx].to_string();
    
    let purchase_value = prompt_float("Purchase Value", Some(current.purchase_value))?;
    let current_value = prompt_float("Current Value", Some(current.current_value))?;
    let purchase_date = select_date_preset()?;
    
    let description = prompt_string(
        "Description (optional)",
        current.description.as_deref(),
        true,
    )?;
    let location = prompt_string(
        "Location (optional)",
        current.location.as_deref(),
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
    
    let user_idx = select_from_list(
        &users,
        "Select User",
        |u| format!("{} ({})", u.name, u.email),
    )?;
    
    let user_id = users[user_idx].id.unwrap_or(0);
    
    print_info("Updating asset...");
    
    let asset_update = AssetCreate {
        user_id,
        name,
        asset_type,
        purchase_value,
        current_value,
        purchase_date,
        description: if description.is_empty() { None } else { Some(description) },
        location: if location.is_empty() { None } else { Some(location) },
    };
    
    let asset = api.update_asset(asset_id, &asset_update)?;
    print_success(&format!("Asset {} updated", asset_id));
    display.show("assets", &vec![asset])?;
    
    Ok(())
}

fn update_asset_value(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Update Asset Value");
    
    let assets = api.list_assets(None, None)?;
    if assets.is_empty() {
        print_error("No assets found");
        return Ok(());
    }
    
    display.show("assets", &assets)?;
    
    let asset_idx = select_from_list(
        &assets,
        "Select Asset",
        |a| format!("{} ({}) - ${:.2}", a.name, a.asset_type, a.current_value),
    )?;
    
    let asset = &assets[asset_idx];
    let asset_id = asset.id.unwrap_or(0);
    
    println!("\nCurrent Value: ${:.2}", asset.current_value);
    let current_value = prompt_float("New Current Value", Some(asset.current_value))?;
    
    print_info("Updating asset value...");
    
    let value_update = AssetValueUpdate { current_value };
    let updated_asset = api.update_asset_value(asset_id, &value_update)?;
    
    print_success("Asset value updated");
    display.show("assets", &vec![updated_asset])?;
    
    Ok(())
}

fn delete_asset(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Delete Asset");
    
    let assets = api.list_assets(None, None)?;
    if assets.is_empty() {
        print_error("No assets found");
        return Ok(());
    }
    
    display.show("assets", &assets)?;
    
    let asset_idx = select_from_list(
        &assets,
        "Select Asset to Delete",
        |a| format!("{} ({}) - ${:.2}", a.name, a.asset_type, a.current_value),
    )?;
    
    let asset = &assets[asset_idx];
    let asset_id = asset.id.unwrap_or(0);
    
    if !prompt_confirm(&format!("Delete asset '{}'?", asset.name), false)? {
        print_info("Cancelled");
        return Ok(());
    }
    
    print_info(&format!("Deleting asset {}...", asset_id));
    api.delete_asset(asset_id)?;
    print_success(&format!("Asset {} deleted", asset_id));
    
    Ok(())
}

fn view_summary(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Assets Summary");
    
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
    
    print_info("Fetching assets summary...");
    let summary = api.get_assets_summary(user_id)?;
    display.show("assets_summary", &summary)?;
    
    Ok(())
}

fn view_depreciation(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Depreciation Analysis");
    
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
    
    print_info("Fetching depreciation analysis...");
    let depreciation = api.get_asset_depreciation(user_id)?;
    display.show("asset_depreciation", &depreciation)?;
    
    Ok(())
}