//! Report command handlers

use anyhow::Result;
use serde_json::json;

use crate::api::ApiClient;
use crate::cli::ReportCommands;
use crate::display::Display;

pub fn handle_report_command(api: &ApiClient, display: &Display, cmd: ReportCommands, json_output: bool) -> Result<()> {
    match cmd {
        ReportCommands::Monthly { month, user_id } => {
            let report = api.get_monthly_report(&month, user_id)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "month": month,
                    "report": report
                }))?);
                return Ok(());
            }
            
            display.show("monthly_report", &report)?;
            Ok(())
        }
        ReportCommands::Family { month } => {
            let summary = api.get_family_summary(&month)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "month": month,
                    "summary": summary
                }))?);
                return Ok(());
            }
            
            display.show("family_summary", &summary)?;
            Ok(())
        }
        ReportCommands::Category { category, from, to, user_id } => {
            let analysis = api.get_category_analysis(&category, &from, &to, user_id)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "category": category,
                    "from": from,
                    "to": to,
                    "analysis": analysis
                }))?);
                return Ok(());
            }
            
            display.show("category_analysis", &analysis)?;
            Ok(())
        }
        ReportCommands::Trends { months, user_id } => {
            let trends = api.get_spending_trends(months, user_id)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "months": months,
                    "trends": trends
                }))?);
                return Ok(());
            }
            
            display.show("spending_trends", &trends)?;
            Ok(())
        }
        ReportCommands::Payments { month, user_id } => {
            let from_date = format!("{}-01", month);
            let to_date = format!("{}-31", month);
            let analysis = api.get_payment_method_analysis(&from_date, &to_date, user_id)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "month": month,
                    "analysis": analysis
                }))?);
                return Ok(());
            }
            
            display.show("payment_analysis", &analysis)?;
            Ok(())
        }
    }
}
