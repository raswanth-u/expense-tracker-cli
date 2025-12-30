use anyhow::Result;
use crate::api::ApiClient;
use crate::config::Config;
use crate::display::Display;
use crate::models::CreditCardCreate;
use crate::ui::*;

pub fn handle_cards(api: &ApiClient, display: &Display, _config: &Config) -> Result<()> {
    loop {
        match show_card_menu()? {
            CardMenuOption::Create => create_card(api, display)?,
            CardMenuOption::List => list_cards(api, display)?,
            CardMenuOption::View => view_card(api, display)?,
            CardMenuOption::Update => update_card(api, display)?,
            CardMenuOption::Deactivate => deactivate_card(api, display)?,
            CardMenuOption::Statement => card_statement(api, display)?,
            CardMenuOption::Utilization => card_utilization(api, display)?,
            CardMenuOption::Summary => cards_summary(api, display)?,
            CardMenuOption::Back => break,
        }
    }
    Ok(())
}

fn create_card(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Creating New Credit Card");
    
    let card_name = prompt_string("Card Name", None, false)?;
    
    let last_four = loop {
        let input = prompt_string("Last 4 digits", None, false)?;
        if input.len() == 4 && input.chars().all(|c| c.is_numeric()) {
            break input;
        }
        print_error("Must be exactly 4 digits. Please try again.");
    };
    
    let credit_limit = prompt_float("Credit Limit", None)?;
    let billing_day = prompt_int("Billing Day (1-31)", None, Some(1), Some(31))?;
    
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
    
    print_info("Creating credit card...");
    
    let card_create = CreditCardCreate {
        user_id,
        card_name,
        last_four,
        credit_limit,
        billing_day,
    };
    
    let card = api.create_credit_card(&card_create)?;
    print_success(&format!("Credit card created with ID: {}", card.id.unwrap_or(0)));
    display.show("credit_cards", &vec![card])?;
    
    Ok(())
}

fn list_cards(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Listing Credit Cards");
    
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
    
    print_info("Fetching credit cards...");
    let cards = api.list_credit_cards(user_id)?;
    
    if cards.is_empty() {
        print_info("No credit cards found");
    } else {
        display.show("credit_cards", &cards)?;
    }
    
    Ok(())
}

fn view_card(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("View Credit Card Details");
    
    // Fetch all cards for selection
    let cards = api.list_credit_cards(None)?;
    if cards.is_empty() {
        print_error("No credit cards found");
        return Ok(());
    }
    
    display.show("credit_cards", &cards)?;
    
    let card_idx = select_from_list(
        &cards,
        "Select Credit Card",
        |c| format!("{} (****{})", c.card_name, c.last_four),
    )?;
    
    let card_id = cards[card_idx].id.unwrap_or(0);
    
    print_info(&format!("Fetching credit card {}...", card_id));
    let card = api.get_credit_card(card_id)?;
    display.show("credit_cards", &vec![card])?;
    
    Ok(())
}

fn update_card(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Update Credit Card");
    
    // Fetch all cards for selection
    let cards = api.list_credit_cards(None)?;
    if cards.is_empty() {
        print_error("No credit cards found");
        return Ok(());
    }
    
    display.show("credit_cards", &cards)?;
    
    let card_idx = select_from_list(
        &cards,
        "Select Credit Card to Update",
        |c| format!("{} (****{})", c.card_name, c.last_four),
    )?;
    
    let current = &cards[card_idx];
    let card_id = current.id.unwrap_or(0);
    
    display.show("credit_cards", &vec![current.clone()])?;
    println!();
    
    let card_name = prompt_string("Card Name", Some(&current.card_name), false)?;
    
    let last_four = loop {
        let input = prompt_string("Last 4 digits", Some(&current.last_four), false)?;
        if input.len() == 4 && input.chars().all(|c| c.is_numeric()) {
            break input;
        }
        print_error("Must be exactly 4 digits. Please try again.");
    };
    
    let credit_limit = prompt_float("Credit Limit", Some(current.credit_limit))?;
    let billing_day = prompt_int("Billing Day (1-31)", Some(current.billing_day), Some(1), Some(31))?;
    
    // User Selection
    print_info("Fetching available users...");
    let users = api.list_users(Some(true))?;
    
    if users.is_empty() {
        print_error("No users found.");
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
    
    print_info("Updating credit card...");
    
    let card_update = CreditCardCreate {
        user_id,
        card_name,
        last_four,
        credit_limit,
        billing_day,
    };
    
    let card = api.update_credit_card(card_id, &card_update)?;
    print_success(&format!("Credit card {} updated", card_id));
    display.show("credit_cards", &vec![card])?;
    
    Ok(())
}

fn deactivate_card(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Deactivate Credit Card");
    
    // Fetch active cards for selection
    let cards = api.list_credit_cards(None)?;
    if cards.is_empty() {
        print_error("No credit cards found");
        return Ok(());
    }
    
    display.show("credit_cards", &cards)?;
    
    let card_idx = select_from_list(
        &cards,
        "Select Credit Card to Deactivate",
        |c| format!("{} (****{})", c.card_name, c.last_four),
    )?;
    
    let card = &cards[card_idx];
    let card_id = card.id.unwrap_or(0);
    
    display.show("credit_cards", &vec![card.clone()])?;
    
    if !prompt_confirm(&format!("Deactivate card '{}'?", card.card_name), false)? {
        print_info("Cancelled");
        return Ok(());
    }
    
    print_info(&format!("Deactivating credit card {}...", card_id));
    api.delete_credit_card(card_id)?;
    print_success(&format!("Credit card {} deactivated", card_id));
    
    Ok(())
}

fn card_statement(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Credit Card Statement");
    
    // Fetch all cards for selection
    let cards = api.list_credit_cards(None)?;
    if cards.is_empty() {
        print_error("No credit cards found");
        return Ok(());
    }
    
    display.show("credit_cards", &cards)?;
    
    let card_idx = select_from_list(
        &cards,
        "Select Credit Card",
        |c| format!("{} (****{})", c.card_name, c.last_four),
    )?;
    
    let card_id = cards[card_idx].id.unwrap_or(0);
    
    let month = select_month_preset()?;
    
    print_info(&format!("Fetching statement for card {} - {}...", card_id, month));
    let statement = api.get_credit_card_statement(card_id, &month)?;
    display.show("card_statement", &statement)?;
    
    Ok(())
}

fn card_utilization(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Credit Card Utilization");
    
    // Fetch all cards for selection
    let cards = api.list_credit_cards(None)?;
    if cards.is_empty() {
        print_error("No credit cards found");
        return Ok(());
    }
    
    display.show("credit_cards", &cards)?;
    
    let card_idx = select_from_list(
        &cards,
        "Select Credit Card",
        |c| format!("{} (****{})", c.card_name, c.last_four),
    )?;
    
    let card_id = cards[card_idx].id.unwrap_or(0);
    
    let months = prompt_int("Number of months to analyze", Some(3), Some(1), Some(12))?;
    
    print_info(&format!("Fetching utilization for card {} (last {} months)...", card_id, months));
    let utilization = api.get_credit_card_utilization(card_id, months)?;
    display.show("card_utilization", &utilization)?;
    
    Ok(())
}

fn cards_summary(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Credit Cards Summary");
    
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
    
    let month = select_month_preset()?;
    
    print_info("Fetching credit cards summary...");
    let summary = api.get_all_cards_summary(user_id, Some(&month))?;
    display.show("cards_summary", &summary)?;
    
    Ok(())
}