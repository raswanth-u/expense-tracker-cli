//! Asset command handlers

use anyhow::Result;
use chrono::Local;
use serde_json::json;

use crate::api::ApiClient;
use crate::cli::AssetCommands;
use crate::display::Display;
use crate::models::{AssetCreate, AssetValueUpdate};

pub fn handle_asset_command(api: &ApiClient, display: &Display, cmd: AssetCommands, json_output: bool) -> Result<()> {
    match cmd {
        AssetCommands::Add { name, asset_type, purchase_value, current_value, user_id, purchase_date, description, location, payment, card_id, account_id, tags } => {
            let asset = AssetCreate {
                user_id,
                name,
                asset_type,
                purchase_value,
                current_value,
                purchase_date,
                description,
                location,
                payment_method: payment,
                credit_card_id: card_id,
                savings_account_id: account_id,
                tags,
            };
            let created = api.create_asset(&asset)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "id": created.id,
                    "asset": created
                }))?);
                return Ok(());
            }
            
            println!("✓ Asset created with ID: {}", created.id.unwrap_or(0));
            display.show("assets", &vec![created])?;
            Ok(())
        }
        AssetCommands::List { user_id, asset_type, active: _ } => {
            let assets = api.list_assets(user_id, asset_type.as_deref())?;
            
            // Calculate totals in frontend
            let total_purchase: f64 = assets.iter().map(|a| a.purchase_value).sum();
            let total_current: f64 = assets.iter().map(|a| a.current_value).sum();
            let net_change = total_current - total_purchase;
            let change_pct = if total_purchase > 0.0 { (net_change / total_purchase) * 100.0 } else { 0.0 };
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "count": assets.len(),
                    "assets": assets,
                    "summary": {
                        "total_purchase_value": total_purchase,
                        "total_current_value": total_current,
                        "net_change": net_change,
                        "change_percent": change_pct
                    }
                }))?);
                return Ok(());
            }
            
            if assets.is_empty() {
                println!("No assets found.");
                return Ok(());
            }
            println!("Found {} asset(s)", assets.len());
            display.show("assets", &assets)?;
            
            println!("\n📊 Portfolio Summary (calculated in frontend):");
            println!("   Total Purchase Value: ${:.2}", total_purchase);
            println!("   Total Current Value: ${:.2}", total_current);
            if net_change >= 0.0 {
                println!("   Net Appreciation: +${:.2} (+{:.1}%)", net_change, change_pct);
            } else {
                println!("   Net Depreciation: -${:.2} ({:.1}%)", net_change.abs(), change_pct);
            }
            Ok(())
        }
        AssetCommands::View { id } => {
            let asset = api.get_asset(id)?;
            
            // Calculate stats in frontend
            let change = asset.current_value - asset.purchase_value;
            let change_pct = if asset.purchase_value > 0.0 { (change / asset.purchase_value) * 100.0 } else { 0.0 };
            
            let mut age_years = 0.0;
            let mut age_days = 0i64;
            let mut annual_change = 0.0;
            
            if let Ok(purchase_date) = chrono::NaiveDate::parse_from_str(&asset.purchase_date, "%Y-%m-%d") {
                let today = Local::now().date_naive();
                age_days = (today - purchase_date).num_days();
                age_years = age_days as f64 / 365.25;
                if age_years > 0.0 {
                    annual_change = change / age_years;
                }
            }
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "asset": asset,
                    "analysis": {
                        "value_change": change,
                        "change_percent": change_pct,
                        "age_years": age_years,
                        "age_days": age_days,
                        "annual_change": annual_change
                    }
                }))?);
                return Ok(());
            }
            
            display.show("assets", &vec![asset])?;
            
            println!("\n📊 Asset Analysis (calculated in frontend):");
            if change >= 0.0 {
                println!("   Appreciation: +${:.2} (+{:.1}%)", change, change_pct);
            } else {
                println!("   Depreciation: -${:.2} ({:.1}%)", change.abs(), change_pct);
            }
            println!("   Age: {:.1} years ({} days)", age_years, age_days);
            if age_years > 0.0 {
                println!("   Annual Change: ${:.2}/year", annual_change);
            }
            Ok(())
        }
        AssetCommands::Update { id, name, current_value, location, active: _ } => {
            let current = api.get_asset(id)?;
            let update = AssetCreate {
                user_id: current.user_id,
                name: name.unwrap_or(current.name),
                asset_type: current.asset_type,
                purchase_value: current.purchase_value,
                current_value: current_value.unwrap_or(current.current_value),
                purchase_date: current.purchase_date,
                description: current.description,
                location: location.or(current.location),
                payment_method: current.payment_method,
                credit_card_id: current.credit_card_id,
                savings_account_id: current.savings_account_id,
                tags: current.tags,
            };
            let updated = api.update_asset(id, &update)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "id": id,
                    "asset": updated
                }))?);
                return Ok(());
            }
            
            println!("✓ Asset {} updated", id);
            display.show("assets", &vec![updated])?;
            Ok(())
        }
        AssetCommands::Delete { id, force } => {
            if !force && !json_output {
                let asset = api.get_asset(id)?;
                println!("About to delete: {} ({}) - Value: ${:.2}", 
                    asset.name, asset.asset_type, asset.current_value);
                anyhow::bail!("Use --force to confirm deletion");
            }
            api.delete_asset(id)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "id": id,
                    "message": format!("Asset {} deleted", id)
                }))?);
                return Ok(());
            }
            
            println!("✓ Asset {} deleted", id);
            Ok(())
        }
        AssetCommands::Value { id, value } => {
            let update = AssetValueUpdate { current_value: value };
            let _result = api.update_asset_value(id, &update)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "id": id,
                    "new_value": value
                }))?);
                return Ok(());
            }
            
            println!("✓ Asset {} value updated to ${:.2}", id, value);
            Ok(())
        }
        AssetCommands::Depreciation { user_id, asset_type: _ } => {
            let assets = api.get_asset_depreciation(user_id)?;
            let count = assets.as_array().map(|a| a.len()).unwrap_or(0);
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "count": count,
                    "depreciation": assets
                }))?);
                return Ok(());
            }
            
            display.show("asset_depreciation", &assets)?;
            Ok(())
        }
        AssetCommands::Summary { user_id } => {
            let summary = api.get_assets_summary(user_id)?;
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(&json!({
                    "success": true,
                    "summary": summary
                }))?);
                return Ok(());
            }
            
            display.show("assets_summary", &summary)?;
            Ok(())
        }
    }
}
