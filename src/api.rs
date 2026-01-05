use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::config::Config;
use crate::models::*;

pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl ApiClient {
    /// Create new API client from config
    pub fn new(config: &Config) -> Result<Self> {
        let mut client_builder = Client::builder();
        
        // Skip SSL verification if configured (for self-signed certs)
        if config.api.skip_ssl_verify {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }
        
        let client = client_builder.build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self {
            client,
            base_url: config.api.base_url.clone(),
            api_key: config.api.api_key.clone().unwrap_or_else(|| {
                eprintln!("❌ API key not configured!");
                std::process::exit(1);
            }),
        })
    }
    
    /// Build headers with API key
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-API-Key",
            HeaderValue::from_str(&self.api_key).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }
    
    /// Make GET request
    fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        let response = self.client
            .get(&url)
            .headers(self.headers())
            .send()
            .context(format!("Failed to GET {}", endpoint))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, error_text);
        }
        
        response.json()
            .context("Failed to parse response JSON")
    }
    
    /// Make POST request
    fn post<T: Serialize, R: DeserializeOwned>(&self, endpoint: &str, body: &T) -> Result<R> {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        let response = self.client
            .post(&url)
            .headers(self.headers())
            .json(body)
            .send()
            .context(format!("Failed to POST {}", endpoint))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, error_text);
        }
        
        response.json()
            .context("Failed to parse response JSON")
    }
    
    /// Make PUT request
    fn put<T: Serialize, R: DeserializeOwned>(&self, endpoint: &str, body: &T) -> Result<R> {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        let response = self.client
            .put(&url)
            .headers(self.headers())
            .json(body)
            .send()
            .context(format!("Failed to PUT {}", endpoint))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, error_text);
        }
        
        response.json()
            .context("Failed to parse response JSON")
    }
    
    /// Make DELETE request
    fn delete<R: DeserializeOwned>(&self, endpoint: &str) -> Result<R> {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        let response = self.client
            .delete(&url)
            .headers(self.headers())
            .send()
            .context(format!("Failed to DELETE {}", endpoint))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, error_text);
        }
        
        response.json()
            .context("Failed to parse response JSON")
    }
    
    // ============================================
    // USER ENDPOINTS
    // ============================================
    
    pub fn create_user(&self, user: &UserCreate) -> Result<User> {
        self.post("users/", user)
    }
    
    pub fn list_users(&self, is_active: Option<bool>) -> Result<Vec<User>> {
        let endpoint = if let Some(active) = is_active {
            format!("users/?is_active={}", active)
        } else {
            "users/".to_string()
        };
        self.get(&endpoint)
    }
    
    pub fn get_user(&self, id: i32) -> Result<User> {
        self.get(&format!("users/{}", id))
    }
    
    pub fn update_user(&self, id: i32, user: &UserCreate) -> Result<User> {
    self.put(&format!("users/{}", id), user)
    }

    pub fn delete_user(&self, id: i32) -> Result<serde_json::Value> {
        self.delete(&format!("users/{}", id))
    }

    pub fn get_user_stats(&self, id: i32, month: Option<&str>) -> Result<serde_json::Value> {
        let endpoint = if let Some(m) = month {
            format!("users/{}/stats?month={}", id, m)
        } else {
            format!("users/{}/stats", id)
        };
        self.get(&endpoint)
    }

    // ============================================
    // EXPENSE ENDPOINTS
    // ============================================
    
    pub fn create_expense(&self, expense: &ExpenseCreate) -> Result<Expense> {
        self.post("expenses/", expense)
    }
    
    pub fn get_expense(&self, id: i32) -> Result<Expense> {
        self.get(&format!("expenses/{}", id))
    }
    
    pub fn get_expense_details(&self, id: i32) -> Result<ExpenseDetails> {
        self.get(&format!("expenses/{}/details", id))
    }
    
    pub fn update_expense(&self, id: i32, expense: &ExpenseCreate) -> Result<Expense> {
        self.put(&format!("expenses/{}", id), expense)
    }
    
    pub fn delete_expense(&self, id: i32) -> Result<serde_json::Value> {
        self.delete(&format!("expenses/{}", id))
    }

    pub fn list_expenses_filtered(&self, filters: &ExpenseFilters) -> Result<Vec<Expense>> {
        let mut params = vec![];
        
        if let Some(uid) = filters.user_id {
            params.push(format!("user_id={}", uid));
        }
        if let Some(ref cat) = filters.category {
            params.push(format!("category={}", cat));
        }
        if let Some(ref pm) = filters.payment_method {
            params.push(format!("payment_method={}", pm));
        }
        if let Some(cid) = filters.credit_card_id {
            params.push(format!("credit_card_id={}", cid));
        }
        if let Some(min) = filters.min_amount {
            params.push(format!("min_amount={}", min));
        }
        if let Some(max) = filters.max_amount {
            params.push(format!("max_amount={}", max));
        }
        if let Some(ref from) = filters.from_date {
            params.push(format!("from_date={}", from));
        }
        if let Some(ref to) = filters.to_date {
            params.push(format!("to_date={}", to));
        }
        if let Some(rec) = filters.is_recurring {
            params.push(format!("is_recurring={}", rec));
        }
        if let Some(ref tags) = filters.tags {
            params.push(format!("tags={}", tags));
        }
        
        let endpoint = if params.is_empty() {
            "expenses/".to_string()
        } else {
            format!("expenses/?{}", params.join("&"))
        };
        
        self.get(&endpoint)
    }
    
    pub fn get_expense_summary(
        &self,
        from_date: Option<&str>,
        to_date: Option<&str>,
        user_id: Option<i32>,
    ) -> Result<serde_json::Value> {
        let mut params = vec![];
        
        if let Some(from) = from_date {
            params.push(format!("from_date={}", from));
        }
        if let Some(to) = to_date {
            params.push(format!("to_date={}", to));
        }
        if let Some(uid) = user_id {
            params.push(format!("user_id={}", uid));
        }
        
        let endpoint = if params.is_empty() {
            "expenses/summary".to_string()
        } else {
            format!("expenses/summary?{}", params.join("&"))
        };
        
        self.get(&endpoint)
    }

    pub fn get_payment_summary(
        &self,
        from_date: Option<&str>,
        to_date: Option<&str>,
        user_id: Option<i32>,
    ) -> Result<serde_json::Value> {
        let mut params = vec![];
        
        if let Some(from) = from_date {
            params.push(format!("from_date={}", from));
        }
        if let Some(to) = to_date {
            params.push(format!("to_date={}", to));
        }
        if let Some(uid) = user_id {
            params.push(format!("user_id={}", uid));
        }
        
        let endpoint = if params.is_empty() {
            "expenses/payment_summary".to_string()
        } else {
            format!("expenses/payment_summary?{}", params.join("&"))
        };
        
        self.get(&endpoint)
    }
    
    // ============================================
    // BUDGET ENDPOINTS
    // ============================================
    
    pub fn create_budget(&self, budget: &BudgetCreate) -> Result<Budget> {
        self.post("budgets/", budget)
    }
    
    pub fn list_budgets(
        &self,
        month: Option<&str>,
        user_id: Option<i32>,
        category: Option<&str>,
    ) -> Result<Vec<Budget>> {
        let mut params = vec![];
        if let Some(m) = month {
            params.push(format!("month={}", m));
        }
        if let Some(uid) = user_id {
            params.push(format!("user_id={}", uid));
        }
        if let Some(cat) = category {
            params.push(format!("category={}", cat));
        }
        
        let endpoint = if params.is_empty() {
            "budgets/".to_string()
        } else {
            format!("budgets/?{}", params.join("&"))
        };
        
        self.get(&endpoint)
    }
    
    pub fn get_budget_status(&self, month: &str, user_id: Option<i32>) -> Result<serde_json::Value> {
        let endpoint = if let Some(uid) = user_id {
            format!("budgets/status/summary?month={}&user_id={}", month, uid)
        } else {
            format!("budgets/status/summary?month={}", month)
        };
        self.get(&endpoint)
    }

    pub fn get_budget(&self, id: i32) -> Result<Budget> {
        self.get(&format!("budgets/{}", id))
    }

    pub fn update_budget(&self, id: i32, budget: &BudgetCreate) -> Result<Budget> {
        self.put(&format!("budgets/{}", id), budget)
    }

    pub fn delete_budget(&self, id: i32) -> Result<serde_json::Value> {
        self.delete(&format!("budgets/{}", id))
    }

    pub fn get_budget_alerts(&self, month: &str, user_id: Option<i32>) -> Result<serde_json::Value> {
        let endpoint = if let Some(uid) = user_id {
            format!("budgets/status/alerts?month={}&user_id={}", month, uid)
        } else {
            format!("budgets/status/alerts?month={}", month)
        };
        self.get(&endpoint)
    }

    pub fn compare_budgets(
        &self,
        month1: &str,
        month2: &str,
        user_id: Option<i32>,
    ) -> Result<serde_json::Value> {
        let endpoint = if let Some(uid) = user_id {
            format!("budgets/compare?month1={}&month2={}&user_id={}", month1, month2, uid)
        } else {
            format!("budgets/compare?month1={}&month2={}", month1, month2)
        };
        self.get(&endpoint)
    }
    
    // ============================================
    // CREDIT CARD ENDPOINTS
    // ============================================
    
    pub fn create_credit_card(&self, card: &CreditCardCreate) -> Result<CreditCard> {
        self.post("credit-cards/", card)
    }
    
    pub fn list_credit_cards(&self, user_id: Option<i32>) -> Result<Vec<CreditCard>> {
        let endpoint = if let Some(uid) = user_id {
            format!("credit-cards/?user_id={}", uid)
        } else {
            "credit-cards/".to_string()
        };
        self.get(&endpoint)
    }

    pub fn get_credit_card(&self, id: i32) -> Result<CreditCard> {
        self.get(&format!("credit-cards/{}", id))
    }

    pub fn update_credit_card(&self, id: i32, card: &CreditCardCreate) -> Result<CreditCard> {
        self.put(&format!("credit-cards/{}", id), card)
    }

    pub fn delete_credit_card(&self, id: i32) -> Result<serde_json::Value> {
        self.delete(&format!("credit-cards/{}", id))
    }

    pub fn get_credit_card_statement(
        &self,
        id: i32,
        month: &str,
    ) -> Result<serde_json::Value> {
        self.get(&format!("credit-cards/{}/statement?month={}", id, month))
    }

    pub fn get_credit_card_utilization(
        &self,
        id: i32,
        months: i32,
    ) -> Result<serde_json::Value> {
        self.get(&format!("credit-cards/{}/utilization?months={}", id, months))
    }

    pub fn get_all_cards_summary(
        &self,
        user_id: Option<i32>,
        month: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut params = vec![];
        
        if let Some(uid) = user_id {
            params.push(format!("user_id={}", uid));
        }
        if let Some(m) = month {
            params.push(format!("month={}", m));
        }
        
        let endpoint = if params.is_empty() {
            "credit-cards/summary".to_string()
        } else {
            format!("credit-cards/summary?{}", params.join("&"))
        };
        
        self.get(&endpoint)
    }
    
    // ============================================
    // CREDIT CARD TRANSACTION ENDPOINTS
    // ============================================
    
    pub fn get_credit_card_transactions(
        &self,
        card_id: i32,
        transaction_type: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<CreditCardTransaction>> {
        let mut params = vec![];
        
        if let Some(t) = transaction_type {
            params.push(format!("transaction_type={}", t));
        }
        if let Some(from) = from_date {
            params.push(format!("from_date={}", from));
        }
        if let Some(to) = to_date {
            params.push(format!("to_date={}", to));
        }
        
        let endpoint = if params.is_empty() {
            format!("credit-cards/{}/transactions", card_id)
        } else {
            format!("credit-cards/{}/transactions?{}", card_id, params.join("&"))
        };
        
        self.get(&endpoint)
    }
    
    pub fn make_credit_card_payment(
        &self,
        card_id: i32,
        payment: &CreditCardPayment,
    ) -> Result<CreditCardTransaction> {
        self.post(&format!("credit-cards/{}/payment", card_id), payment)
    }
    
    // ============================================
    // DEBIT CARD ENDPOINTS
    // ============================================
    
    pub fn create_debit_card(&self, card: &DebitCardCreate) -> Result<DebitCard> {
        self.post("debit-cards/", card)
    }
    
    pub fn list_debit_cards(&self, user_id: Option<i32>) -> Result<Vec<DebitCard>> {
        let endpoint = if let Some(uid) = user_id {
            format!("debit-cards/?user_id={}", uid)
        } else {
            "debit-cards/".to_string()
        };
        self.get(&endpoint)
    }
    
    pub fn get_debit_card(&self, id: i32) -> Result<DebitCard> {
        let response: serde_json::Value = self.get(&format!("debit-cards/{}", id))?;
        // The API returns {card: {...}, linked_account: {...}}
        // Extract just the card
        let card: DebitCard = serde_json::from_value(response["card"].clone())?;
        Ok(card)
    }
    
    pub fn get_debit_card_details(&self, id: i32) -> Result<serde_json::Value> {
        self.get(&format!("debit-cards/{}", id))
    }
    
    pub fn update_debit_card(&self, id: i32, card: &DebitCardCreate) -> Result<DebitCard> {
        self.put(&format!("debit-cards/{}", id), card)
    }
    
    pub fn delete_debit_card(&self, id: i32) -> Result<serde_json::Value> {
        self.delete(&format!("debit-cards/{}", id))
    }
    
    pub fn get_debit_card_transactions(
        &self,
        card_id: i32,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<SavingsAccountTransaction>> {
        let mut params = vec![];
        
        if let Some(from) = from_date {
            params.push(format!("from_date={}", from));
        }
        if let Some(to) = to_date {
            params.push(format!("to_date={}", to));
        }
        
        let endpoint = if params.is_empty() {
            format!("debit-cards/{}/transactions", card_id)
        } else {
            format!("debit-cards/{}/transactions?{}", card_id, params.join("&"))
        };
        
        self.get(&endpoint)
    }
    
    // ============================================
    // REPORT ENDPOINTS
    // ============================================
    
    pub fn get_monthly_report(&self, month: &str, user_id: Option<i32>) -> Result<serde_json::Value> {
        let endpoint = if let Some(uid) = user_id {
            format!("reports/monthly?month={}&user_id={}", month, uid)
        } else {
            format!("reports/monthly?month={}", month)
        };
        self.get(&endpoint)
    }

    pub fn get_family_summary(&self, month: &str) -> Result<serde_json::Value> {
        self.get(&format!("reports/family-summary?month={}", month))
    }

    pub fn get_category_analysis(
        &self,
        category: &str,
        from_date: &str,
        to_date: &str,
        user_id: Option<i32>,
    ) -> Result<serde_json::Value> {
            let endpoint = if let Some(uid) = user_id {
                format!(
                    "reports/category-analysis?category={}&from_date={}&to_date={}&user_id={}",
                    category, from_date, to_date, uid
                )
            } else {
                format!(
                    "reports/category-analysis?category={}&from_date={}&to_date={}",
                    category, from_date, to_date
                )
            };
        self.get(&endpoint)
    }

    pub fn get_spending_trends(
        &self,
        months: i32,
        user_id: Option<i32>,
    ) -> Result<serde_json::Value> {
        let endpoint = if let Some(uid) = user_id {
            format!("reports/spending-trends?months={}&user_id={}", months, uid)
        } else {
            format!("reports/spending-trends?months={}", months)
        };
        self.get(&endpoint)
    }

    pub fn get_payment_method_analysis(
        &self,
        from_date: &str,
        to_date: &str,
        user_id: Option<i32>,
    ) -> Result<serde_json::Value> {
        let endpoint = if let Some(uid) = user_id {
            format!(
                "reports/payment-method-analysis?from_date={}&to_date={}&user_id={}",
                from_date, to_date, uid
            )
        } else {
            format!(
                "reports/payment-method-analysis?from_date={}&to_date={}",
                from_date, to_date
            )
        };
        self.get(&endpoint)
    }

    pub fn export_expenses_json(
        &self,
        from_date: &str,
        to_date: &str,
        user_id: Option<i32>,
    ) -> Result<serde_json::Value> {
        let endpoint = if let Some(uid) = user_id {
            format!(
                "reports/export?from_date={}&to_date={}&user_id={}&format=json",
                from_date, to_date, uid
            )
        } else {
            format!(
                "reports/export?from_date={}&to_date={}&format=json",
                from_date, to_date
            )
        };
        self.get(&endpoint)
    }

    pub fn export_expenses_csv(
        &self,
        from_date: &str,
        to_date: &str,
        user_id: Option<i32>,
    ) -> Result<String> {
        let url = if let Some(uid) = user_id {
            format!(
                "{}/reports/export?from_date={}&to_date={}&user_id={}&format=csv",
                self.base_url, from_date, to_date, uid
            )
        } else {
            format!(
                "{}/reports/export?from_date={}&to_date={}&format=csv",
                self.base_url, from_date, to_date
            )
        };
        
        let response = self.client
            .get(&url)
            .headers(self.headers())
            .send()
            .context("Failed to export CSV")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, error_text);
        }
        
        response.text().context("Failed to read CSV response")
    }

    // ============================================
    // SAVINGS GOAL ENDPOINTS
    // ============================================

    pub fn create_savings_goal(&self, goal: &crate::models::SavingsGoalCreate) -> Result<crate::models::SavingsGoal> {
        self.post("savings-goals/", goal)
    }

    pub fn list_savings_goals(&self, user_id: Option<i32>) -> Result<Vec<crate::models::SavingsGoal>> {
        let endpoint = if let Some(uid) = user_id {
            format!("savings-goals/?user_id={}", uid)
        } else {
            "savings-goals/".to_string()
        };
        self.get(&endpoint)
    }

    pub fn update_savings_goal(&self, id: i32, goal: &crate::models::SavingsGoalCreate) -> Result<crate::models::SavingsGoal> {
        self.put(&format!("savings-goals/{}", id), goal)
    }

    pub fn delete_savings_goal(&self, id: i32) -> Result<serde_json::Value> {
        self.delete(&format!("savings-goals/{}", id))
    }

    pub fn add_to_savings_goal(&self, id: i32, update: &crate::models::SavingsGoalUpdate) -> Result<crate::models::SavingsGoal> {
        self.post(&format!("savings-goals/{}/add", id), update)
    }

    pub fn withdraw_from_savings_goal(&self, id: i32, update: &crate::models::SavingsGoalUpdate) -> Result<crate::models::SavingsGoal> {
        self.post(&format!("savings-goals/{}/withdraw", id), update)
    }

    pub fn get_savings_goal_progress(&self, id: i32) -> Result<serde_json::Value> {
        self.get(&format!("savings-goals/{}/progress", id))
    }

    // ============================================
    // ASSET ENDPOINTS
    // ============================================

    pub fn create_asset(&self, asset: &crate::models::AssetCreate) -> Result<crate::models::Asset> {
        self.post("assets/", asset)
    }

    pub fn list_assets(&self, user_id: Option<i32>, asset_type: Option<&str>) -> Result<Vec<crate::models::Asset>> {
        let mut params = vec![];
        
        if let Some(uid) = user_id {
            params.push(format!("user_id={}", uid));
        }
        if let Some(atype) = asset_type {
            params.push(format!("asset_type={}", atype));
        }
        
        let endpoint = if params.is_empty() {
            "assets/".to_string()
        } else {
            format!("assets/?{}", params.join("&"))
        };
        
        self.get(&endpoint)
    }

    pub fn get_asset(&self, id: i32) -> Result<crate::models::Asset> {
        self.get(&format!("assets/{}", id))
    }

    pub fn update_asset(&self, id: i32, asset: &crate::models::AssetCreate) -> Result<crate::models::Asset> {
        self.put(&format!("assets/{}", id), asset)
    }

    pub fn delete_asset(&self, id: i32) -> Result<serde_json::Value> {
        self.delete(&format!("assets/{}", id))
    }

    pub fn update_asset_value(&self, id: i32, update: &crate::models::AssetValueUpdate) -> Result<crate::models::Asset> {
        self.put(&format!("assets/{}/value", id), update)
    }

    pub fn get_assets_summary(&self, user_id: Option<i32>) -> Result<serde_json::Value> {
        let endpoint = if let Some(uid) = user_id {
            format!("assets/summary?user_id={}", uid)
        } else {
            "assets/summary".to_string()
        };
        self.get(&endpoint)
    }

    pub fn get_asset_depreciation(&self, user_id: Option<i32>) -> Result<serde_json::Value> {
        let endpoint = if let Some(uid) = user_id {
            format!("assets/depreciation?user_id={}", uid)
        } else {
            "assets/depreciation".to_string()
        };
        self.get(&endpoint)
    }

    // ============================================
    // RECURRING EXPENSE ENDPOINTS
    // ============================================

    pub fn create_recurring_template(&self, template: &crate::models::RecurringExpenseTemplateCreate) -> Result<crate::models::RecurringExpenseTemplate> {
        self.post("recurring-expenses/", template)
    }

    pub fn list_recurring_templates(&self, user_id: Option<i32>, frequency: Option<&str>) -> Result<Vec<crate::models::RecurringExpenseTemplate>> {
        let mut params = vec![];
        
        if let Some(uid) = user_id {
            params.push(format!("user_id={}", uid));
        }
        if let Some(freq) = frequency {
            params.push(format!("frequency={}", freq));
        }
        
        let endpoint = if params.is_empty() {
            "recurring-expenses/".to_string()
        } else {
            format!("recurring-expenses/?{}", params.join("&"))
        };
        
        self.get(&endpoint)
    }

    pub fn get_recurring_template(&self, id: i32) -> Result<crate::models::RecurringExpenseTemplate> {
        self.get(&format!("recurring-expenses/{}", id))
    }

    pub fn update_recurring_template(&self, id: i32, template: &crate::models::RecurringExpenseTemplateCreate) -> Result<crate::models::RecurringExpenseTemplate> {
        self.put(&format!("recurring-expenses/{}", id), template)
    }

    pub fn delete_recurring_template(&self, id: i32) -> Result<serde_json::Value> {
        self.delete(&format!("recurring-expenses/{}", id))
    }

    // pub fn generate_expense_from_template(&self, id: i32) -> Result<crate::models::Expense> {
    //     self.post(&format!("recurring-expenses/{}/generate", id), &serde_json::json!({}))
    // }

    pub fn get_upcoming_recurring_expenses(&self, days: i32, user_id: Option<i32>) -> Result<serde_json::Value> {
        let mut params = vec![format!("days={}", days)];
        
        if let Some(uid) = user_id {
            params.push(format!("user_id={}", uid));
        }
        
        self.get(&format!("recurring-expenses/upcoming?{}", params.join("&")))
    }

    pub fn skip_recurring_occurrence(&self, id: i32) -> Result<serde_json::Value> {
        self.post(&format!("recurring-expenses/{}/skip", id), &serde_json::json!({}))
    }

    pub fn generate_due_recurring_expenses(&self) -> Result<serde_json::Value> {
        self.post("recurring-expenses/generate-due", &serde_json::json!({}))
    }

    // ============================================
    // SAVINGS ACCOUNT ENDPOINTS
    // ============================================

    pub fn create_savings_account(&self, account: &crate::models::SavingsAccountCreate) -> Result<crate::models::SavingsAccount> {
        self.post("savings-accounts/", account)
    }

    pub fn list_savings_accounts(&self, user_id: Option<i32>) -> Result<Vec<crate::models::SavingsAccount>> {
        let endpoint = if let Some(uid) = user_id {
            format!("savings-accounts/?user_id={}", uid)
        } else {
            "savings-accounts/".to_string()
        };
        self.get(&endpoint)
    }

    // pub fn get_savings_account(&self, id: i32) -> Result<crate::models::SavingsAccount> {
    //     self.get(&format!("savings-accounts/{}", id))
    // }

    pub fn update_savings_account(&self, id: i32, account: &crate::models::SavingsAccountCreate) -> Result<crate::models::SavingsAccount> {
        self.put(&format!("savings-accounts/{}", id), account)
    }

    pub fn delete_savings_account(&self, id: i32) -> Result<serde_json::Value> {
        self.delete(&format!("savings-accounts/{}", id))
    }

    pub fn deposit_to_account(&self, id: i32, deposit: &crate::models::SavingsAccountDeposit) -> Result<crate::models::SavingsAccount> {
        self.post(&format!("savings-accounts/{}/deposit", id), deposit)
    }

    pub fn withdraw_from_account(&self, id: i32, withdrawal: &crate::models::SavingsAccountWithdraw) -> Result<crate::models::SavingsAccount> {
        self.post(&format!("savings-accounts/{}/withdraw", id), withdrawal)
    }

    pub fn post_interest(&self, id: i32, interest: &crate::models::SavingsAccountDeposit) -> Result<crate::models::SavingsAccount> {
        self.post(&format!("savings-accounts/{}/interest", id), interest)
    }

    pub fn get_account_transactions(
        &self,
        id: i32,
        from_date: Option<&str>,
        to_date: Option<&str>,
        transaction_type: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut params = vec![];
        
        if let Some(from) = from_date {
            params.push(format!("from_date={}", from));
        }
        if let Some(to) = to_date {
            params.push(format!("to_date={}", to));
        }
        if let Some(ttype) = transaction_type {
            params.push(format!("transaction_type={}", ttype));
        }
        
        let endpoint = if params.is_empty() {
            format!("savings-accounts/{}/transactions", id)
        } else {
            format!("savings-accounts/{}/transactions?{}", id, params.join("&"))
        };
        
        self.get(&endpoint)
    }

    pub fn get_account_summary(&self, id: i32) -> Result<serde_json::Value> {
        self.get(&format!("savings-accounts/{}/summary", id))
    }

    pub fn get_all_accounts_summary(&self, user_id: Option<i32>) -> Result<serde_json::Value> {
        let endpoint = if let Some(uid) = user_id {
            format!("savings-accounts/summary/all?user_id={}", uid)
        } else {
            "savings-accounts/summary/all".to_string()
        };
        self.get(&endpoint)
    }

}
