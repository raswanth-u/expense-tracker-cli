use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub role: String,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Expense {
    pub id: Option<i32>,
    pub user_id: i32,
    pub amount: f64,
    pub category: String,
    pub description: Option<String>,
    pub date: String,
    pub payment_method: String,
    pub credit_card_id: Option<i32>,
    pub is_recurring: Option<bool>,
    pub tags: Option<String>,
    pub savings_account_id: Option<i32>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseCreate {
    pub user_id: i32,
    pub amount: f64,
    pub category: String,
    pub description: Option<String>,
    pub date: String,
    pub payment_method: String,
    pub credit_card_id: Option<i32>,
    pub is_recurring: Option<bool>,
    pub tags: Option<String>,
    pub savings_account_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Budget {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub category: String,
    pub amount: f64,
    pub month: String,
    pub period: Option<String>,
    pub is_active: Option<bool>,
    pub tags: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetCreate {
    pub user_id: Option<i32>,
    pub category: String,
    pub amount: f64,
    pub month: String,
    pub tags: Option<String>,
    pub period: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreditCard {
    pub id: Option<i32>,
    pub user_id: i32,
    pub card_name: String,
    pub last_four: String,
    pub credit_limit: f64,
    pub billing_day: i32,
    pub tags: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditCardCreate {
    pub user_id: i32,
    pub card_name: String,
    pub last_four: String,
    pub credit_limit: f64,
    pub tags: Option<String>,
    pub billing_day: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DebitCard {
    pub id: Option<i32>,
    pub user_id: i32,
    pub card_name: String,
    pub last_four: String,
    pub savings_account_id: i32,
    pub daily_limit: Option<f64>,
    pub is_active: Option<bool>,
    pub tags: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebitCardCreate {
    pub user_id: i32,
    pub card_name: String,
    pub last_four: String,
    pub savings_account_id: i32,
    pub daily_limit: Option<f64>,
    pub tags: Option<String>,
}

#[derive(Debug, Default)]
pub struct ExpenseFilters {
    pub user_id: Option<i32>,
    pub category: Option<String>,
    pub payment_method: Option<String>,
    pub credit_card_id: Option<i32>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub is_recurring: Option<bool>,
    pub tags: Option<String>,
}

// ============================================
// SAVINGS GOAL MODELS
// ============================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SavingsGoal {
    pub id: Option<i32>,
    pub user_id: i32,
    pub name: String,
    pub target_amount: f64,
    pub current_amount: f64,
    pub deadline: String,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingsGoalCreate {
    pub user_id: i32,
    pub name: String,
    pub target_amount: f64,
    pub current_amount: f64,
    pub deadline: String,
    pub description: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingsGoalUpdate {
    pub amount: f64,
}

// ============================================
// ASSET MODELS
// ============================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub id: Option<i32>,
    pub user_id: i32,
    pub name: String,
    pub asset_type: String,
    pub purchase_value: f64,
    pub current_value: f64,
    pub purchase_date: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub payment_method: String,
    pub credit_card_id: Option<i32>,
    pub savings_account_id: Option<i32>,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub tags: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AssetCreate {
    pub user_id: i32,
    pub name: String,
    pub asset_type: String,
    pub purchase_value: f64,
    pub current_value: f64,
    pub purchase_date: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub payment_method: String,
    pub credit_card_id: Option<i32>,
    pub savings_account_id: Option<i32>,
    pub tags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetValueUpdate {
    pub current_value: f64,
}

// ============================================
// RECURRING EXPENSE MODELS
// ============================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecurringExpenseTemplate {
    pub id: Option<i32>,
    pub user_id: i32,
    pub amount: f64,
    pub category: String,
    pub description: Option<String>,
    pub frequency: String,
    pub interval: i32,
    pub day_of_week: Option<i32>,
    pub day_of_month: Option<i32>,
    pub month_of_year: Option<i32>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub next_occurrence: String,
    pub last_generated: Option<String>,
    pub is_active: Option<bool>,
    pub tags: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecurringExpenseTemplateCreate {
    pub user_id: i32,
    pub amount: f64,
    pub category: String,
    pub description: Option<String>,
    pub frequency: String,
    pub interval: i32,
    pub day_of_week: Option<i32>,
    pub day_of_month: Option<i32>,
    pub month_of_year: Option<i32>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub tags: Option<String>,
}

// ============================================
// SAVINGS ACCOUNT MODELS
// ============================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SavingsAccount {
    pub id: Option<i32>,
    pub user_id: i32,
    pub account_name: String,
    pub bank_name: String,
    pub account_number_last_four: String,
    pub account_type: String,
    pub current_balance: f64,
    pub minimum_balance: f64,
    pub interest_rate: f64,
    pub tags: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingsAccountCreate {
    pub user_id: i32,
    pub account_name: String,
    pub bank_name: String,
    pub account_number_last_four: String,
    pub account_type: String,
    pub minimum_balance: f64,
    pub interest_rate: f64,
    pub tags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingsAccountDeposit {
    pub amount: f64,
    pub date: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingsAccountWithdraw {
    pub amount: f64,
    pub date: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
}

// ============================================
// CREDIT CARD TRANSACTION MODELS
// ============================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreditCardTransaction {
    pub id: Option<i32>,
    pub credit_card_id: i32,
    pub transaction_type: String,  // "charge", "payment", "refund", "fee"
    pub amount: f64,
    pub balance_after: f64,
    pub related_expense_id: Option<i32>,
    pub related_asset_id: Option<i32>,
    pub date: String,
    pub description: Option<String>,
    pub merchant: Option<String>,
    pub tags: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditCardPayment {
    pub amount: f64,
    pub date: Option<String>,
    pub description: Option<String>,
    pub source_savings_account_id: Option<i32>,
}

// ============================================
// EXPENSE DETAILS RESPONSE MODEL
// ============================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseDetails {
    pub expense: Expense,
    pub user: User,
    pub credit_card: Option<CreditCard>,
    pub credit_card_transaction: Option<CreditCardTransaction>,
    pub savings_account: Option<SavingsAccount>,
    pub savings_transaction: Option<SavingsAccountTransaction>,
}

// ============================================
// SAVINGS ACCOUNT TRANSACTION MODEL
// ============================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SavingsAccountTransaction {
    pub id: Option<i32>,
    pub savings_account_id: i32,
    pub transaction_type: String,
    pub amount: f64,
    pub balance_after: f64,
    pub related_expense_id: Option<i32>,
    pub related_asset_id: Option<i32>,
    pub date: String,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub created_at: Option<String>,
}