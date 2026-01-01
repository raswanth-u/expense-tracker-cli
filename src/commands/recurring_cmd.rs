//! Recurring expense command handlers

use anyhow::Result;
use serde_json::json;

use crate::api::ApiClient;
use crate::cli::RecurringCommands;
use crate::display::Display;
use crate::models::RecurringExpenseTemplateCreate;

pub fn handle_recurring_command(api: &ApiClient, display: &Display, cmd: RecurringCommands, json_output: bool) -> Result<()> {
    match cmd {
        RecurringCommands::Add { amount, category, user_id, frequency, start_date, description, interval, day_of_week, day_of_month, end_date, tags } => {
            let template = RecurringExpenseTemplateCreate {
                user_id,
                amount,
                category,
                description,
                frequency,
                interval,
                day_of_week,
                day_of_month,
                month_of_year: None,
                start_date,
                end_date,
                tags,
            };
            let created = api.create_recurring_template(&template)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "id": created.id,
                    "template": created
                }))?);
                return Ok(());
            }
            
            println!("✓ Recurring expense template created with ID: {}", created.id.unwrap_or(0));
            display.show("recurring_templates", &vec![created])?;
            Ok(())
        }
        RecurringCommands::List { user_id, active: _ } => {
            let templates = api.list_recurring_templates(user_id, None)?;
            
            // Calculate totals in frontend
            let monthly_total: f64 = templates.iter()
                .filter(|t| t.frequency == "monthly")
                .map(|t| t.amount)
                .sum();
            let weekly_total: f64 = templates.iter()
                .filter(|t| t.frequency == "weekly")
                .map(|t| t.amount * 4.33)
                .sum();
            let daily_total: f64 = templates.iter()
                .filter(|t| t.frequency == "daily")
                .map(|t| t.amount * 30.0)
                .sum();
            let yearly_total: f64 = templates.iter()
                .filter(|t| t.frequency == "yearly")
                .map(|t| t.amount / 12.0)
                .sum();
            let estimated_monthly = monthly_total + weekly_total + daily_total + yearly_total;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "count": templates.len(),
                    "templates": templates,
                    "summary": {
                        "monthly_total": monthly_total,
                        "weekly_total": weekly_total,
                        "daily_total": daily_total,
                        "yearly_total": yearly_total,
                        "estimated_monthly": estimated_monthly
                    }
                }))?);
                return Ok(());
            }
            
            if templates.is_empty() {
                println!("No recurring expense templates found.");
                return Ok(());
            }
            println!("Found {} template(s)", templates.len());
            display.show("recurring_templates", &templates)?;
            println!("\n📊 Estimated Monthly Recurring: ${:.2}", estimated_monthly);
            Ok(())
        }
        RecurringCommands::View { id } => {
            let template = api.get_recurring_template(id)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "template": template
                }))?);
                return Ok(());
            }
            
            display.show("recurring_templates", &vec![template.clone()])?;
            println!("\n📅 Next Occurrence: {}", template.next_occurrence);
            if let Some(last) = &template.last_generated {
                println!("   Last Generated: {}", last);
            }
            Ok(())
        }
        RecurringCommands::Update { id, amount, category, active: _ } => {
            let current = api.get_recurring_template(id)?;
            let update = RecurringExpenseTemplateCreate {
                user_id: current.user_id,
                amount: amount.unwrap_or(current.amount),
                category: category.unwrap_or(current.category),
                description: current.description,
                frequency: current.frequency,
                interval: current.interval,
                day_of_week: current.day_of_week,
                day_of_month: current.day_of_month,
                month_of_year: current.month_of_year,
                start_date: current.start_date,
                end_date: current.end_date,
                tags: current.tags,
            };
            let updated = api.update_recurring_template(id, &update)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "id": id,
                    "template": updated
                }))?);
                return Ok(());
            }
            
            println!("✓ Template {} updated", id);
            display.show("recurring_templates", &vec![updated])?;
            Ok(())
        }
        RecurringCommands::Delete { id, force } => {
            if !force && !json_output {
                let template = api.get_recurring_template(id)?;
                println!("About to delete: {} - ${:.2} ({})", 
                    template.category, template.amount, template.frequency);
                anyhow::bail!("Use --force to confirm deletion");
            }
            api.delete_recurring_template(id)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "id": id,
                    "message": format!("Template {} deleted", id)
                }))?);
                return Ok(());
            }
            
            println!("✓ Template {} deleted", id);
            Ok(())
        }
        RecurringCommands::Upcoming { days } => {
            let upcoming = api.get_upcoming_recurring_expenses(days, None)?;
            let count = upcoming.as_array().map(|a| a.len()).unwrap_or(0);
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "days": days,
                    "count": count,
                    "upcoming": upcoming
                }))?);
                return Ok(());
            }
            
            display.show("upcoming_recurring", &upcoming)?;
            Ok(())
        }
        RecurringCommands::Process => {
            let result = api.generate_due_recurring_expenses()?;
            let processed = result.get("generated_count").and_then(|v| v.as_i64()).unwrap_or(0);
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "generated_count": processed,
                    "result": result
                }))?);
                return Ok(());
            }
            
            println!("✓ Processed {} recurring expense(s)", processed);
            if let Some(expenses) = result.get("expenses") {
                if let Some(arr) = expenses.as_array() {
                    for exp in arr {
                        println!("   - {} ${:.2} ({})", 
                            exp["category"].as_str().unwrap_or(""),
                            exp["amount"].as_f64().unwrap_or(0.0),
                            exp["date"].as_str().unwrap_or("")
                        );
                    }
                }
            }
            Ok(())
        }
    }
}
