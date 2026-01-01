//! Command handlers module
//! 
//! This module contains all CLI command handlers for the expense tracker.

// New clap-based command handlers
pub mod expense_cmd;
pub mod user_cmd;
pub mod budget_cmd;
pub mod card_cmd;
pub mod debit_cmd;
pub mod account_cmd;
pub mod goal_cmd;
pub mod asset_cmd;
pub mod recurring_cmd;
pub mod report_cmd;
pub mod search_cmd;
pub mod backup_cmd;
pub mod dashboard_cmd;

// Re-export handler functions
pub use expense_cmd::handle_expense_command;
pub use user_cmd::handle_user_command;
pub use budget_cmd::handle_budget_command;
pub use card_cmd::handle_card_command;
pub use debit_cmd::handle_debit_command;
pub use account_cmd::handle_account_command;
pub use goal_cmd::handle_goal_command;
pub use asset_cmd::handle_asset_command;
pub use recurring_cmd::handle_recurring_command;
pub use report_cmd::handle_report_command;
pub use search_cmd::handle_search_command;
pub use backup_cmd::handle_backup_command;
pub use dashboard_cmd::handle_dashboard_command;