use anyhow::Result;
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
    
    let selection = select_with_number("Select Date", &options)?;
    
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
    
    let selection = select_with_number("Select Month", &options)?;
    
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
        .map(|item| display_fn(item))
        .collect();
    
    select_with_number(prompt, &display_items)
}

pub fn select_multiple_from_list(
    items: &[String],
    prompt: &str,
) -> Result<Vec<usize>> {
    println!("\n{}", prompt);
    println!("Enter numbers separated by commas (e.g., 1,3,5) or 'all' for all items:\n");
    
    for (i, item) in items.iter().enumerate() {
        println!("  {}. {}", i + 1, item);
    }
    
    loop {
        use crate::ui::prompts::prompt_string;
        let input = prompt_string("\nYour selection", None, false)?;
        
        if input.trim().to_lowercase() == "all" {
            return Ok((0..items.len()).collect());
        }
        
        let selections: Result<Vec<usize>, _> = input
            .split(',')
            .map(|s| s.trim().parse::<usize>())
            .collect();
        
        match selections {
            Ok(nums) => {
                let valid: Vec<usize> = nums
                    .into_iter()
                    .filter(|&n| n > 0 && n <= items.len())
                    .map(|n| n - 1)
                    .collect();
                
                if valid.is_empty() {
                    println!("❌ No valid selections. Please try again.");
                    continue;
                }
                
                return Ok(valid);
            }
            Err(_) => {
                println!("❌ Invalid input. Please enter numbers separated by commas.");
                continue;
            }
        }
    }
}

// New helper function for number-based selection
pub fn select_with_number(prompt: &str, items: &[String]) -> Result<usize> {
    println!("\n{}", prompt);
    println!();
    
    for (i, item) in items.iter().enumerate() {
        println!("  {}. {}", i + 1, item);
    }
    
    loop {
        use crate::ui::prompts::prompt_int;
        let selection = prompt_int("\nEnter number", None, Some(1), Some(items.len() as i32))?;
        return Ok((selection - 1) as usize);
    }
}