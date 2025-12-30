#!/usr/bin/env python3
"""
Display module for expense tracker CLI
Uses rich for beautiful terminal output
"""

from rich.console import Console
from rich.table import Table
from rich.panel import Panel
from rich import box
from typing import List, Dict, Any
import json
import sys


console = Console()


def display_users(users: List[Dict[str, Any]]) -> None:
    """Display users in a formatted table."""
    if not users:
        console.print("[yellow]No users found[/yellow]")
        return
    
    table = Table(title="👥 Users", box=box.ROUNDED)
    table.add_column("ID", style="cyan", justify="right")
    table.add_column("Name", style="green")
    table.add_column("Email", style="blue")
    table.add_column("Role", style="magenta")
    table.add_column("Active", style="yellow")
    
    for user in users:
        table.add_row(
            str(user.get("id", "")),
            user.get("name", ""),
            user.get("email", ""),
            user.get("role", ""),
            "✓" if user.get("is_active") else "✗"
        )
    
    console.print(table)


def display_expenses(expenses: List[Dict[str, Any]]) -> None:
    """Display expenses in a formatted table."""
    if not expenses:
        console.print("[yellow]No expenses found[/yellow]")
        return
    
    table = Table(title="💰 Expenses", box=box.ROUNDED)
    table.add_column("ID", style="cyan", justify="right")
    table.add_column("Date", style="blue")
    table.add_column("Amount", style="green", justify="right")
    table.add_column("Category", style="magenta")
    table.add_column("Payment", style="yellow")
    table.add_column("Description", style="white")
    
    for expense in expenses:
        table.add_row(
            str(expense.get("id", "")),
            expense.get("date", ""),
            f"${expense.get('amount', 0):.2f}",
            expense.get("category", ""),
            expense.get("payment_method", ""),
            expense.get("description", "")[:30] if expense.get("description") else ""
        )
    
    console.print(table)


def display_budgets(budgets: List[Dict[str, Any]]) -> None:
    """Display budgets in a formatted table."""
    if not budgets:
        console.print("[yellow]No budgets found[/yellow]")
        return
    
    table = Table(title="📊 Budgets", box=box.ROUNDED)
    table.add_column("ID", style="cyan", justify="right")
    table.add_column("Category", style="magenta")
    table.add_column("Amount", style="green", justify="right")
    table.add_column("Month", style="blue")
    table.add_column("Period", style="yellow")
    
    for budget in budgets:
        table.add_row(
            str(budget.get("id", "")),
            budget.get("category", ""),
            f"${budget.get('amount', 0):.2f}",
            budget.get("month", ""),
            budget.get("period", "")
        )
    
    console.print(table)


def display_credit_cards(cards: List[Dict[str, Any]]) -> None:
    """Display credit cards in a formatted table."""
    if not cards:
        console.print("[yellow]No credit cards found[/yellow]")
        return
    
    table = Table(title="💳 Credit Cards", box=box.ROUNDED)
    table.add_column("ID", style="cyan", justify="right")
    table.add_column("Name", style="green")
    table.add_column("Last 4", style="blue")
    table.add_column("Limit", style="magenta", justify="right")
    table.add_column("Billing Day", style="yellow", justify="right")
    
    for card in cards:
        table.add_row(
            str(card.get("id", "")),
            card.get("card_name", ""),
            f"****{card.get('last_four', '')}",
            f"${card.get('credit_limit', 0):,.2f}",
            str(card.get("billing_day", ""))
        )
    
    console.print(table)


def display_budget_status(status: Dict[str, Any]) -> None:
    """Display budget status with alerts."""
    console.print(Panel.fit(
        f"[bold]Budget Status for {status.get('month', '')}[/bold]",
        border_style="blue"
    ))
    
    # Overall summary
    console.print(f"\n[bold]Overall:[/bold]")
    console.print(f"  Total Budget: [green]${status.get('total_budget', 0):,.2f}[/green]")
    console.print(f"  Total Spent:  [yellow]${status.get('total_spent', 0):,.2f}[/yellow]")
    console.print(f"  Remaining:    [cyan]${status.get('total_remaining', 0):,.2f}[/cyan]")
    console.print(f"  Usage:        [magenta]{status.get('overall_percentage', 0):.1f}%[/magenta]")
    
    # Budget details
    budgets = status.get("budgets", [])
    if budgets:
        console.print("\n[bold]By Category:[/bold]")
        
        table = Table(box=box.SIMPLE)
        table.add_column("Category", style="cyan")
        table.add_column("Budget", style="green", justify="right")
        table.add_column("Spent", style="yellow", justify="right")
        table.add_column("Remaining", style="blue", justify="right")
        table.add_column("Status", style="magenta")
        
        for b in budgets:
            status_emoji = {
                "ok": "✅",
                "warning": "⚠️",
                "exceeded": "🚨"
            }.get(b.get("status", "ok"), "❓")
            
            table.add_row(
                b.get("category", ""),
                f"${b.get('budget', 0):,.2f}",
                f"${b.get('spent', 0):,.2f}",
                f"${b.get('remaining', 0):,.2f}",
                f"{status_emoji} {b.get('percentage', 0):.1f}%"
            )
        
        console.print(table)


def display_monthly_report(report: Dict[str, Any]) -> None:
    """Display comprehensive monthly report."""
    console.print(Panel.fit(
        f"[bold]Monthly Report - {report.get('month', '')}[/bold]",
        border_style="green"
    ))
    
    summary = report.get("summary", {})
    console.print(f"\n[bold]Summary:[/bold]")
    console.print(f"  Total Spent:     [green]${summary.get('total_spent', 0):,.2f}[/green]")
    console.print(f"  Transactions:    [cyan]{summary.get('transaction_count', 0)}[/cyan]")
    console.print(f"  Average:         [yellow]${summary.get('average_transaction', 0):,.2f}[/yellow]")
    console.print(f"  Largest:         [red]${summary.get('largest_expense', 0):,.2f}[/red]")
    console.print(f"  Smallest:        [blue]${summary.get('smallest_expense', 0):,.2f}[/blue]")
    
    # By category
    by_category = report.get("by_category", {})
    if by_category:
        console.print("\n[bold]By Category:[/bold]")
        table = Table(box=box.SIMPLE)
        table.add_column("Category", style="cyan")
        table.add_column("Amount", style="green", justify="right")
        
        for category, amount in by_category.items():
            table.add_row(category, f"${amount:,.2f}")
        
        console.print(table)


def display_success(message: str) -> None:
    """Display success message."""
    console.print(f"[bold green]✓[/bold green] {message}")


def display_error(message: str) -> None:
    """Display error message."""
    console.print(f"[bold red]✗[/bold red] {message}")


def display_info(message: str) -> None:
    """Display info message."""
    console.print(f"[bold blue]ℹ[/bold blue] {message}")
    
def display_user_stats(stats: Dict[str, Any]) -> None:
    """Display user statistics."""
    console.print(Panel.fit(
        f"[bold]Statistics for {stats.get('user_name', 'User')}[/bold]",
        border_style="cyan"
    ))
    
    console.print(f"\n[bold]Period:[/bold] {stats.get('period', 'N/A')}")
    console.print(f"[bold]Total Spent:[/bold] [green]${stats.get('total_spent', 0):,.2f}[/green]")
    console.print(f"[bold]Transactions:[/bold] [cyan]{stats.get('transaction_count', 0)}[/cyan]")
    console.print(f"[bold]Average:[/bold] [yellow]${stats.get('average_transaction', 0):,.2f}[/yellow]")
    
    # By category
    by_category = stats.get("by_category", {})
    if by_category:
        console.print("\n[bold]By Category:[/bold]")
        table = Table(box=box.SIMPLE)
        table.add_column("Category", style="cyan")
        table.add_column("Amount", style="green", justify="right")
        
        for category, amount in sorted(by_category.items(), key=lambda x: x[1], reverse=True):
            table.add_row(category, f"${amount:,.2f}")
        
        console.print(table)
    
    # By payment method
    by_payment = stats.get("by_payment_method", {})
    if by_payment:
        console.print("\n[bold]By Payment Method:[/bold]")
        table = Table(box=box.SIMPLE)
        table.add_column("Method", style="magenta")
        table.add_column("Amount", style="green", justify="right")
        
        for method, amount in sorted(by_payment.items(), key=lambda x: x[1], reverse=True):
            table.add_row(method, f"${amount:,.2f}")
        
        console.print(table)

def display_expense_summary(summary: List[Dict[str, Any]]) -> None:
    """Display expense summary by category."""
    if not summary:
        console.print("[yellow]No expense data found[/yellow]")
        return
    
    table = Table(title="💰 Expense Summary by Category", box=box.ROUNDED)
    table.add_column("Category", style="cyan")
    table.add_column("Total", style="green", justify="right")
    table.add_column("Count", style="yellow", justify="right")
    table.add_column("Average", style="blue", justify="right")
    
    total_amount = 0
    total_count = 0
    
    for item in summary:
        category = item.get("category", "Unknown")
        total = item.get("total", 0)
        count = item.get("count", 0)
        avg = total / count if count > 0 else 0
        
        total_amount += total
        total_count += count
        
        table.add_row(
            category,
            f"${total:,.2f}",
            str(count),
            f"${avg:,.2f}"
        )
    
    console.print(table)
    console.print(f"\n[bold]Grand Total:[/bold] [green]${total_amount:,.2f}[/green] ({total_count} transactions)")

def display_payment_summary(summary: List[Dict[str, Any]]) -> None:
    """Display payment method summary."""
    if not summary:
        console.print("[yellow]No payment data found[/yellow]")
        return
    
    table = Table(title="💳 Payment Method Summary", box=box.ROUNDED)
    table.add_column("Payment Method", style="magenta")
    table.add_column("Total", style="green", justify="right")
    table.add_column("Count", style="yellow", justify="right")
    table.add_column("Average", style="blue", justify="right")
    
    total_amount = 0
    total_count = 0
    
    for item in summary:
        method = item.get("payment_method", "Unknown")
        total = item.get("total", 0)
        count = item.get("count", 0)
        avg = total / count if count > 0 else 0
        
        total_amount += total
        total_count += count
        
        table.add_row(
            method,
            f"${total:,.2f}",
            str(count),
            f"${avg:,.2f}"
        )
    
    console.print(table)
    console.print(f"\n[bold]Grand Total:[/bold] [green]${total_amount:,.2f}[/green] ({total_count} transactions)")

def display_budget_alerts(alerts: Dict[str, Any]) -> None:
    """Display budget alerts (warnings and exceeded)."""
    console.print(Panel.fit(
        f"[bold]Budget Alerts for {alerts.get('month', '')}[/bold]",
        border_style="red"
    ))
    
    alert_list = alerts.get("alerts", [])
    
    if not alert_list:
        console.print("\n[green]✓ No budget alerts! All spending is under control.[/green]")
        return
    
    console.print(f"\n[bold red]⚠️  {alerts.get('alert_count', 0)} Budget(s) Need Attention[/bold red]\n")
    
    table = Table(box=box.ROUNDED)
    table.add_column("Category", style="cyan")
    table.add_column("Budget", style="green", justify="right")
    table.add_column("Spent", style="yellow", justify="right")
    table.add_column("Status", style="red")
    table.add_column("Alert", style="magenta")
    
    for alert in alert_list:
        status_emoji = {
            "warning": "⚠️",
            "exceeded": "🚨"
        }.get(alert.get("status", ""), "❓")
        
        table.add_row(
            alert.get("category", ""),
            f"${alert.get('budget', 0):,.2f}",
            f"${alert.get('spent', 0):,.2f}",
            f"{status_emoji} {alert.get('percentage', 0):.1f}%",
            alert.get("alert", "")
        )
    
    console.print(table)

def display_budget_comparison(comparison: Dict[str, Any]) -> None:
    """Display budget comparison between two months."""
    console.print(Panel.fit(
        f"[bold]Budget Comparison: {comparison.get('month1', '')} vs {comparison.get('month2', '')}[/bold]",
        border_style="blue"
    ))
    
    total_change = comparison.get("total_change", 0)
    change_color = "red" if total_change > 0 else "green" if total_change < 0 else "yellow"
    change_symbol = "↑" if total_change > 0 else "↓" if total_change < 0 else "→"
    
    console.print(f"\n[bold]Total Change:[/bold] [{change_color}]{change_symbol} ${abs(total_change):,.2f}[/{change_color}]")
    
    categories = comparison.get("categories", [])
    
    if not categories:
        console.print("\n[yellow]No category data available[/yellow]")
        return
    
    console.print("\n[bold]By Category:[/bold]")
    
    table = Table(box=box.ROUNDED)
    table.add_column("Category", style="cyan")
    table.add_column(comparison.get('month1', 'Month 1'), style="blue", justify="right")
    table.add_column(comparison.get('month2', 'Month 2'), style="blue", justify="right")
    table.add_column("Change", style="magenta", justify="right")
    table.add_column("Trend", style="yellow")
    
    for cat in categories:
        month1_key = f"{comparison.get('month1', '')}_spent"
        month2_key = f"{comparison.get('month2', '')}_spent"
        
        change = cat.get("change", 0)
        change_pct = cat.get("change_percentage", 0)
        trend = cat.get("trend", "stable")
        
        trend_emoji = {
            "increased": "📈",
            "decreased": "📉",
            "stable": "➡️"
        }.get(trend, "❓")
        
        change_color = "red" if change > 0 else "green" if change < 0 else "yellow"
        
        table.add_row(
            cat.get("category", ""),
            f"${cat.get(month1_key, 0):,.2f}",
            f"${cat.get(month2_key, 0):,.2f}",
            f"[{change_color}]${abs(change):,.2f} ({change_pct:+.1f}%)[/{change_color}]",
            f"{trend_emoji} {trend}"
        )
    
    console.print(table)

def display_card_statement(statement: Dict[str, Any]) -> None:
    """Display credit card billing statement."""
    console.print(Panel.fit(
        f"[bold]Statement: {statement.get('card_name', '')} (****{statement.get('last_four', '')})[/bold]",
        border_style="blue"
    ))
    
    billing = statement.get("billing_cycle", {})
    console.print(f"\n[bold]Billing Cycle:[/bold] {billing.get('start', '')} to {billing.get('end', '')}")
    console.print(f"[bold]Billing Day:[/bold] {billing.get('billing_day', '')}")
    
    summary = statement.get("summary", {})
    console.print(f"\n[bold]Summary:[/bold]")
    console.print(f"  Total Spent:        [red]${summary.get('total_spent', 0):,.2f}[/red]")
    console.print(f"  Transactions:       [cyan]{summary.get('transaction_count', 0)}[/cyan]")
    console.print(f"  Credit Limit:       [green]${summary.get('credit_limit', 0):,.2f}[/green]")
    console.print(f"  Available Credit:   [blue]${summary.get('available_credit', 0):,.2f}[/blue]")
    
    utilization = summary.get("utilization_percentage", 0)
    util_color = "red" if utilization >= 70 else "yellow" if utilization >= 30 else "green"
    console.print(f"  Utilization:        [{util_color}]{utilization:.1f}%[/{util_color}]")
    
    # By category
    by_category = statement.get("by_category", {})
    if by_category:
        console.print("\n[bold]By Category:[/bold]")
        table = Table(box=box.SIMPLE)
        table.add_column("Category", style="cyan")
        table.add_column("Amount", style="green", justify="right")
        
        for category, amount in sorted(by_category.items(), key=lambda x: x[1], reverse=True):
            table.add_row(category, f"${amount:,.2f}")
        
        console.print(table)
    
    # Transactions
    transactions = statement.get("transactions", [])
    if transactions:
        console.print(f"\n[bold]Recent Transactions ({len(transactions)}):[/bold]")
        table = Table(box=box.SIMPLE)
        table.add_column("Date", style="blue")
        table.add_column("Category", style="magenta")
        table.add_column("Description", style="white")
        table.add_column("Amount", style="green", justify="right")
        
        for txn in transactions[:10]:  # Show last 10
            table.add_row(
                txn.get("date", ""),
                txn.get("category", ""),
                (txn.get("description", "") or "")[:30],
                f"${txn.get('amount', 0):,.2f}"
            )
        
        console.print(table)
        
        if len(transactions) > 10:
            console.print(f"\n[dim]... and {len(transactions) - 10} more transactions[/dim]")

def display_card_utilization(utilization: Dict[str, Any]) -> None:
    """Display credit card utilization trend."""
    console.print(Panel.fit(
        f"[bold]Utilization Trend: {utilization.get('card_name', '')} (****{utilization.get('last_four', '')})[/bold]",
        border_style="magenta"
    ))
    
    console.print(f"\n[bold]Credit Limit:[/bold] [green]${utilization.get('credit_limit', 0):,.2f}[/green]")
    console.print(f"[bold]Months Analyzed:[/bold] {utilization.get('months_analyzed', 0)}")
    
    avg_util = utilization.get("average_utilization", 0)
    util_color = "red" if avg_util >= 70 else "yellow" if avg_util >= 30 else "green"
    console.print(f"[bold]Average Utilization:[/bold] [{util_color}]{avg_util:.1f}%[/{util_color}]")
    
    recommendation = utilization.get("recommendation", "")
    rec_color = "green" if "Good" in recommendation else "yellow" if "Consider" in recommendation else "red"
    console.print(f"[bold]Recommendation:[/bold] [{rec_color}]{recommendation}[/{rec_color}]")
    
    history = utilization.get("history", [])
    if history:
        console.print("\n[bold]Monthly History:[/bold]")
        table = Table(box=box.ROUNDED)
        table.add_column("Month", style="blue")
        table.add_column("Spent", style="green", justify="right")
        table.add_column("Utilization", style="magenta", justify="right")
        table.add_column("Status", style="yellow")
        
        for month_data in history:
            util = month_data.get("utilization", 0)
            status = "🟢" if util < 30 else "🟡" if util < 70 else "🔴"
            
            table.add_row(
                month_data.get("month", ""),
                f"${month_data.get('spent', 0):,.2f}",
                f"{util:.1f}%",
                status
            )
        
        console.print(table)

def display_cards_summary(summary: Dict[str, Any]) -> None:
    """Display all credit cards summary."""
    if "message" in summary:
        console.print(f"[yellow]{summary['message']}[/yellow]")
        return
    
    console.print(Panel.fit(
        f"[bold]Credit Cards Summary - {summary.get('month', '')}[/bold]",
        border_style="blue"
    ))
    
    console.print(f"\n[bold]Overview:[/bold]")
    console.print(f"  Total Cards:        [cyan]{summary.get('total_cards', 0)}[/cyan]")
    console.print(f"  Total Credit Limit: [green]${summary.get('total_credit_limit', 0):,.2f}[/green]")
    console.print(f"  Total Spent:        [red]${summary.get('total_spent', 0):,.2f}[/red]")
    console.print(f"  Total Available:    [blue]${summary.get('total_available', 0):,.2f}[/blue]")
    
    overall_util = summary.get("overall_utilization", 0)
    util_color = "red" if overall_util >= 70 else "yellow" if overall_util >= 30 else "green"
    console.print(f"  Overall Utilization: [{util_color}]{overall_util:.1f}%[/{util_color}]")
    
    cards = summary.get("cards", [])
    if cards:
        console.print("\n[bold]Individual Cards:[/bold]")
        table = Table(box=box.ROUNDED)
        table.add_column("Card", style="cyan")
        table.add_column("Limit", style="green", justify="right")
        table.add_column("Spent", style="red", justify="right")
        table.add_column("Available", style="blue", justify="right")
        table.add_column("Utilization", style="magenta", justify="right")
        
        for card in cards:
            util = card.get("utilization", 0)
            util_color = "red" if util >= 70 else "yellow" if util >= 30 else "green"
            
            table.add_row(
                f"{card.get('card_name', '')} (****{card.get('last_four', '')})",
                f"${card.get('credit_limit', 0):,.2f}",
                f"${card.get('spent', 0):,.2f}",
                f"${card.get('available', 0):,.2f}",
                f"[{util_color}]{util:.1f}%[/{util_color}]"
            )
        
        console.print(table)

def display_family_summary(summary: Dict[str, Any]) -> None:
    """Display family-wide spending summary."""
    if "message" in summary:
        console.print(f"[yellow]{summary['message']}[/yellow]")
        return
    
    console.print(Panel.fit(
        f"[bold]Family Summary - {summary.get('month', '')}[/bold]",
        border_style="green"
    ))
    
    console.print(f"\n[bold]Period:[/bold] {summary.get('period', '')}")
    console.print(f"[bold]Family Total:[/bold] [green]${summary.get('family_total', 0):,.2f}[/green]")
    console.print(f"[bold]Members:[/bold] [cyan]{summary.get('member_count', 0)}[/cyan]")
    console.print(f"[bold]Average per Member:[/bold] [yellow]${summary.get('average_per_member', 0):,.2f}[/yellow]")
    
    members = summary.get("members", [])
    if members:
        console.print("\n[bold]By Member:[/bold]")
        table = Table(box=box.ROUNDED)
        table.add_column("Member", style="cyan")
        table.add_column("Spent", style="green", justify="right")
        table.add_column("Transactions", style="yellow", justify="right")
        table.add_column("Budget", style="blue", justify="right")
        table.add_column("Status", style="magenta")
        table.add_column("Top Category", style="white")
        
        for member in members:
            budget_str = f"${member.get('budget', 0):,.2f}" if member.get('budget') else "N/A"
            
            status_emoji = {
                "within_budget": "✅",
                "over_budget": "🚨",
                "no_budget": "➖"
            }.get(member.get("budget_status", "no_budget"), "❓")
            
            top_cat = member.get("top_category", {})
            top_cat_str = f"{top_cat.get('name', 'N/A')} (${top_cat.get('amount', 0):,.2f})"
            
            table.add_row(
                member.get("name", ""),
                f"${member.get('total_spent', 0):,.2f}",
                str(member.get("transaction_count", 0)),
                budget_str,
                status_emoji,
                top_cat_str
            )
        
        console.print(table)

def display_category_analysis(analysis: Dict[str, Any]) -> None:
    """Display deep dive category analysis."""
    if "message" in analysis:
        console.print(f"[yellow]{analysis['message']}[/yellow]")
        return
    
    console.print(Panel.fit(
        f"[bold]Category Analysis: {analysis.get('category', '')}[/bold]",
        border_style="magenta"
    ))
    
    console.print(f"\n[bold]Period:[/bold] {analysis.get('period', '')}")
    if analysis.get('user_id'):
        console.print(f"[bold]User ID:[/bold] {analysis.get('user_id')}")
    
    summary = analysis.get("summary", {})
    console.print(f"\n[bold]Summary:[/bold]")
    console.print(f"  Total Spent:       [green]${summary.get('total_spent', 0):,.2f}[/green]")
    console.print(f"  Transactions:      [cyan]{summary.get('transaction_count', 0)}[/cyan]")
    console.print(f"  Average:           [yellow]${summary.get('average_transaction', 0):,.2f}[/yellow]")
    console.print(f"  Highest:           [red]${summary.get('highest_transaction', 0):,.2f}[/red]")
    console.print(f"  Lowest:            [blue]${summary.get('lowest_transaction', 0):,.2f}[/blue]")
    
    # By payment method
    by_payment = analysis.get("by_payment_method", {})
    if by_payment:
        console.print("\n[bold]By Payment Method:[/bold]")
        table = Table(box=box.SIMPLE)
        table.add_column("Method", style="magenta")
        table.add_column("Amount", style="green", justify="right")
        
        for method, amount in sorted(by_payment.items(), key=lambda x: x[1], reverse=True):
            table.add_row(method, f"${amount:,.2f}")
        
        console.print(table)
    
    # Monthly trend
    monthly_trend = analysis.get("monthly_trend", {})
    if monthly_trend:
        console.print("\n[bold]Monthly Trend:[/bold]")
        table = Table(box=box.SIMPLE)
        table.add_column("Month", style="blue")
        table.add_column("Amount", style="green", justify="right")
        
        for month, amount in sorted(monthly_trend.items()):
            table.add_row(month, f"${amount:,.2f}")
        
        console.print(table)
    
    # By user (if applicable)
    by_user = analysis.get("by_user")
    if by_user:
        console.print("\n[bold]By User:[/bold]")
        table = Table(box=box.SIMPLE)
        table.add_column("User", style="cyan")
        table.add_column("Amount", style="green", justify="right")
        
        for user, amount in sorted(by_user.items(), key=lambda x: x[1], reverse=True):
            table.add_row(user, f"${amount:,.2f}")
        
        console.print(table)
    
    # Recent transactions
    recent = analysis.get("recent_transactions", [])
    if recent:
        console.print(f"\n[bold]Recent Transactions (showing {len(recent)}):[/bold]")
        table = Table(box=box.SIMPLE)
        table.add_column("Date", style="blue")
        table.add_column("Description", style="white")
        table.add_column("Amount", style="green", justify="right")
        table.add_column("Payment", style="magenta")
        
        for txn in recent:
            table.add_row(
                txn.get("date", ""),
                (txn.get("description", "") or "")[:30],
                f"${txn.get('amount', 0):,.2f}",
                txn.get("payment_method", "")
            )
        
        console.print(table)

def display_spending_trends(trends: Dict[str, Any]) -> None:
    """Display spending trends over multiple months."""
    console.print(Panel.fit(
        "[bold]Spending Trends Analysis[/bold]",
        border_style="blue"
    ))
    
    if trends.get('user_id'):
        console.print(f"\n[bold]User ID:[/bold] {trends.get('user_id')}")
    
    console.print(f"[bold]Months Analyzed:[/bold] {trends.get('months_analyzed', 0)}")
    console.print(f"[bold]Total Spent:[/bold] [green]${trends.get('total_spent', 0):,.2f}[/green]")
    console.print(f"[bold]Average Monthly:[/bold] [yellow]${trends.get('average_monthly', 0):,.2f}[/yellow]")
    
    trend = trends.get("trend", "")
    trend_emoji = {
        "increasing": "📈",
        "decreasing": "📉",
        "stable": "➡️",
        "insufficient_data": "❓"
    }.get(trend, "❓")
    
    trend_color = {
        "increasing": "red",
        "decreasing": "green",
        "stable": "yellow",
        "insufficient_data": "white"
    }.get(trend, "white")
    
    console.print(f"[bold]Trend:[/bold] [{trend_color}]{trend_emoji} {trend}[/{trend_color}]")
    
    monthly_data = trends.get("monthly_data", [])
    if monthly_data:
        console.print("\n[bold]Monthly Breakdown:[/bold]")
        table = Table(box=box.ROUNDED)
        table.add_column("Month", style="blue")
        table.add_column("Spent", style="green", justify="right")
        table.add_column("Transactions", style="yellow", justify="right")
        table.add_column("Top Category", style="magenta")
        table.add_column("Amount", style="cyan", justify="right")
        
        for month in monthly_data:
            top_cat = month.get("top_category", {})
            table.add_row(
                month.get("month", ""),
                f"${month.get('total_spent', 0):,.2f}",
                str(month.get("transaction_count", 0)),
                top_cat.get("name", "N/A"),
                f"${top_cat.get('amount', 0):,.2f}"
            )
        
        console.print(table)

def display_payment_analysis(analysis: Dict[str, Any]) -> None:
    """Display payment method analysis."""
    if "message" in analysis:
        console.print(f"[yellow]{analysis['message']}[/yellow]")
        return
    
    console.print(Panel.fit(
        "[bold]Payment Method Analysis[/bold]",
        border_style="magenta"
    ))
    
    console.print(f"\n[bold]Period:[/bold] {analysis.get('period', '')}")
    if analysis.get('user_id'):
        console.print(f"[bold]User ID:[/bold] {analysis.get('user_id')}")
    
    console.print(f"[bold]Total Spent:[/bold] [green]${analysis.get('total_spent', 0):,.2f}[/green]")
    console.print(f"[bold]Transactions:[/bold] [cyan]{analysis.get('transaction_count', 0)}[/cyan]")
    
    by_payment = analysis.get("by_payment_method", [])
    if by_payment:
        console.print("\n[bold]By Payment Method:[/bold]")
        table = Table(box=box.ROUNDED)
        table.add_column("Method", style="magenta")
        table.add_column("Total", style="green", justify="right")
        table.add_column("Count", style="yellow", justify="right")
        table.add_column("Average", style="blue", justify="right")
        table.add_column("% of Total", style="cyan", justify="right")
        
        for method in by_payment:
            table.add_row(
                method.get("payment_method", ""),
                f"${method.get('total_spent', 0):,.2f}",
                str(method.get("transaction_count", 0)),
                f"${method.get('average_transaction', 0):,.2f}",
                f"{method.get('percentage_of_total', 0):.1f}%"
            )
        
        console.print(table)
    
    # Credit card breakdown
    credit_card_breakdown = analysis.get("credit_card_breakdown")
    if credit_card_breakdown:
        console.print(f"\n[bold]Credit Card Spending:[/bold] [red]${analysis.get('credit_card_total', 0):,.2f}[/red]")
        console.print("\n[bold]By Card:[/bold]")
        table = Table(box=box.SIMPLE)
        table.add_column("Card", style="cyan")
        table.add_column("Amount", style="green", justify="right")
        
        for card, amount in sorted(credit_card_breakdown.items(), key=lambda x: x[1], reverse=True):
            table.add_row(card, f"${amount:,.2f}")
        
        console.print(table)

def main():
    """Main entry point for display module."""
    if len(sys.argv) < 2:
        display_error("No display type specified")
        sys.exit(1)
    
    display_type = sys.argv[1]
    
    # Read JSON from stdin
    try:
        data = json.load(sys.stdin)
    except json.JSONDecodeError as e:
        display_error(f"Invalid JSON input: {e}")
        sys.exit(1)
    
    # Route to appropriate display function
    display_functions = {
        "users": display_users,
        "expenses": display_expenses,
        "budgets": display_budgets,
        "credit_cards": display_credit_cards,
        "budget_status": display_budget_status,
        "monthly_report": display_monthly_report,
        "user_stats": display_user_stats,
        "expense_summary": display_expense_summary,
        "payment_summary": display_payment_summary,
        "budget_alerts": display_budget_alerts,
        "budget_comparison": display_budget_comparison,
        "card_statement": display_card_statement,
        "card_utilization": display_card_utilization,
        "cards_summary": display_cards_summary,
        "family_summary": display_family_summary,
        "category_analysis": display_category_analysis,
        "spending_trends": display_spending_trends,
        "payment_analysis": display_payment_analysis,
    }
    
    if display_type in display_functions:
        display_functions[display_type](data)
    else:
        display_error(f"Unknown display type: {display_type}")
        sys.exit(1)

if __name__ == "__main__":
    main()