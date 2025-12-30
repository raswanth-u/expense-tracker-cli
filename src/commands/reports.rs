use anyhow::Result;
use crate::api::ApiClient;
use crate::config::Config;
use crate::display::Display;
use crate::ui::*;

pub fn handle_reports(api: &ApiClient, display: &Display, _config: &Config) -> Result<()> {
    loop {
        match show_report_menu()? {
            ReportMenuOption::Monthly => monthly_report(api, display)?,
            ReportMenuOption::Family => family_summary(api, display)?,
            ReportMenuOption::Category => category_analysis(api, display)?,
            ReportMenuOption::Trends => spending_trends(api, display)?,
            ReportMenuOption::Payments => payment_analysis(api, display)?,
            ReportMenuOption::Export => export_data(api, display)?,
            ReportMenuOption::Back => break,
        }
    }
    Ok(())
}

fn monthly_report(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Monthly Report");
    
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
    
    print_info(&format!("Generating monthly report for {}...", month));
    let report = api.get_monthly_report(&month, user_id)?;
    display.show("monthly_report", &report)?;
    
    Ok(())
}

fn family_summary(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Family Summary");
    
    let month = select_month_preset()?;
    
    print_info(&format!("Generating family summary for {}...", month));
    let report = api.get_family_summary(&month)?;
    display.show("family_summary", &report)?;
    
    Ok(())
}

fn category_analysis(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Category Analysis");
    
    let category = crate::commands::expenses::select_category()?;
    
    println!("\nFrom Date:");
    let from_date = select_date_preset()?;
    
    println!("\nTo Date:");
    let to_date = select_date_preset()?;
    
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
    
    print_info(&format!("Analyzing category: {}...", category));
    let analysis = api.get_category_analysis(&category, &from_date, &to_date, user_id)?;
    display.show("category_analysis", &analysis)?;
    
    Ok(())
}

fn spending_trends(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Spending Trends");
    
    let months = prompt_int("Number of months to analyze", Some(6), Some(1), Some(12))?;
    
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
    
    print_info(&format!("Analyzing spending trends (last {} months)...", months));
    let trends = api.get_spending_trends(months, user_id)?;
    display.show("spending_trends", &trends)?;
    
    Ok(())
}

fn payment_analysis(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Payment Method Analysis");
    
    println!("\nFrom Date:");
    let from_date = select_date_preset()?;
    
    println!("\nTo Date:");
    let to_date = select_date_preset()?;
    
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
    
    print_info("Analyzing payment methods...");
    let analysis = api.get_payment_method_analysis(&from_date, &to_date, user_id)?;
    display.show("payment_analysis", &analysis)?;
    
    Ok(())
}

fn export_data(api: &ApiClient, display: &Display) -> Result<()> {
    print_section_header("Export Data");
    
    println!("\nFrom Date:");
    let from_date = select_date_preset()?;
    
    println!("\nTo Date:");
    let to_date = select_date_preset()?;
    
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
    
    let format_options = vec!["JSON".to_string(), "CSV".to_string()];
    let format_idx = select_with_number("Export Format", &format_options)?;
    
    let format = if format_idx == 0 { "json" } else { "csv" };
    
    let default_filename = format!("expenses_{}_{}.{}", from_date, to_date, format);
    let filename = prompt_string("Output filename", Some(&default_filename), false)?;
    
    print_info(&format!("Exporting expenses ({} format)...", format));
    
    if format == "csv" {
        let csv_data = api.export_expenses_csv(&from_date, &to_date, user_id)?;
        std::fs::write(&filename, csv_data)?;
    } else {
        let json_data = api.export_expenses_json(&from_date, &to_date, user_id)?;
        let json_str = serde_json::to_string_pretty(&json_data)?;
        std::fs::write(&filename, json_str)?;
    }
    
    print_success(&format!("Exported to: {}", filename));
    
    Ok(())
}