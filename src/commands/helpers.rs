//! # Common Helper Functions
//!
//! Shared utilities used across command modules to reduce code duplication.

use anyhow::Result;
use crate::api::ApiClient;
use crate::display::Display;
use crate::ui::*;
use crate::constants::PAYMENT_METHODS;

// ============================================================================
// USER SELECTION
// ============================================================================

/// Select a user from the list of active users
/// Returns the user_id or an error if no users exist
pub fn select_user(api: &ApiClient, display: &Display) -> Result<i32> {
    print_info("Fetching users...");
    let users = api.list_users(Some(true))?;
    
    if users.is_empty() {
        print_error("No active users found. Please create a user first.");
        anyhow::bail!("No users available");
    }
    
    display.show("users", &users)?;
    
    let idx = select_from_list(
        &users,
        "Select User",
        |u| format!("{} ({})", u.name, u.email),
    )?;
    
    Ok(users[idx].id.unwrap_or(0))
}

/// Select a user from the list (optional - returns None if user cancels)
pub fn select_user_optional(api: &ApiClient, display: &Display) -> Result<Option<i32>> {
    let users = api.list_users(Some(true))?;
    if users.is_empty() {
        print_info("No active users found");
        return Ok(None);
    }
    
    display.show("users", &users)?;
    let idx = select_from_list(
        &users,
        "Select User",
        |u| format!("{} ({})", u.name, u.email),
    )?;
    
    Ok(users[idx].id)
}

// ============================================================================
// PAYMENT METHOD SELECTION
// ============================================================================

/// Payment method selection result
pub struct PaymentSelection {
    pub method: String,
    pub credit_card_id: Option<i32>,
    pub savings_account_id: Option<i32>,
}

/// Select payment method with associated card/account if applicable
pub fn select_payment_method(api: &ApiClient, display: &Display) -> Result<PaymentSelection> {
    let options: Vec<String> = PAYMENT_METHODS.iter().map(|s| s.to_string()).collect();
    let idx = select_with_number("Payment Method", &options)?;
    let method = PAYMENT_METHODS[idx].to_string();
    
    let mut credit_card_id = None;
    let mut savings_account_id = None;
    
    if method == "credit_card" {
        credit_card_id = select_credit_card_optional(api, display)?;
        if credit_card_id.is_none() {
            print_error("No credit card selected. Please add a credit card first.");
            anyhow::bail!("Credit card required");
        }
    } else if method == "savings_account" {
        savings_account_id = select_savings_account_optional(api, display)?;
        if savings_account_id.is_none() {
            print_error("No savings account selected. Please add a savings account first.");
            anyhow::bail!("Savings account required");
        }
    }
    
    Ok(PaymentSelection {
        method,
        credit_card_id,
        savings_account_id,
    })
}

// ============================================================================
// CREDIT CARD SELECTION
// ============================================================================

/// Select a credit card (optional)
pub fn select_credit_card_optional(api: &ApiClient, display: &Display) -> Result<Option<i32>> {
    print_info("Fetching credit cards...");
    let cards = api.list_credit_cards(None)?;
    
    if cards.is_empty() {
        return Ok(None);
    }
    
    display.show("credit_cards", &cards)?;
    let idx = select_from_list(
        &cards,
        "Select Credit Card",
        |c| format!("{} (****{})", c.card_name, c.last_four),
    )?;
    
    Ok(cards[idx].id)
}

// ============================================================================
// SAVINGS ACCOUNT SELECTION
// ============================================================================

/// Select a savings account (optional)
pub fn select_savings_account_optional(api: &ApiClient, display: &Display) -> Result<Option<i32>> {
    print_info("Fetching savings accounts...");
    let accounts = api.list_savings_accounts(None)?;
    
    if accounts.is_empty() {
        return Ok(None);
    }
    
    display.show("savings_accounts", &accounts)?;
    let idx = select_from_list(
        &accounts,
        "Select Savings Account",
        |a| format!("{} - {} ({})", a.account_name, a.bank_name, format_currency(a.current_balance)),
    )?;
    
    Ok(accounts[idx].id)
}

/// Select a savings account (required)
pub fn select_savings_account(api: &ApiClient, display: &Display) -> Result<i32> {
    match select_savings_account_optional(api, display)? {
        Some(id) => Ok(id),
        None => {
            print_error("No savings accounts found. Please create one first.");
            anyhow::bail!("No savings accounts available");
        }
    }
}

// ============================================================================
// DATE HELPERS
// ============================================================================

/// Get current date as YYYY-MM-DD string
pub fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

/// Get current month as YYYY-MM string
pub fn current_month() -> String {
    chrono::Local::now().format("%Y-%m").to_string()
}

// ============================================================================
// DISPLAY HELPERS
// ============================================================================

/// Display a list of items if not empty, otherwise show "no items" message
pub fn display_or_empty<T: serde::Serialize>(
    display: &Display,
    entity_type: &str,
    items: &Vec<T>,
    empty_message: &str,
) -> Result<()> {
    if items.is_empty() {
        print_info(empty_message);
    } else {
        display.show(entity_type, items)?;
    }
    Ok(())
}

// ============================================================================
// TAG SELECTION
// ============================================================================

/// Select tags interactively
pub fn select_tags() -> Result<String> {
    use crate::constants::COMMON_TAGS;
    
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
