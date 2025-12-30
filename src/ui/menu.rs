use anyhow::Result;
use dialoguer::Select;

pub fn show_main_menu() -> Result<MainMenuOption> {
    println!("\n╔═══════════════════════════════════╗");
    println!("║   Family Expense Tracker CLI      ║");
    println!("╚═══════════════════════════════════╝\n");

    let options = vec![
        "👤 User Management",
        "💰 Expense Management",
        "📊 Budget Management",
        "💳 Credit Card Management",
        "📈 Reports & Analytics",
        "⚙️  Settings",
        "❌ Exit",
    ];

    let quick_actions = vec![
        "",
        "Quick Actions:",
        "  q - Quick Expense",
        "  t - Today's Summary",
        "  m - This Month's Report",
    ];

    for action in &quick_actions {
        println!("{}", action);
    }
    println!();

    let selection = Select::new()
        .with_prompt("Select option")
        .items(&options)
        .default(0)
        .interact()?;

    Ok(match selection {
        0 => MainMenuOption::Users,
        1 => MainMenuOption::Expenses,
        2 => MainMenuOption::Budgets,
        3 => MainMenuOption::Cards,
        4 => MainMenuOption::Reports,
        5 => MainMenuOption::Settings,
        6 => MainMenuOption::Exit,
        _ => MainMenuOption::Exit,
    })
}

pub enum MainMenuOption {
    Users,
    Expenses,
    Budgets,
    Cards,
    Reports,
    Settings,
    Exit,
}

pub fn show_user_menu() -> Result<UserMenuOption> {
    println!("\n╔═══════════════════════════════════╗");
    println!("║      User Management Menu         ║");
    println!("╚═══════════════════════════════════╝\n");

    let options = vec![
        "Create New User",
        "List All Users",
        "View User Details",
        "Update User",
        "Deactivate User",
        "View User Statistics",
        "Back to Main Menu",
    ];

    let selection = Select::new()
        .with_prompt("Select option")
        .items(&options)
        .default(0)
        .interact()?;

    Ok(match selection {
        0 => UserMenuOption::Create,
        1 => UserMenuOption::List,
        2 => UserMenuOption::View,
        3 => UserMenuOption::Update,
        4 => UserMenuOption::Deactivate,
        5 => UserMenuOption::Stats,
        6 => UserMenuOption::Back,
        _ => UserMenuOption::Back,
    })
}

pub enum UserMenuOption {
    Create,
    List,
    View,
    Update,
    Deactivate,
    Stats,
    Back,
}

pub fn show_expense_menu() -> Result<ExpenseMenuOption> {
    println!("\n╔═══════════════════════════════════╗");
    println!("║     Expense Management Menu       ║");
    println!("╚═══════════════════════════════════╝\n");

    let options = vec![
        "Create New Expense",
        "List Expenses",
        "View Expense Details",
        "Update Expense",
        "Delete Expense",
        "View Summary by Category",
        "View Summary by Payment Method",
        "Back to Main Menu",
    ];

    let selection = Select::new()
        .with_prompt("Select option")
        .items(&options)
        .default(0)
        .interact()?;

    Ok(match selection {
        0 => ExpenseMenuOption::Create,
        1 => ExpenseMenuOption::List,
        2 => ExpenseMenuOption::View,
        3 => ExpenseMenuOption::Update,
        4 => ExpenseMenuOption::Delete,
        5 => ExpenseMenuOption::SummaryByCategory,
        6 => ExpenseMenuOption::SummaryByPayment,
        7 => ExpenseMenuOption::Back,
        _ => ExpenseMenuOption::Back,
    })
}

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

pub fn show_budget_menu() -> Result<BudgetMenuOption> {
    println!("\n╔═══════════════════════════════════╗");
    println!("║      Budget Management Menu       ║");
    println!("╚═══════════════════════════════════╝\n");

    let options = vec![
        "Create New Budget",
        "List Budgets",
        "View Budget Details",
        "Update Budget",
        "Delete Budget",
        "View Budget Status",
        "View Budget Alerts",
        "Compare Budgets (Two Months)",
        "Back to Main Menu",
    ];

    let selection = Select::new()
        .with_prompt("Select option")
        .items(&options)
        .default(0)
        .interact()?;

    Ok(match selection {
        0 => BudgetMenuOption::Create,
        1 => BudgetMenuOption::List,
        2 => BudgetMenuOption::View,
        3 => BudgetMenuOption::Update,
        4 => BudgetMenuOption::Delete,
        5 => BudgetMenuOption::Status,
        6 => BudgetMenuOption::Alerts,
        7 => BudgetMenuOption::Compare,
        8 => BudgetMenuOption::Back,
        _ => BudgetMenuOption::Back,
    })
}

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

pub fn show_card_menu() -> Result<CardMenuOption> {
    println!("\n╔═══════════════════════════════════╗");
    println!("║   Credit Card Management Menu     ║");
    println!("╚═══════════════════════════════════╝\n");

    let options = vec![
        "Create New Credit Card",
        "List Credit Cards",
        "View Card Details",
        "Update Card",
        "Deactivate Card",
        "View Billing Statement",
        "View Utilization Trend",
        "View All Cards Summary",
        "Back to Main Menu",
    ];

    let selection = Select::new()
        .with_prompt("Select option")
        .items(&options)
        .default(0)
        .interact()?;

    Ok(match selection {
        0 => CardMenuOption::Create,
        1 => CardMenuOption::List,
        2 => CardMenuOption::View,
        3 => CardMenuOption::Update,
        4 => CardMenuOption::Deactivate,
        5 => CardMenuOption::Statement,
        6 => CardMenuOption::Utilization,
        7 => CardMenuOption::Summary,
        8 => CardMenuOption::Back,
        _ => CardMenuOption::Back,
    })
}

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

pub fn show_report_menu() -> Result<ReportMenuOption> {
    println!("\n╔═══════════════════════════════════╗");
    println!("║     Reports & Analytics Menu      ║");
    println!("╚═══════════════════════════════════╝\n");

    let options = vec![
        "Monthly Report",
        "Family Summary",
        "Category Analysis",
        "Spending Trends",
        "Payment Method Analysis",
        "Export Data",
        "Back to Main Menu",
    ];

    let selection = Select::new()
        .with_prompt("Select option")
        .items(&options)
        .default(0)
        .interact()?;

    Ok(match selection {
        0 => ReportMenuOption::Monthly,
        1 => ReportMenuOption::Family,
        2 => ReportMenuOption::Category,
        3 => ReportMenuOption::Trends,
        4 => ReportMenuOption::Payments,
        5 => ReportMenuOption::Export,
        6 => ReportMenuOption::Back,
        _ => ReportMenuOption::Back,
    })
}

pub enum ReportMenuOption {
    Monthly,
    Family,
    Category,
    Trends,
    Payments,
    Export,
    Back,
}

pub fn show_settings_menu() -> Result<SettingsMenuOption> {
    println!("\n╔═══════════════════════════════════╗");
    println!("║         Settings Menu             ║");
    println!("╚═══════════════════════════════════╝\n");

    let options = vec![
        "View Current Settings",
        "Change Default User",
        "Change Default Category",
        "Toggle Recent Values",
        "Back to Main Menu",
    ];

    let selection = Select::new()
        .with_prompt("Select option")
        .items(&options)
        .default(0)
        .interact()?;

    Ok(match selection {
        0 => SettingsMenuOption::View,
        1 => SettingsMenuOption::ChangeDefaultUser,
        2 => SettingsMenuOption::ChangeDefaultCategory,
        3 => SettingsMenuOption::ToggleRecentValues,
        4 => SettingsMenuOption::Back,
        _ => SettingsMenuOption::Back,
    })
}

pub enum SettingsMenuOption {
    View,
    ChangeDefaultUser,
    ChangeDefaultCategory,
    ToggleRecentValues,
    Back,
}