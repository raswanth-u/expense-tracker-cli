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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetCreate {
    pub user_id: Option<i32>,
    pub category: String,
    pub amount: f64,
    pub month: String,
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
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditCardCreate {
    pub user_id: i32,
    pub card_name: String,
    pub last_four: String,
    pub credit_limit: f64,
    pub billing_day: i32,
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