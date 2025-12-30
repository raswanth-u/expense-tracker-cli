use anyhow::Result;
use dialoguer::Select;
use chrono::{Local, Datelike};

pub fn select_date_preset() -> Result<String> {
    let today = Local::now();
    let yesterday = today - chrono::Duration::days(1);
    let start_of_month = Local::now().with_day(1).unwrap();
    
    let options = vec![
        format!("Today ({})", today.format("%Y-%m-%d")),
        format!("Yesterday ({})", yesterday.format("%Y-%m-%d")),
        format!("Start of Month ({})", start_of_month.format("%Y-%m-%d")),
        "Custom Date".to_string(),
    ];
    
    let selection = Select::new()
        .with_prompt("Select Date")
        .items(&options)
        .default(0)
        .interact()?;
    
    Ok(match selection {
        0 => today.format("%Y-%m-%d").to_string(),
        1 => yesterday.format("%Y-%m-%d").to_string(),
        2 => start_of_month.format("%Y-%m-%d").to_string(),
        3 => {
            use crate::ui::prompts::prompt_string;
            prompt_string("Enter date (YYYY-MM-DD)", None, false)?
        }
        _ => today.format("%Y-%m-%d").to_string(),
    })
}

pub fn select_month_preset() -> Result<String> {
    let today = Local::now();
    let current_month = today.format("%Y-%m").to_string();
    let last_month = (today - chrono::Duration::days(30)).format("%Y-%m").to_string();
    let two_months_ago = (today - chrono::Duration::days(60)).format("%Y-%m").to_string();
    
    let options = vec![
        format!("Current Month ({})", current_month),
        format!("Last Month ({})", last_month),
        format!("2 Months Ago ({})", two_months_ago),
        "Custom Month".to_string(),
    ];
    
    let selection = Select::new()
        .with_prompt("Select Month")
        .items(&options)
        .default(0)
        .interact()?;
    
    Ok(match selection {
        0 => current_month,
        1 => last_month,
        2 => two_months_ago,
        3 => {
            use crate::ui::prompts::prompt_string;
            prompt_string("Enter month (YYYY-MM)", None, false)?
        }
        _ => current_month,
    })
}

pub fn select_from_list<T>(
    items: &[T],
    prompt: &str,
    display_fn: impl Fn(&T) -> String,
) -> Result<usize> {
    let display_items: Vec<String> = items
        .iter()
        .enumerate()
        .map(|(i, item)| format!("{}. {}", i + 1, display_fn(item)))
        .collect();
    
    Select::new()
        .with_prompt(prompt)
        .items(&display_items)
        .default(0)
        .interact()
        .map_err(|e| anyhow::anyhow!("Selection error: {}", e))
}

pub fn select_multiple_from_list(
    items: &[String],
    prompt: &str,
) -> Result<Vec<usize>> {
    use dialoguer::MultiSelect;
    
    MultiSelect::new()
        .with_prompt(prompt)
        .items(items)
        .interact()
        .map_err(|e| anyhow::anyhow!("Selection error: {}", e))
}