use anyhow::Result;
use dialoguer::{Input, Confirm};

pub fn prompt_string(prompt: &str, default: Option<&str>, allow_empty: bool) -> Result<String> {
    loop {
        let mut input = Input::<String>::new().with_prompt(prompt);
        
        if let Some(def) = default {
            input = input.with_initial_text(def);
        }
        
        if allow_empty {
            input = input.allow_empty(true);
        }
        
        match input.interact_text() {
            Ok(value) => return Ok(value),
            Err(e) => {
                println!("❌ Invalid input: {}. Please try again.", e);
                continue;
            }
        }
    }
}

pub fn prompt_float(prompt: &str, default: Option<f64>) -> Result<f64> {
    loop {
        let mut input = Input::<String>::new().with_prompt(prompt);
        
        if let Some(def) = default {
            input = input.with_initial_text(&def.to_string());
        }
        
        match input.interact_text() {
            Ok(value) => {
                match value.parse::<f64>() {
                    Ok(num) if num > 0.0 => return Ok(num),
                    Ok(_) => println!("❌ Amount must be greater than 0. Please try again."),
                    Err(_) => println!("❌ Invalid number. Please try again."),
                }
            }
            Err(e) => {
                println!("❌ Invalid input: {}. Please try again.", e);
            }
        }
    }
}

pub fn prompt_int(prompt: &str, default: Option<i32>, min: Option<i32>, max: Option<i32>) -> Result<i32> {
    loop {
        let mut input = Input::<String>::new().with_prompt(prompt);
        
        if let Some(def) = default {
            input = input.with_initial_text(&def.to_string());
        }
        
        match input.interact_text() {
            Ok(value) => {
                match value.parse::<i32>() {
                    Ok(num) => {
                        if let Some(min_val) = min {
                            if num < min_val {
                                println!("❌ Value must be at least {}. Please try again.", min_val);
                                continue;
                            }
                        }
                        if let Some(max_val) = max {
                            if num > max_val {
                                println!("❌ Value must be at most {}. Please try again.", max_val);
                                continue;
                            }
                        }
                        return Ok(num);
                    }
                    Err(_) => println!("❌ Invalid number. Please try again."),
                }
            }
            Err(e) => {
                println!("❌ Invalid input: {}. Please try again.", e);
            }
        }
    }
}

pub fn prompt_confirm(prompt: &str, default: bool) -> Result<bool> {
    Ok(Confirm::new()
        .with_prompt(prompt)
        .default(default)
        .interact()?)
}

pub fn prompt_email(prompt: &str, default: Option<&str>) -> Result<String> {
    loop {
        let mut input = Input::<String>::new().with_prompt(prompt);
        
        if let Some(def) = default {
            input = input.with_initial_text(def);
        }
        
        match input.interact_text() {
            Ok(value) => {
                if value.contains('@') && value.contains('.') {
                    return Ok(value);
                } else {
                    println!("❌ Invalid email format. Please try again.");
                }
            }
            Err(e) => {
                println!("❌ Invalid input: {}. Please try again.", e);
            }
        }
    }
}