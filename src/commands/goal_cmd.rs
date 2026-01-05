//! Savings goal command handlers

use anyhow::{Result, Context};
use serde_json::json;

use crate::api::ApiClient;
use crate::cli::GoalCommands;
use crate::display::Display;
use crate::models::{SavingsGoalCreate, SavingsGoalUpdate};

pub fn handle_goal_command(api: &ApiClient, display: &Display, cmd: GoalCommands, json_output: bool) -> Result<()> {
    match cmd {
        GoalCommands::Add { name, target, user_id, deadline, current, description, tags } => {
            add_goal(api, display, name, target, user_id, deadline, current, description, tags, json_output)
        }
        GoalCommands::List { user_id, active: _ } => {
            list_goals(api, display, user_id, json_output)
        }
        GoalCommands::View { id } => {
            view_goal(api, display, id, json_output)
        }
        GoalCommands::Update { id, target, deadline, active: _ } => {
            update_goal(api, display, id, target, deadline, json_output)
        }
        GoalCommands::Delete { id, force } => {
            delete_goal(api, id, force, json_output)
        }
        GoalCommands::Contribute { id, amount } => {
            contribute_to_goal(api, id, amount, json_output)
        }
        GoalCommands::Progress { user_id } => {
            show_progress(api, user_id, json_output)
        }
    }
}

fn add_goal(
    api: &ApiClient,
    display: &Display,
    name: String,
    target: f64,
    user_id: i32,
    deadline: String,
    current: f64,
    description: Option<String>,
    tags: Option<String>,
    json_output: bool,
) -> Result<()> {
    let goal = SavingsGoalCreate {
        user_id,
        name,
        target_amount: target,
        current_amount: current,
        deadline,
        description,
        tags,
    };
    let created = api.create_savings_goal(&goal)?;
    
    if json_output {
        println!("{}", json!({
            "success": true,
            "message": "Savings goal created",
            "goal": created
        }));
    } else {
        println!("✓ Savings goal created with ID: {}", created.id.unwrap_or(0));
        display.show("savings_goals", &vec![created])?;
    }
    Ok(())
}

fn list_goals(api: &ApiClient, display: &Display, user_id: Option<i32>, json_output: bool) -> Result<()> {
    // API only supports user_id filter
    let goals = api.list_savings_goals(user_id)?;
    
    if json_output {
        let total_target: f64 = goals.iter().map(|g| g.target_amount).sum();
        let total_current: f64 = goals.iter().map(|g| g.current_amount).sum();
        let progress = if total_target > 0.0 { (total_current / total_target) * 100.0 } else { 0.0 };
        
        println!("{}", json!({
            "success": true,
            "count": goals.len(),
            "goals": goals,
            "summary": {
                "total_target": total_target,
                "total_current": total_current,
                "overall_progress_percent": progress
            }
        }));
        return Ok(());
    }
    
    if goals.is_empty() {
        println!("No savings goals found.");
        return Ok(());
    }
    println!("Found {} goal(s)", goals.len());
    display.show("savings_goals", &goals)?;
    
    // Calculate totals in frontend
    let total_target: f64 = goals.iter().map(|g| g.target_amount).sum();
    let total_current: f64 = goals.iter().map(|g| g.current_amount).sum();
    let progress = if total_target > 0.0 { (total_current / total_target) * 100.0 } else { 0.0 };
    println!("\n📊 Overall Progress: ${:.2} / ${:.2} ({:.1}%)", total_current, total_target, progress);
    Ok(())
}

fn view_goal(api: &ApiClient, display: &Display, id: i32, json_output: bool) -> Result<()> {
    let progress = api.get_savings_goal_progress(id)?;
    
    if json_output {
        println!("{}", json!({
            "success": true,
            "progress": progress
        }));
    } else {
        display.show("savings_goal_progress", &progress)?;
    }
    Ok(())
}

fn update_goal(
    api: &ApiClient,
    display: &Display,
    id: i32,
    target: Option<f64>,
    deadline: Option<String>,
    json_output: bool,
) -> Result<()> {
    // Get current goal from list
    let goals = api.list_savings_goals(None)?;
    let current = goals.into_iter()
        .find(|g| g.id == Some(id))
        .context(format!("Goal {} not found", id))?;
    let update = SavingsGoalCreate {
        user_id: current.user_id,
        name: current.name,
        target_amount: target.unwrap_or(current.target_amount),
        current_amount: current.current_amount,
        deadline: deadline.unwrap_or(current.deadline),
        description: current.description,
        tags: current.tags,
    };
    let updated = api.update_savings_goal(id, &update)?;
    
    if json_output {
        println!("{}", json!({
            "success": true,
            "message": format!("Goal {} updated", id),
            "goal": updated
        }));
    } else {
        println!("✓ Goal {} updated", id);
        display.show("savings_goals", &vec![updated])?;
    }
    Ok(())
}

fn delete_goal(api: &ApiClient, id: i32, force: bool, json_output: bool) -> Result<()> {
    if !force {
        // Get goal from list
        let goals = api.list_savings_goals(None)?;
        if let Some(goal) = goals.into_iter().find(|g| g.id == Some(id)) {
            if json_output {
                println!("{}", json!({
                    "success": false,
                    "error": "Confirmation required",
                    "message": "Use --force to confirm deletion",
                    "goal": {
                        "id": id,
                        "name": goal.name,
                        "current_amount": goal.current_amount,
                        "target_amount": goal.target_amount
                    }
                }));
                return Ok(());
            }
            println!("About to delete goal: {} (${:.2} / ${:.2})", 
                goal.name, goal.current_amount, goal.target_amount);
        }
        anyhow::bail!("Use --force to confirm deletion");
    }
    api.delete_savings_goal(id)?;
    
    if json_output {
        println!("{}", json!({
            "success": true,
            "message": format!("Goal {} deleted", id),
            "deleted_id": id
        }));
    } else {
        println!("✓ Goal {} deleted", id);
    }
    Ok(())
}

fn contribute_to_goal(api: &ApiClient, id: i32, amount: f64, json_output: bool) -> Result<()> {
    let update = SavingsGoalUpdate { amount };
    let result = api.add_to_savings_goal(id, &update)?;
    let progress = (result.current_amount / result.target_amount) * 100.0;
    
    if json_output {
        println!("{}", json!({
            "success": true,
            "message": format!("Added ${:.2} to goal {}", amount, id),
            "goal": result,
            "contribution": {
                "amount": amount,
                "new_current_amount": result.current_amount,
                "progress_percent": progress
            }
        }));
    } else {
        println!("✓ Added ${:.2} to goal {}", amount, id);
        println!("   New Amount: ${:.2}", result.current_amount);
        println!("   Progress: {:.1}%", progress);
    }
    Ok(())
}

fn show_progress(api: &ApiClient, user_id: Option<i32>, json_output: bool) -> Result<()> {
    // API only supports user_id filter
    let goals = api.list_savings_goals(user_id)?;
    // Filter active goals in frontend
    let active_goals: Vec<_> = goals.into_iter()
        .filter(|g| g.is_active.unwrap_or(true))
        .collect();
    
    if json_output {
        let progress_data: Vec<_> = active_goals.iter().map(|goal| {
            let progress = (goal.current_amount / goal.target_amount) * 100.0;
            let remaining = goal.target_amount - goal.current_amount;
            json!({
                "id": goal.id,
                "name": goal.name,
                "current_amount": goal.current_amount,
                "target_amount": goal.target_amount,
                "remaining": remaining,
                "progress_percent": progress,
                "deadline": goal.deadline
            })
        }).collect();
        
        println!("{}", json!({
            "success": true,
            "count": active_goals.len(),
            "goals_progress": progress_data
        }));
        return Ok(());
    }
    
    if active_goals.is_empty() {
        println!("No active goals found.");
        return Ok(());
    }
    
    println!("📊 Savings Goals Progress\n");
    for goal in &active_goals {
        let progress = (goal.current_amount / goal.target_amount) * 100.0;
        let remaining = goal.target_amount - goal.current_amount;
        let bar_len = 20;
        let filled = ((progress / 100.0) * bar_len as f64) as usize;
        let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(bar_len - filled));
        
        println!("{}: ${:.2} / ${:.2}", goal.name, goal.current_amount, goal.target_amount);
        println!("   {} {:.1}% (${:.2} remaining)", bar, progress, remaining);
        println!("   Deadline: {}", goal.deadline);
        println!();
    }
    Ok(())
}
