//! # Menu System
//!
//! Provides a unified menu system using a generic approach to reduce code duplication.

use anyhow::Result;
use crate::ui::selectors::select_with_number;

// ============================================================================
// GENERIC MENU BUILDER
// ============================================================================

/// Generic function to display a menu and get user selection
pub fn show_menu<T: Clone>(title: &str, options: &[(&str, T)]) -> Result<T> {
    println!("\n╔═══════════════════════════════════════╗");
    println!("║ {:^37} ║", title);
    println!("╚═══════════════════════════════════════╝\n");

    let labels: Vec<String> = options.iter().map(|(label, _)| label.to_string()).collect();
    let selection = select_with_number("Select option", &labels)?;
    
    Ok(options[selection].1.clone())
}

// ============================================================================
// MAIN MENU
// ============================================================================

#[derive(Clone, Debug)]
pub enum MainMenuOption {
    Expenses,
    Budgets,
    Users,
    Cards,
    DebitCards,
    SavingsAccounts,
    SavingsGoals,
    Assets,
    Recurring,
    Reports,
    Search,
    Backup,
    Exit,
}

pub fn show_main_menu() -> Result<MainMenuOption> {
    println!("\n╔═══════════════════════════════════════╗");
    println!("║     Family Expense Tracker CLI        ║");
    println!("╚═══════════════════════════════════════╝\n");

    let options = vec![
        ("💸 Expense Management", MainMenuOption::Expenses),
        ("📊 Budget Management", MainMenuOption::Budgets),
        ("👤 User Management", MainMenuOption::Users),
        ("💳 Credit Cards", MainMenuOption::Cards),
        ("💳 Debit Cards", MainMenuOption::DebitCards),
        ("🏦 Savings Accounts", MainMenuOption::SavingsAccounts),
        ("🎯 Savings Goals", MainMenuOption::SavingsGoals),
        ("🏠 Assets", MainMenuOption::Assets),
        ("🔄 Recurring Expenses", MainMenuOption::Recurring),
        ("📈 Reports & Analytics", MainMenuOption::Reports),
        ("🔍 Search", MainMenuOption::Search),
        ("💾 Backup & Restore", MainMenuOption::Backup),
        ("❌ Exit", MainMenuOption::Exit),
    ];

    let labels: Vec<String> = options.iter().map(|(l, _)| l.to_string()).collect();
    let selection = select_with_number("Select option", &labels)?;
    
    Ok(options[selection].1.clone())
}

// ============================================================================
// CRUD MENU OPTIONS (Shared across entities)
// ============================================================================

#[derive(Clone, Debug)]
pub enum CrudOption {
    Create,
    List,
    View,
    Update,
    Delete,
    Back,
}

/// Standard CRUD menu for entities
pub fn show_crud_menu(entity_name: &str) -> Result<CrudOption> {
    show_menu(
        &format!("{} Management", entity_name),
        &[
            ("➕ Create New", CrudOption::Create),
            ("📋 List All", CrudOption::List),
            ("🔍 View Details", CrudOption::View),
            ("✏️  Update", CrudOption::Update),
            ("🗑️  Delete", CrudOption::Delete),
            ("🔙 Back", CrudOption::Back),
        ],
    )
}

// ============================================================================
// USER MENU
// ============================================================================

#[derive(Clone, Debug)]
pub enum UserMenuOption {
    Create,
    List,
    View,
    Update,
    Deactivate,
    Stats,
    Back,
}

pub fn show_user_menu() -> Result<UserMenuOption> {
    show_menu(
        "User Management",
        &[
            ("➕ Create New User", UserMenuOption::Create),
            ("📋 List Users", UserMenuOption::List),
            ("🔍 View User Details", UserMenuOption::View),
            ("✏️  Update User", UserMenuOption::Update),
            ("⏸️  Deactivate User", UserMenuOption::Deactivate),
            ("📊 User Statistics", UserMenuOption::Stats),
            ("🔙 Back", UserMenuOption::Back),
        ],
    )
}

// ============================================================================
// EXPENSE MENU
// ============================================================================

#[derive(Clone, Debug)]
pub enum ExpenseMenuOption {
    Create,
    List,
    View,
    Update,
    Delete,
    SummaryByCategory,
    SummaryByPayment,
    Back,
}

pub fn show_expense_menu() -> Result<ExpenseMenuOption> {
    show_menu(
        "Expense Management",
        &[
            ("➕ Create Expense", ExpenseMenuOption::Create),
            ("📋 List Expenses", ExpenseMenuOption::List),
            ("🔍 View Expense", ExpenseMenuOption::View),
            ("✏️  Update Expense", ExpenseMenuOption::Update),
            ("🗑️  Delete Expense", ExpenseMenuOption::Delete),
            ("📊 Summary by Category", ExpenseMenuOption::SummaryByCategory),
            ("💳 Summary by Payment", ExpenseMenuOption::SummaryByPayment),
            ("🔙 Back", ExpenseMenuOption::Back),
        ],
    )
}

// ============================================================================
// BUDGET MENU
// ============================================================================

#[derive(Clone, Debug)]
pub enum BudgetMenuOption {
    Create,
    List,
    View,
    Update,
    Delete,
    Status,
    Alerts,
    Compare,
    Back,
}

pub fn show_budget_menu() -> Result<BudgetMenuOption> {
    show_menu(
        "Budget Management",
        &[
            ("➕ Create Budget", BudgetMenuOption::Create),
            ("📋 List Budgets", BudgetMenuOption::List),
            ("🔍 View Budget", BudgetMenuOption::View),
            ("✏️  Update Budget", BudgetMenuOption::Update),
            ("🗑️  Delete Budget", BudgetMenuOption::Delete),
            ("📊 Budget Status", BudgetMenuOption::Status),
            ("🔔 Budget Alerts", BudgetMenuOption::Alerts),
            ("⚖️  Compare Months", BudgetMenuOption::Compare),
            ("🔙 Back", BudgetMenuOption::Back),
        ],
    )
}

// ============================================================================
// CREDIT CARD MENU
// ============================================================================

#[derive(Clone, Debug)]
pub enum CardMenuOption {
    Create,
    List,
    View,
    Update,
    Deactivate,
    Statement,
    Utilization,
    Summary,
    Back,
}

pub fn show_card_menu() -> Result<CardMenuOption> {
    show_menu(
        "Credit Card Management",
        &[
            ("➕ Add Credit Card", CardMenuOption::Create),
            ("📋 List Cards", CardMenuOption::List),
            ("🔍 View Card", CardMenuOption::View),
            ("✏️  Update Card", CardMenuOption::Update),
            ("⏸️  Deactivate Card", CardMenuOption::Deactivate),
            ("📄 Billing Statement", CardMenuOption::Statement),
            ("📈 Utilization Trend", CardMenuOption::Utilization),
            ("📊 All Cards Summary", CardMenuOption::Summary),
            ("🔙 Back", CardMenuOption::Back),
        ],
    )
}

// ============================================================================
// DEBIT CARD MENU
// ============================================================================

#[derive(Clone, Debug)]
pub enum DebitMenuOption {
    Create,
    List,
    View,
    Update,
    Deactivate,
    Transactions,
    Summary,
    Back,
}

pub fn show_debit_menu() -> Result<DebitMenuOption> {
    show_menu(
        "Debit Card Management",
        &[
            ("➕ Add Debit Card", DebitMenuOption::Create),
            ("📋 List Cards", DebitMenuOption::List),
            ("🔍 View Card", DebitMenuOption::View),
            ("✏️  Update Card", DebitMenuOption::Update),
            ("⏸️  Deactivate Card", DebitMenuOption::Deactivate),
            ("📊 Transactions", DebitMenuOption::Transactions),
            ("📋 Summary", DebitMenuOption::Summary),
            ("🔙 Back", DebitMenuOption::Back),
        ],
    )
}

// ============================================================================
// REPORT MENU
// ============================================================================

#[derive(Clone, Debug)]
pub enum ReportMenuOption {
    Monthly,
    Family,
    Category,
    Trends,
    Payments,
    Export,
    Back,
}

pub fn show_report_menu() -> Result<ReportMenuOption> {
    show_menu(
        "Reports & Analytics",
        &[
            ("📅 Monthly Report", ReportMenuOption::Monthly),
            ("👨‍👩‍👧‍👦 Family Summary", ReportMenuOption::Family),
            ("📂 Category Analysis", ReportMenuOption::Category),
            ("📈 Spending Trends", ReportMenuOption::Trends),
            ("💳 Payment Analysis", ReportMenuOption::Payments),
            ("📤 Export Data", ReportMenuOption::Export),
            ("🔙 Back", ReportMenuOption::Back),
        ],
    )
}

// ============================================================================
// SAVINGS GOALS MENU
// ============================================================================

#[derive(Clone, Debug)]
pub enum SavingsMenuOption {
    Create,
    List,
    View,
    Deposit,
    Withdraw,
    Delete,
    Back,
}

pub fn show_savings_menu() -> Result<SavingsMenuOption> {
    show_menu(
        "Savings Goals",
        &[
            ("➕ Create Goal", SavingsMenuOption::Create),
            ("📋 List Goals", SavingsMenuOption::List),
            ("🔍 View Progress", SavingsMenuOption::View),
            ("💰 Add Funds", SavingsMenuOption::Deposit),
            ("💸 Withdraw", SavingsMenuOption::Withdraw),
            ("🗑️  Delete Goal", SavingsMenuOption::Delete),
            ("🔙 Back", SavingsMenuOption::Back),
        ],
    )
}

// ============================================================================
// SAVINGS ACCOUNTS MENU
// ============================================================================

#[derive(Clone, Debug)]
pub enum SavingsAccountMenuOption {
    Create,
    List,
    View,
    Deposit,
    Withdraw,
    Transactions,
    Delete,
    Back,
}

pub fn show_savings_account_menu() -> Result<SavingsAccountMenuOption> {
    show_menu(
        "Savings Accounts",
        &[
            ("➕ Add Account", SavingsAccountMenuOption::Create),
            ("📋 List Accounts", SavingsAccountMenuOption::List),
            ("🔍 View Account", SavingsAccountMenuOption::View),
            ("💰 Deposit", SavingsAccountMenuOption::Deposit),
            ("💸 Withdraw", SavingsAccountMenuOption::Withdraw),
            ("📜 Transactions", SavingsAccountMenuOption::Transactions),
            ("🗑️  Delete Account", SavingsAccountMenuOption::Delete),
            ("🔙 Back", SavingsAccountMenuOption::Back),
        ],
    )
}

// ============================================================================
// ASSET MENU
// ============================================================================

#[derive(Clone, Debug)]
pub enum AssetMenuOption {
    Create,
    List,
    View,
    Update,
    UpdateValue,
    Delete,
    Summary,
    Back,
}

pub fn show_asset_menu() -> Result<AssetMenuOption> {
    show_menu(
        "Asset Management",
        &[
            ("➕ Add Asset", AssetMenuOption::Create),
            ("📋 List Assets", AssetMenuOption::List),
            ("🔍 View Asset", AssetMenuOption::View),
            ("✏️  Update Asset", AssetMenuOption::Update),
            ("💰 Update Value", AssetMenuOption::UpdateValue),
            ("🗑️  Delete Asset", AssetMenuOption::Delete),
            ("📊 Assets Summary", AssetMenuOption::Summary),
            ("🔙 Back", AssetMenuOption::Back),
        ],
    )
}

// ============================================================================
// RECURRING EXPENSE MENU
// ============================================================================

#[derive(Clone, Debug)]
pub enum RecurringMenuOption {
    Create,
    List,
    View,
    Update,
    Generate,
    Pause,
    Delete,
    Back,
}

pub fn show_recurring_menu() -> Result<RecurringMenuOption> {
    show_menu(
        "Recurring Expenses",
        &[
            ("➕ Create Template", RecurringMenuOption::Create),
            ("📋 List Templates", RecurringMenuOption::List),
            ("🔍 View Template", RecurringMenuOption::View),
            ("✏️  Update Template", RecurringMenuOption::Update),
            ("⚡ Generate Expenses", RecurringMenuOption::Generate),
            ("⏸️  Pause/Resume", RecurringMenuOption::Pause),
            ("🗑️  Delete Template", RecurringMenuOption::Delete),
            ("🔙 Back", RecurringMenuOption::Back),
        ],
    )
}

// ============================================================================
// BACKUP MENU
// ============================================================================

#[derive(Clone, Debug)]
pub enum BackupMenuOption {
    Create,
    Restore,
    List,
    Back,
}

pub fn show_backup_menu() -> Result<BackupMenuOption> {
    show_menu(
        "Backup & Restore",
        &[
            ("💾 Create Backup", BackupMenuOption::Create),
            ("📥 Restore Backup", BackupMenuOption::Restore),
            ("📋 List Backups", BackupMenuOption::List),
            ("🔙 Back", BackupMenuOption::Back),
        ],
    )
}

// ============================================================================
// SETTINGS MENU (kept for compatibility but simplified)
// ============================================================================

#[derive(Clone, Debug)]
pub enum SettingsMenuOption {
    View,
    ChangeDefaultUser,
    ChangeDefaultCategory,
    ToggleRecentValues,
    Back,
}

pub fn show_settings_menu() -> Result<SettingsMenuOption> {
    show_menu(
        "Settings",
        &[
            ("👁️  View Settings", SettingsMenuOption::View),
            ("👤 Default User", SettingsMenuOption::ChangeDefaultUser),
            ("📂 Default Category", SettingsMenuOption::ChangeDefaultCategory),
            ("🔄 Toggle Recent Values", SettingsMenuOption::ToggleRecentValues),
            ("🔙 Back", SettingsMenuOption::Back),
        ],
    )
}
