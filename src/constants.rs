// Predefined categories
pub const CATEGORIES: &[&str] = &[
    "Food",
    "Transport",
    "Shopping",
    "Entertainment",
    "Healthcare",
    "Education",
    "Bills & Utilities",
    "Rent",
    "Insurance",
    "Savings",
    "Investment",
    "Personal Care",
    "Gifts",
    "Travel",
    "Other (Custom)",
];

// Payment methods
pub const PAYMENT_METHODS: &[&str] = &[
    "cash",
    "debit_card",
    "credit_card",
    "upi",
];

// Budget periods
pub const BUDGET_PERIODS: &[&str] = &[
    "monthly",
    "weekly",
    "yearly",
];

// User roles
pub const USER_ROLES: &[&str] = &[
    "member",
    "admin",
];

// Common tags
pub const COMMON_TAGS: &[&str] = &[
    "essential",
    "discretionary",
    "urgent",
    "planned",
    "unplanned",
    "weekly",
    "monthly",
    "one-time",
];

// Category keywords for smart suggestions
pub const CATEGORY_KEYWORDS: &[(&str, &str)] = &[
    ("uber", "Transport"),
    ("taxi", "Transport"),
    ("bus", "Transport"),
    ("train", "Transport"),
    ("flight", "Transport"),
    ("gas", "Transport"),
    ("fuel", "Transport"),
    ("restaurant", "Food"),
    ("grocery", "Food"),
    ("coffee", "Food"),
    ("lunch", "Food"),
    ("dinner", "Food"),
    ("breakfast", "Food"),
    ("movie", "Entertainment"),
    ("netflix", "Entertainment"),
    ("spotify", "Entertainment"),
    ("game", "Entertainment"),
    ("doctor", "Healthcare"),
    ("medicine", "Healthcare"),
    ("hospital", "Healthcare"),
    ("pharmacy", "Healthcare"),
    ("rent", "Rent"),
    ("electricity", "Bills & Utilities"),
    ("water", "Bills & Utilities"),
    ("internet", "Bills & Utilities"),
    ("phone", "Bills & Utilities"),
    ("insurance", "Insurance"),
    ("book", "Education"),
    ("course", "Education"),
    ("tuition", "Education"),
    ("gift", "Gifts"),
    ("hotel", "Travel"),
    ("vacation", "Travel"),
];

// Smart category suggestion
pub fn suggest_category(description: &str) -> Option<&'static str> {
    let desc_lower = description.to_lowercase();
    for (keyword, category) in CATEGORY_KEYWORDS {
        if desc_lower.contains(keyword) {
            return Some(category);
        }
    }
    None
}