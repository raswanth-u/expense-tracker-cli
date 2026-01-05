//! CLI argument parsing using clap
//!
//! Provides a non-interactive CLI with subcommands for all operations.
//! Use --interactive for menu-driven mode.

use clap::{Parser, Subcommand, ValueEnum};

/// Environment to connect to
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum Environment {
    /// Development environment (localhost:8443)
    #[default]
    Dev,
    /// Production environment (localhost:443)
    Prod,
}

#[derive(Parser)]
#[command(name = "expense")]
#[command(author = "Family Expense Tracker")]
#[command(version = "2.0.0")]
#[command(about = "Family Expense Tracker CLI - Manage expenses, budgets, and finances", long_about = None)]
pub struct Cli {
    /// Output in JSON format (for scripting/testing)
    #[arg(long, global = true)]
    pub json: bool,
    
    /// Run in interactive mode with menus
    #[arg(short, long)]
    pub interactive: bool,
    
    /// Environment to connect to (dev or prod)
    #[arg(short, long, value_enum, default_value = "dev", global = true)]
    pub env: Environment,
    
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage expenses
    #[command(subcommand)]
    Expense(ExpenseCommands),
    
    /// Manage users
    #[command(subcommand)]
    User(UserCommands),
    
    /// Manage budgets
    #[command(subcommand)]
    Budget(BudgetCommands),
    
    /// Manage credit cards
    #[command(subcommand)]
    Card(CardCommands),
    
    /// Manage debit cards
    #[command(subcommand)]
    Debit(DebitCommands),
    
    /// Manage savings accounts
    #[command(subcommand)]
    Account(AccountCommands),
    
    /// Manage savings goals
    #[command(subcommand)]
    Goal(GoalCommands),
    
    /// Manage assets
    #[command(subcommand)]
    Asset(AssetCommands),
    
    /// Manage recurring expenses
    #[command(subcommand)]
    Recurring(RecurringCommands),
    
    /// View reports
    #[command(subcommand)]
    Report(ReportCommands),
    
    /// Search across all data
    Search(SearchArgs),
    
    /// Backup and restore data
    #[command(subcommand)]
    Backup(BackupCommands),
    
    /// View dashboard
    Dashboard(DashboardArgs),
}

// ============================================
// EXPENSE COMMANDS
// ============================================

#[derive(Subcommand)]
pub enum ExpenseCommands {
    /// Add a new expense
    Add {
        /// Amount of the expense
        #[arg(short, long)]
        amount: f64,
        
        /// Category (e.g., Food, Transport, Utilities)
        #[arg(short, long)]
        category: String,
        
        /// User ID who made the expense
        #[arg(short, long)]
        user_id: i32,
        
        /// Description of the expense
        #[arg(short, long)]
        description: Option<String>,
        
        /// Date (YYYY-MM-DD), defaults to today
        #[arg(long)]
        date: Option<String>,
        
        /// Payment method (cash, credit_card, debit_card, savings_account, bank_transfer)
        #[arg(short, long, default_value = "cash")]
        payment: String,
        
        /// Credit card ID (required if payment is credit_card)
        #[arg(long)]
        card_id: Option<i32>,
        
        /// Debit card ID (required if payment is debit_card)
        #[arg(long)]
        debit_id: Option<i32>,
        
        /// Savings account ID (required if payment is savings_account)
        #[arg(long)]
        account_id: Option<i32>,
        
        /// Tags (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
        
        /// Is this a recurring expense
        #[arg(long)]
        recurring: bool,
    },
    
    /// List expenses with optional filters
    List {
        /// Filter by user ID
        #[arg(short, long)]
        user_id: Option<i32>,
        
        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,
        
        /// Filter by payment method
        #[arg(short, long)]
        payment: Option<String>,
        
        /// Minimum amount
        #[arg(long)]
        min: Option<f64>,
        
        /// Maximum amount
        #[arg(long)]
        max: Option<f64>,
        
        /// From date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,
        
        /// To date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
        
        /// Filter by tags
        #[arg(short, long)]
        tags: Option<String>,
        
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<i32>,
    },
    
    /// View expense details (shows all related data)
    View {
        /// Expense ID
        id: i32,
    },
    
    /// Update an expense
    Update {
        /// Expense ID to update
        id: i32,
        
        /// New amount
        #[arg(short, long)]
        amount: Option<f64>,
        
        /// New category
        #[arg(short, long)]
        category: Option<String>,
        
        /// New description
        #[arg(short, long)]
        description: Option<String>,
        
        /// New date
        #[arg(long)]
        date: Option<String>,
        
        /// New payment method
        #[arg(short, long)]
        payment: Option<String>,
        
        /// New tags
        #[arg(short, long)]
        tags: Option<String>,
    },
    
    /// Delete an expense
    Delete {
        /// Expense ID to delete
        id: i32,
        
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// Show expense summary by category
    Summary {
        /// Filter by user ID
        #[arg(short, long)]
        user_id: Option<i32>,
        
        /// Month (YYYY-MM)
        #[arg(short, long)]
        month: Option<String>,
    },
}

// ============================================
// USER COMMANDS
// ============================================

#[derive(Subcommand)]
pub enum UserCommands {
    /// Add a new user
    Add {
        /// User's name
        #[arg(short, long)]
        name: String,
        
        /// User's email
        #[arg(short, long)]
        email: String,
        
        /// Role (admin, member)
        #[arg(short, long, default_value = "member")]
        role: String,
    },
    
    /// List all users
    List {
        /// Show only active users
        #[arg(short, long)]
        active: bool,
    },
    
    /// View user details (shows all related data: expenses, budgets, cards, etc.)
    View {
        /// User ID
        id: i32,
    },
    
    /// Update a user
    Update {
        /// User ID to update
        id: i32,
        
        /// New name
        #[arg(short, long)]
        name: Option<String>,
        
        /// New email
        #[arg(short, long)]
        email: Option<String>,
        
        /// New role
        #[arg(short, long)]
        role: Option<String>,
        
        /// Set active status
        #[arg(long)]
        active: Option<bool>,
    },
    
    /// Delete a user
    Delete {
        /// User ID to delete
        id: i32,
        
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// Show user statistics
    Stats {
        /// User ID
        id: i32,
        
        /// Month (YYYY-MM)
        #[arg(short, long)]
        month: Option<String>,
    },
}

// ============================================
// BUDGET COMMANDS
// ============================================

#[derive(Subcommand)]
pub enum BudgetCommands {
    /// Add a new budget
    Add {
        /// Category for the budget
        #[arg(short, long)]
        category: String,
        
        /// Budget amount
        #[arg(short, long)]
        amount: f64,
        
        /// Month (YYYY-MM)
        #[arg(short, long)]
        month: String,
        
        /// User ID (optional, for personal budgets)
        #[arg(short, long)]
        user_id: Option<i32>,
        
        /// Period (monthly, weekly, yearly)
        #[arg(short, long, default_value = "monthly")]
        period: String,
        
        /// Tags
        #[arg(short, long)]
        tags: Option<String>,
    },
    
    /// List budgets
    List {
        /// Filter by user ID
        #[arg(short, long)]
        user_id: Option<i32>,
        
        /// Filter by month
        #[arg(short, long)]
        month: Option<String>,
        
        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,
        
        /// Show only active budgets
        #[arg(short, long)]
        active: bool,
    },
    
    /// View budget details with spending status
    View {
        /// Budget ID
        id: i32,
    },
    
    /// Update a budget
    Update {
        /// Budget ID to update
        id: i32,
        
        /// New amount
        #[arg(short, long)]
        amount: Option<f64>,
        
        /// New category
        #[arg(short, long)]
        category: Option<String>,
        
        /// Set active status
        #[arg(long)]
        active: Option<bool>,
    },
    
    /// Delete a budget
    Delete {
        /// Budget ID to delete
        id: i32,
        
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// Check budget status and alerts
    Status {
        /// Month (YYYY-MM), defaults to current month
        #[arg(short, long)]
        month: Option<String>,
        
        /// User ID
        #[arg(short, long)]
        user_id: Option<i32>,
    },
    
    /// Compare budgets across months
    Compare {
        /// First month (YYYY-MM)
        month1: String,
        
        /// Second month (YYYY-MM)
        month2: String,
        
        /// User ID
        #[arg(short, long)]
        user_id: Option<i32>,
    },
}

// ============================================
// CREDIT CARD COMMANDS
// ============================================

#[derive(Subcommand)]
pub enum CardCommands {
    /// Add a new credit card
    Add {
        /// Card name (e.g., "Chase Sapphire")
        #[arg(short, long)]
        name: String,
        
        /// Last 4 digits of card number
        #[arg(short, long)]
        last_four: String,
        
        /// Credit limit
        #[arg(short, long)]
        limit: f64,
        
        /// User ID who owns the card
        #[arg(short, long)]
        user_id: i32,
        
        /// Billing day of month (1-31)
        #[arg(short, long, default_value = "1")]
        billing_day: i32,
        
        /// Tags
        #[arg(short, long)]
        tags: Option<String>,
    },
    
    /// List credit cards
    List {
        /// Filter by user ID
        #[arg(short, long)]
        user_id: Option<i32>,
        
        /// Show only active cards
        #[arg(short, long)]
        active: bool,
    },
    
    /// View credit card details (shows transactions, expenses, utilization)
    View {
        /// Card ID
        id: i32,
    },
    
    /// Update a credit card
    Update {
        /// Card ID to update
        id: i32,
        
        /// New card name
        #[arg(short, long)]
        name: Option<String>,
        
        /// New credit limit
        #[arg(short, long)]
        limit: Option<f64>,
        
        /// New billing day
        #[arg(short, long)]
        billing_day: Option<i32>,
        
        /// Set active status
        #[arg(long)]
        active: Option<bool>,
    },
    
    /// Delete a credit card
    Delete {
        /// Card ID to delete
        id: i32,
        
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// View card statement
    Statement {
        /// Card ID
        id: i32,
        
        /// Month (YYYY-MM)
        #[arg(short, long)]
        month: String,
    },
    
    /// Make a payment to the card
    Pay {
        /// Card ID
        id: i32,
        
        /// Payment amount
        #[arg(short, long)]
        amount: f64,
        
        /// Source savings account ID (optional)
        #[arg(short, long)]
        from_account: Option<i32>,
        
        /// Description
        #[arg(short, long)]
        description: Option<String>,
    },
    
    /// View card transactions
    Transactions {
        /// Card ID
        id: i32,
        
        /// Filter by transaction type (charge, payment, refund, fee)
        #[arg(short, long)]
        txn_type: Option<String>,
        
        /// From date
        #[arg(long)]
        from: Option<String>,
        
        /// To date
        #[arg(long)]
        to: Option<String>,
    },
    
    /// View utilization for all cards
    Utilization {
        /// Number of months to analyze
        #[arg(short, long, default_value = "6")]
        months: i32,
    },
}

// ============================================
// DEBIT CARD COMMANDS
// ============================================

#[derive(Subcommand)]
pub enum DebitCommands {
    /// Add a new debit card (linked to savings account)
    Add {
        /// Card name
        #[arg(short, long)]
        name: String,
        
        /// Last 4 digits of card number
        #[arg(short, long)]
        last_four: String,
        
        /// User ID who owns the card
        #[arg(short, long)]
        user_id: i32,
        
        /// Linked savings account ID
        #[arg(short, long)]
        account_id: i32,
        
        /// Daily spending limit
        #[arg(short, long)]
        daily_limit: Option<f64>,
        
        /// Tags
        #[arg(short, long)]
        tags: Option<String>,
    },
    
    /// List debit cards
    List {
        /// Filter by user ID
        #[arg(short, long)]
        user_id: Option<i32>,
        
        /// Show only active cards
        #[arg(short, long)]
        active: bool,
    },
    
    /// View debit card details (shows linked account, transactions)
    View {
        /// Debit card ID
        id: i32,
    },
    
    /// Update a debit card
    Update {
        /// Debit card ID to update
        id: i32,
        
        /// New card name
        #[arg(short, long)]
        name: Option<String>,
        
        /// New daily limit
        #[arg(short, long)]
        daily_limit: Option<f64>,
        
        /// Set active status
        #[arg(long)]
        active: Option<bool>,
    },
    
    /// Delete a debit card
    Delete {
        /// Debit card ID to delete
        id: i32,
        
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// View debit card transactions
    Transactions {
        /// Debit card ID
        id: i32,
        
        /// From date
        #[arg(long)]
        from: Option<String>,
        
        /// To date
        #[arg(long)]
        to: Option<String>,
    },
}

// ============================================
// SAVINGS ACCOUNT COMMANDS
// ============================================

#[derive(Subcommand)]
pub enum AccountCommands {
    /// Add a new savings account
    Add {
        /// Account name
        #[arg(short, long)]
        name: String,
        
        /// Bank name
        #[arg(short, long)]
        bank: String,
        
        /// Last 4 digits of account number
        #[arg(short, long)]
        last_four: String,
        
        /// User ID who owns the account
        #[arg(short, long)]
        user_id: i32,
        
        /// Account type (checking, savings, money_market)
        #[arg(short, long, default_value = "savings")]
        account_type: String,
        
        /// Initial balance
        #[arg(long, default_value = "0")]
        balance: f64,
        
        /// Minimum balance requirement
        #[arg(short, long, default_value = "0")]
        min_balance: f64,
        
        /// Interest rate (as percentage)
        #[arg(short, long, default_value = "0")]
        interest_rate: f64,
        
        /// Tags
        #[arg(short, long)]
        tags: Option<String>,
    },
    
    /// List savings accounts
    List {
        /// Filter by user ID
        #[arg(short, long)]
        user_id: Option<i32>,
        
        /// Show only active accounts
        #[arg(short, long)]
        active: bool,
    },
    
    /// View account details (shows transactions, linked debit cards, expenses)
    View {
        /// Account ID
        id: i32,
    },
    
    /// Update an account
    Update {
        /// Account ID to update
        id: i32,
        
        /// New account name
        #[arg(short, long)]
        name: Option<String>,
        
        /// New minimum balance
        #[arg(short, long)]
        min_balance: Option<f64>,
        
        /// New interest rate
        #[arg(short, long)]
        interest_rate: Option<f64>,
        
        /// Set active status
        #[arg(long)]
        active: Option<bool>,
    },
    
    /// Delete an account
    Delete {
        /// Account ID to delete
        id: i32,
        
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// Deposit money into account
    Deposit {
        /// Account ID
        id: i32,
        
        /// Amount to deposit
        #[arg(short, long)]
        amount: f64,
        
        /// Description
        #[arg(short, long)]
        description: Option<String>,
    },
    
    /// Withdraw money from account
    Withdraw {
        /// Account ID
        id: i32,
        
        /// Amount to withdraw
        #[arg(short, long)]
        amount: f64,
        
        /// Description
        #[arg(short, long)]
        description: Option<String>,
    },
    
    /// View account transactions
    Transactions {
        /// Account ID
        id: i32,
        
        /// Filter by transaction type
        #[arg(short, long)]
        txn_type: Option<String>,
        
        /// From date
        #[arg(long)]
        from: Option<String>,
        
        /// To date
        #[arg(long)]
        to: Option<String>,
    },
    
    /// View all accounts summary
    Summary {
        /// User ID
        #[arg(short, long)]
        user_id: Option<i32>,
    },
}

// ============================================
// SAVINGS GOAL COMMANDS
// ============================================

#[derive(Subcommand)]
pub enum GoalCommands {
    /// Add a new savings goal
    Add {
        /// Goal name
        #[arg(short, long)]
        name: String,
        
        /// Target amount
        #[arg(short, long)]
        target: f64,
        
        /// User ID
        #[arg(short, long)]
        user_id: i32,
        
        /// Deadline (YYYY-MM-DD)
        #[arg(short, long)]
        deadline: String,
        
        /// Current amount saved
        #[arg(short, long, default_value = "0")]
        current: f64,
        
        /// Description
        #[arg(long)]
        description: Option<String>,
        
        /// Tags
        #[arg(long)]
        tags: Option<String>,
    },
    
    /// List savings goals
    List {
        /// Filter by user ID
        #[arg(short, long)]
        user_id: Option<i32>,
        
        /// Show only active goals
        #[arg(short, long)]
        active: bool,
    },
    
    /// View goal details with progress
    View {
        /// Goal ID
        id: i32,
    },
    
    /// Update a goal
    Update {
        /// Goal ID to update
        id: i32,
        
        /// New target amount
        #[arg(short, long)]
        target: Option<f64>,
        
        /// New deadline
        #[arg(short, long)]
        deadline: Option<String>,
        
        /// Set active status
        #[arg(long)]
        active: Option<bool>,
    },
    
    /// Delete a goal
    Delete {
        /// Goal ID to delete
        id: i32,
        
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// Add funds to a goal
    Contribute {
        /// Goal ID
        id: i32,
        
        /// Amount to add
        #[arg(short, long)]
        amount: f64,
    },
    
    /// View all goals progress
    Progress {
        /// User ID
        #[arg(short, long)]
        user_id: Option<i32>,
    },
}

// ============================================
// ASSET COMMANDS
// ============================================

#[derive(Subcommand)]
pub enum AssetCommands {
    /// Add a new asset
    Add {
        /// Asset name
        #[arg(short, long)]
        name: String,
        
        /// Asset type (property, vehicle, electronics, jewelry, investment, other)
        #[arg(short, long)]
        asset_type: String,
        
        /// Purchase value
        #[arg(short, long)]
        purchase_value: f64,
        
        /// Current value
        #[arg(short, long)]
        current_value: f64,
        
        /// User ID
        #[arg(short, long)]
        user_id: i32,
        
        /// Purchase date (YYYY-MM-DD)
        #[arg(long)]
        purchase_date: String,
        
        /// Description
        #[arg(short, long)]
        description: Option<String>,
        
        /// Location
        #[arg(short, long)]
        location: Option<String>,
        
        /// Payment method
        #[arg(long, default_value = "cash")]
        payment: String,
        
        /// Credit card ID (if paid by credit card)
        #[arg(long)]
        card_id: Option<i32>,
        
        /// Savings account ID (if paid from savings)
        #[arg(long)]
        account_id: Option<i32>,
        
        /// Tags
        #[arg(short, long)]
        tags: Option<String>,
    },
    
    /// List assets
    List {
        /// Filter by user ID
        #[arg(short, long)]
        user_id: Option<i32>,
        
        /// Filter by asset type
        #[arg(short, long)]
        asset_type: Option<String>,
        
        /// Show only active assets
        #[arg(short, long)]
        active: bool,
    },
    
    /// View asset details
    View {
        /// Asset ID
        id: i32,
    },
    
    /// Update an asset
    Update {
        /// Asset ID to update
        id: i32,
        
        /// New name
        #[arg(short, long)]
        name: Option<String>,
        
        /// New current value
        #[arg(short, long)]
        current_value: Option<f64>,
        
        /// New location
        #[arg(short, long)]
        location: Option<String>,
        
        /// Set active status
        #[arg(long)]
        active: Option<bool>,
    },
    
    /// Delete an asset
    Delete {
        /// Asset ID to delete
        id: i32,
        
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// Update asset value
    Value {
        /// Asset ID
        id: i32,
        
        /// New current value
        value: f64,
    },
    
    /// View depreciation/appreciation analysis
    Depreciation {
        /// Filter by user ID
        #[arg(short, long)]
        user_id: Option<i32>,
        
        /// Filter by asset type
        #[arg(short, long)]
        asset_type: Option<String>,
    },
    
    /// View assets summary
    Summary {
        /// User ID
        #[arg(short, long)]
        user_id: Option<i32>,
    },
}

// ============================================
// RECURRING EXPENSE COMMANDS
// ============================================

#[derive(Subcommand)]
pub enum RecurringCommands {
    /// Add a recurring expense template
    Add {
        /// Amount
        #[arg(short, long)]
        amount: f64,
        
        /// Category
        #[arg(short, long)]
        category: String,
        
        /// User ID
        #[arg(short, long)]
        user_id: i32,
        
        /// Frequency (daily, weekly, monthly, yearly)
        #[arg(short, long)]
        frequency: String,
        
        /// Start date (YYYY-MM-DD)
        #[arg(short, long)]
        start_date: String,
        
        /// Description
        #[arg(short, long)]
        description: Option<String>,
        
        /// Interval (every N periods)
        #[arg(short, long, default_value = "1")]
        interval: i32,
        
        /// Day of week (0=Monday, 6=Sunday) for weekly
        #[arg(long)]
        day_of_week: Option<i32>,
        
        /// Day of month (1-31) for monthly
        #[arg(long)]
        day_of_month: Option<i32>,
        
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
        
        /// Tags
        #[arg(short, long)]
        tags: Option<String>,
    },
    
    /// List recurring templates
    List {
        /// Filter by user ID
        #[arg(short, long)]
        user_id: Option<i32>,
        
        /// Show only active templates
        #[arg(short, long)]
        active: bool,
    },
    
    /// View recurring template details
    View {
        /// Template ID
        id: i32,
    },
    
    /// Update a recurring template
    Update {
        /// Template ID to update
        id: i32,
        
        /// New amount
        #[arg(short, long)]
        amount: Option<f64>,
        
        /// New category
        #[arg(short, long)]
        category: Option<String>,
        
        /// Set active status
        #[arg(long)]
        active: Option<bool>,
    },
    
    /// Delete a recurring template
    Delete {
        /// Template ID to delete
        id: i32,
        
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// View upcoming recurring expenses
    Upcoming {
        /// Number of days to look ahead
        #[arg(short, long, default_value = "30")]
        days: i32,
    },
    
    /// Process pending recurring expenses
    Process,
}

// ============================================
// REPORT COMMANDS
// ============================================

#[derive(Subcommand)]
pub enum ReportCommands {
    /// Monthly expense report
    Monthly {
        /// Month (YYYY-MM)
        #[arg(short, long)]
        month: String,
        
        /// User ID
        #[arg(short, long)]
        user_id: Option<i32>,
    },
    
    /// Family summary report
    Family {
        /// Month (YYYY-MM)
        #[arg(short, long)]
        month: String,
    },
    
    /// Category analysis report
    Category {
        /// Category to analyze
        category: String,
        
        /// From date (YYYY-MM-DD)
        #[arg(short, long)]
        from: String,
        
        /// To date (YYYY-MM-DD)
        #[arg(short, long)]
        to: String,
        
        /// User ID
        #[arg(short, long)]
        user_id: Option<i32>,
    },
    
    /// Spending trends report
    Trends {
        /// Number of months to analyze
        #[arg(short, long, default_value = "6")]
        months: i32,
        
        /// User ID
        #[arg(short, long)]
        user_id: Option<i32>,
    },
    
    /// Payment method analysis
    Payments {
        /// Month (YYYY-MM)
        #[arg(short, long)]
        month: String,
        
        /// User ID
        #[arg(short, long)]
        user_id: Option<i32>,
    },
}

// ============================================
// SEARCH COMMAND
// ============================================

#[derive(clap::Args)]
pub struct SearchArgs {
    /// Search query
    pub query: String,
    
    /// Entity type to search (expenses, users, budgets, cards, accounts, goals, assets)
    #[arg(short, long)]
    pub entity: Option<String>,
    
    /// From date for date range
    #[arg(long)]
    pub from: Option<String>,
    
    /// To date for date range
    #[arg(long)]
    pub to: Option<String>,
    
    /// Minimum amount
    #[arg(long)]
    pub min: Option<f64>,
    
    /// Maximum amount
    #[arg(long)]
    pub max: Option<f64>,
}

// ============================================
// BACKUP COMMANDS
// ============================================

#[derive(Subcommand)]
pub enum BackupCommands {
    /// Create a backup
    Create {
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Restore from a backup
    Restore {
        /// Backup file path
        file: String,
        
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// List available backups
    List,
}

// ============================================
// DASHBOARD COMMAND
// ============================================

#[derive(clap::Args)]
pub struct DashboardArgs {
    /// Month (YYYY-MM), defaults to current month
    #[arg(short, long)]
    pub month: Option<String>,
    
    /// User ID
    #[arg(short, long)]
    pub user_id: Option<i32>,
}
