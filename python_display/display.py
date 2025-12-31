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

def display_dashboard(dashboard: Dict[str, Any]) -> None:
    """Display comprehensive dashboard."""
    from rich.layout import Layout
    from rich.panel import Panel
    
    console.print(Panel.fit(
        f"[bold cyan]Financial Dashboard[/bold cyan]\n[dim]{dashboard.get('generated_at', '')}[/dim]",
        border_style="cyan"
    ))
    
    # TODAY'S SUMMARY
    today = dashboard.get("today", {})
    console.print("\n[bold]📅 Today's Summary[/bold]")
    console.print(f"  Date: [cyan]{today.get('date', '')}[/cyan]")
    console.print(f"  Total Spent: [red]${today.get('total', 0):,.2f}[/red]")
    console.print(f"  Transactions: [yellow]{today.get('count', 0)}[/yellow]")
    console.print(f"  Largest Expense: [magenta]${today.get('largest', 0):,.2f}[/magenta]")
    
    # Show today's transactions if any
    today_txns = today.get("transactions", [])
    if today_txns:
        console.print("\n[bold]Today's Transactions:[/bold]")
        table = Table(box=box.SIMPLE, show_header=False)
        table.add_column("", style="cyan")
        table.add_column("", style="white")
        table.add_column("", style="green", justify="right")
        
        for txn in today_txns[:5]:  # Show max 5
            desc = txn.get("description", "") or txn.get("category", "")
            table.add_row(
                txn.get("payment_method", ""),
                desc[:30],
                f"${txn.get('amount', 0):,.2f}"
            )
        console.print(table)
    
    # WEEK COMPARISON
    week = dashboard.get("week", {})
    week_change = week.get("change", 0)
    week_change_pct = week.get("change_percentage", 0)
    
    change_color = "red" if week_change > 0 else "green" if week_change < 0 else "yellow"
    change_symbol = "↑" if week_change > 0 else "↓" if week_change < 0 else "→"
    
    console.print("\n[bold]📊 This Week vs Last Week[/bold]")
    console.print(f"  This Week: [cyan]${week.get('total', 0):,.2f}[/cyan]")
    console.print(f"  Last Week: [blue]${week.get('last_week_total', 0):,.2f}[/blue]")
    console.print(f"  Change: [{change_color}]{change_symbol} ${abs(week_change):,.2f} ({week_change_pct:+.1f}%)[/{change_color}]")
    
    # MONTH SUMMARY
    month = dashboard.get("month", {})
    budget_pct = month.get("percentage", 0)
    
    budget_color = "red" if budget_pct >= 100 else "yellow" if budget_pct >= 80 else "green"
    
    console.print(f"\n[bold]📈 This Month ({month.get('month', '')})[/bold]")
    console.print(f"  Total Spent: [red]${month.get('total_spent', 0):,.2f}[/red]")
    console.print(f"  Total Budget: [green]${month.get('total_budget', 0):,.2f}[/green]")
    console.print(f"  Remaining: [cyan]${month.get('remaining', 0):,.2f}[/cyan]")
    console.print(f"  Usage: [{budget_color}]{budget_pct:.1f}%[/{budget_color}]")
    console.print(f"  Avg Daily: [yellow]${month.get('avg_daily', 0):,.2f}[/yellow]")
    console.print(f"  Days Elapsed: [dim]{month.get('days_in_month', 0)}[/dim]")
    
    # BUDGET ALERTS
    alerts = dashboard.get("budget_alerts", {})
    alerts_count = alerts.get("count", 0)
    
    if alerts_count > 0:
        console.print(f"\n[bold red]⚠️  Budget Alerts: {alerts_count}[/bold red]")
        
        alert_budgets = alerts.get("budgets", [])
        warning_budgets = [b for b in alert_budgets if b.get("status") in ["warning", "exceeded"]]
        
        if warning_budgets:
            table = Table(box=box.SIMPLE)
            table.add_column("Category", style="cyan")
            table.add_column("Status", style="red")
            table.add_column("Used", style="yellow", justify="right")
            
            for budget in warning_budgets[:3]:  # Show top 3
                status_emoji = "🚨" if budget.get("status") == "exceeded" else "⚠️"
                table.add_row(
                    budget.get("category", ""),
                    f"{status_emoji} {budget.get('percentage', 0):.1f}%",
                    f"${budget.get('spent', 0):,.2f}"
                )
            
            console.print(table)
    else:
        console.print("\n[bold green]✓ All Budgets OK[/bold green]")
    
    # CREDIT CARDS
    cards = dashboard.get("credit_cards", {})
    total_cards = cards.get("total_cards", 0)
    
    if total_cards > 0:
        utilization = cards.get("utilization", 0)
        util_color = "red" if utilization >= 70 else "yellow" if utilization >= 30 else "green"
        
        console.print(f"\n[bold]💳 Credit Cards ({total_cards})[/bold]")
        console.print(f"  Total Limit: [green]${cards.get('total_limit', 0):,.2f}[/green]")
        console.print(f"  Total Used: [red]${cards.get('total_spent', 0):,.2f}[/red]")
        console.print(f"  Utilization: [{util_color}]{utilization:.1f}%[/{util_color}]")
    
    # RECENT TRANSACTIONS
    recent = dashboard.get("recent_transactions", [])
    if recent:
        console.print("\n[bold]🕐 Recent Transactions[/bold]")
        table = Table(box=box.ROUNDED)
        table.add_column("Date", style="blue")
        table.add_column("Category", style="magenta")
        table.add_column("Description", style="white")
        table.add_column("Amount", style="green", justify="right")
        
        for txn in recent:
            table.add_row(
                txn.get("date", ""),
                txn.get("category", ""),
                (txn.get("description", "") or "")[:30],
                f"${txn.get('amount', 0):,.2f}"
            )
        
        console.print(table)
    
    # QUICK STATS
    stats = dashboard.get("quick_stats", {})
    console.print("\n[bold]⚡ Quick Stats[/bold]")
    console.print(f"  Average Daily Spending: [yellow]${stats.get('avg_daily_spending', 0):,.2f}[/yellow]")
    console.print(f"  Largest Expense Today: [magenta]${stats.get('largest_expense_today', 0):,.2f}[/magenta]")
    console.print(f"  Days in Month: [cyan]{stats.get('days_elapsed', 0)}[/cyan]")
    
    console.print()

def display_savings_goals(goals: List[Dict[str, Any]]) -> None:
    """Display savings goals in a formatted table."""
    if not goals:
        console.print("[yellow]No savings goals found[/yellow]")
        return
    
    table = Table(title="💰 Savings Goals", box=box.ROUNDED)
    table.add_column("ID", style="cyan", justify="right")
    table.add_column("Name", style="green")
    table.add_column("Target", style="blue", justify="right")
    table.add_column("Current", style="yellow", justify="right")
    table.add_column("Progress", style="magenta", justify="right")
    table.add_column("Deadline", style="red")
    
    for goal in goals:
        target = goal.get("target_amount", 0)
        current = goal.get("current_amount", 0)
        progress = (current / target * 100) if target > 0 else 0
        
        progress_color = "green" if progress >= 100 else "yellow" if progress >= 50 else "red"
        
        table.add_row(
            str(goal.get("id", "")),
            goal.get("name", ""),
            f"${target:,.2f}",
            f"${current:,.2f}",
            f"[{progress_color}]{progress:.1f}%[/{progress_color}]",
            goal.get("deadline", "")
        )
    
    console.print(table)

def display_savings_goal_progress(progress: Dict[str, Any]) -> None:
    """Display detailed savings goal progress."""
    console.print(Panel.fit(
        f"[bold]Goal Progress: {progress.get('goal_name', '')}[/bold]",
        border_style="green"
    ))
    
    target = progress.get("target_amount", 0)
    current = progress.get("current_amount", 0)
    remaining = progress.get("remaining_amount", 0)
    percentage = progress.get("progress_percentage", 0)
    days_remaining = progress.get("days_remaining", 0)
    status = progress.get("status", "")
    
    # Progress bar
    progress_color = "green" if percentage >= 100 else "yellow" if percentage >= 50 else "red"
    
    console.print(f"\n[bold]Target:[/bold] [blue]${target:,.2f}[/blue]")
    console.print(f"[bold]Current:[/bold] [green]${current:,.2f}[/green]")
    console.print(f"[bold]Remaining:[/bold] [yellow]${remaining:,.2f}[/yellow]")
    console.print(f"[bold]Progress:[/bold] [{progress_color}]{percentage:.1f}%[/{progress_color}]")
    console.print(f"[bold]Deadline:[/bold] {progress.get('deadline', '')}")
    console.print(f"[bold]Days Remaining:[/bold] {days_remaining}")
    
    # Status
    status_messages = {
        "completed": "🎉 Goal Completed!",
        "overdue": "⏰ Overdue",
        "urgent": "🚨 Urgent (< 30 days)",
        "just_started": "🌱 Just Started",
        "on_track": "✅ On Track",
        "halfway": "📊 Halfway There",
        "almost_there": "🎯 Almost There"
    }
    
    console.print(f"\n[bold]Status:[/bold] {status_messages.get(status, status)}")
    
    # Required savings
    if progress.get("is_achievable", True) and remaining > 0:
        required = progress.get("required_savings", {})
        console.print(f"\n[bold]Required Savings:[/bold]")
        console.print(f"  Daily:   [cyan]${required.get('daily', 0):,.2f}[/cyan]")
        console.print(f"  Weekly:  [yellow]${required.get('weekly', 0):,.2f}[/yellow]")
        console.print(f"  Monthly: [magenta]${required.get('monthly', 0):,.2f}[/magenta]")

def display_assets(assets: List[Dict[str, Any]]) -> None:
    """Display assets in a formatted table."""
    if not assets:
        console.print("[yellow]No assets found[/yellow]")
        return
    
    table = Table(title="🏠 Assets", box=box.ROUNDED)
    table.add_column("ID", style="cyan", justify="right")
    table.add_column("Name", style="green")
    table.add_column("Type", style="magenta")
    table.add_column("Purchase", style="blue", justify="right")
    table.add_column("Current", style="yellow", justify="right")
    table.add_column("Gain/Loss", style="white", justify="right")
    table.add_column("Location", style="dim")
    
    for asset in assets:
        purchase = asset.get("purchase_value", 0)
        current = asset.get("current_value", 0)
        gain_loss = current - purchase
        gain_loss_pct = (gain_loss / purchase * 100) if purchase > 0 else 0
        
        gain_loss_color = "green" if gain_loss >= 0 else "red"
        gain_loss_symbol = "+" if gain_loss >= 0 else ""
        
        table.add_row(
            str(asset.get("id", "")),
            asset.get("name", ""),
            asset.get("asset_type", ""),
            f"${purchase:,.2f}",
            f"${current:,.2f}",
            f"[{gain_loss_color}]{gain_loss_symbol}${gain_loss:,.2f} ({gain_loss_pct:+.1f}%)[/{gain_loss_color}]",
            asset.get("location", "")[:20] if asset.get("location") else ""
        )
    
    console.print(table)

def display_assets_summary(summary: Dict[str, Any]) -> None:
    """Display assets summary."""
    if "message" in summary:
        console.print(f"[yellow]{summary['message']}[/yellow]")
        return
    
    console.print(Panel.fit(
        "[bold]Assets Summary[/bold]",
        border_style="green"
    ))
    
    total_purchase = summary.get("total_purchase_value", 0)
    total_current = summary.get("total_current_value", 0)
    total_gain_loss = summary.get("total_gain_loss", 0)
    gain_loss_pct = summary.get("gain_loss_percentage", 0)
    
    gain_loss_color = "green" if total_gain_loss >= 0 else "red"
    
    console.print(f"\n[bold]Overview:[/bold]")
    console.print(f"  Total Assets:      [cyan]{summary.get('total_assets', 0)}[/cyan]")
    console.print(f"  Purchase Value:    [blue]${total_purchase:,.2f}[/blue]")
    console.print(f"  Current Value:     [yellow]${total_current:,.2f}[/yellow]")
    console.print(f"  Gain/Loss:         [{gain_loss_color}]${total_gain_loss:,.2f} ({gain_loss_pct:+.1f}%)[/{gain_loss_color}]")
    
    # By type
    by_type = summary.get("by_type", {})
    if by_type:
        console.print("\n[bold]By Asset Type:[/bold]")
        table = Table(box=box.ROUNDED)
        table.add_column("Type", style="magenta")
        table.add_column("Count", style="cyan", justify="right")
        table.add_column("Purchase", style="blue", justify="right")
        table.add_column("Current", style="yellow", justify="right")
        table.add_column("Gain/Loss", style="white", justify="right")
        
        for asset_type, data in sorted(by_type.items(), key=lambda x: x[1].get("current_value", 0), reverse=True):
            gain_loss = data.get("gain_loss", 0)
            gain_loss_pct = data.get("gain_loss_percentage", 0)
            gain_loss_color = "green" if gain_loss >= 0 else "red"
            
            table.add_row(
                asset_type,
                str(data.get("count", 0)),
                f"${data.get('purchase_value', 0):,.2f}",
                f"${data.get('current_value', 0):,.2f}",
                f"[{gain_loss_color}]${gain_loss:,.2f} ({gain_loss_pct:+.1f}%)[/{gain_loss_color}]"
            )
        
        console.print(table)
    
    # By user
    by_user = summary.get("by_user")
    if by_user:
        console.print("\n[bold]By User:[/bold]")
        table = Table(box=box.SIMPLE)
        table.add_column("User", style="cyan")
        table.add_column("Count", style="yellow", justify="right")
        table.add_column("Total Value", style="green", justify="right")
        
        for user, data in sorted(by_user.items(), key=lambda x: x[1].get("total_value", 0), reverse=True):
            table.add_row(
                user,
                str(data.get("count", 0)),
                f"${data.get('total_value', 0):,.2f}"
            )
        
        console.print(table)

def display_asset_depreciation(depreciation: Dict[str, Any]) -> None:
    """Display asset depreciation analysis."""
    if "message" in depreciation:
        console.print(f"[yellow]{depreciation['message']}[/yellow]")
        return
    
    console.print(Panel.fit(
        "[bold]Asset Depreciation Analysis[/bold]",
        border_style="red"
    ))
    
    console.print(f"\n[bold]Total Assets:[/bold] {depreciation.get('total_assets', 0)}")
    console.print(f"[bold]Total Depreciation:[/bold] [red]${depreciation.get('total_depreciation', 0):,.2f}[/red]")
    
    assets = depreciation.get("assets", [])
    if assets:
        console.print("\n[bold]Depreciation by Asset:[/bold]")
        table = Table(box=box.ROUNDED)
        table.add_column("Asset", style="cyan")
        table.add_column("Type", style="magenta")
        table.add_column("Age (years)", style="blue", justify="right")
        table.add_column("Purchase", style="green", justify="right")
        table.add_column("Current", style="yellow", justify="right")
        table.add_column("Depreciation", style="red", justify="right")
        table.add_column("Annual", style="dim", justify="right")
        
        for asset in assets:
            table.add_row(
                asset.get("name", ""),
                asset.get("type", ""),
                f"{asset.get('age_years', 0):.1f}",
                f"${asset.get('purchase_value', 0):,.2f}",
                f"${asset.get('current_value', 0):,.2f}",
                f"${asset.get('depreciation', 0):,.2f} ({asset.get('depreciation_percentage', 0):.1f}%)",
                f"${asset.get('annual_depreciation', 0):,.2f}/yr"
            )
        
        console.print(table)

def display_recurring_templates(templates: List[Dict[str, Any]]) -> None:
    """Display recurring expense templates."""
    if not templates:
        console.print("[yellow]No recurring templates found[/yellow]")
        return
    
    table = Table(title="🔄 Recurring Expense Templates", box=box.ROUNDED)
    table.add_column("ID", style="cyan", justify="right")
    table.add_column("Description", style="green")
    table.add_column("Amount", style="yellow", justify="right")
    table.add_column("Category", style="magenta")
    table.add_column("Frequency", style="blue")
    table.add_column("Next", style="red")
    table.add_column("Payment", style="white")
    
    for template in templates:
        freq = template.get("frequency", "")
        interval = template.get("interval", 1)
        
        # Format frequency display
        if freq == "custom":
            freq_display = f"Every {interval} days"
        elif interval > 1:
            freq_display = f"Every {interval} {freq}"
        else:
            freq_display = freq.capitalize()
        
        table.add_row(
            str(template.get("id", "")),
            (template.get("description", "") or template.get("category", ""))[:30],
            f"${template.get('amount', 0):,.2f}",
            template.get("category", ""),
            freq_display,
            template.get("next_occurrence", ""),
            template.get("payment_method", "")
        )
    
    console.print(table)
    
    # Show summary
    total_monthly_est = 0
    for template in templates:
        amount = template.get("amount", 0)
        freq = template.get("frequency", "")
        interval = template.get("interval", 1)
        
        # Estimate monthly cost
        if freq == "daily":
            monthly = amount * 30 / interval
        elif freq == "weekly":
            monthly = amount * 4.33 / interval
        elif freq == "monthly":
            monthly = amount / interval
        elif freq == "yearly":
            monthly = amount / 12 / interval
        elif freq == "custom":
            monthly = amount * 30 / interval
        else:
            monthly = 0
        
        total_monthly_est += monthly
    
    console.print(f"\n[bold]Estimated Monthly Total:[/bold] [yellow]${total_monthly_est:,.2f}[/yellow]")

def display_upcoming_recurring(upcoming: Dict[str, Any]) -> None:
    """Display upcoming recurring expenses."""
    console.print(Panel.fit(
        f"[bold]Upcoming Recurring Expenses - {upcoming.get('period', '')}[/bold]",
        border_style="yellow"
    ))
    
    count = upcoming.get("count", 0)
    
    if count == 0:
        console.print("\n[green]✓ No recurring expenses due in this period[/green]")
        return
    
    console.print(f"\n[bold]Total Upcoming:[/bold] [yellow]{count}[/yellow]")
    
    expenses = upcoming.get("upcoming_expenses", [])
    
    if not expenses:
        return
    
    # Group by status
    today_expenses = [e for e in expenses if e.get("status") == "today"]
    upcoming_expenses = [e for e in expenses if e.get("status") == "upcoming"]
    
    # Show due today
    if today_expenses:
        console.print("\n[bold red]🔴 Due Today:[/bold red]")
        table = Table(box=box.SIMPLE)
        table.add_column("Description", style="white")
        table.add_column("Amount", style="red", justify="right")
        table.add_column("Category", style="magenta")
        table.add_column("Payment", style="cyan")
        
        for expense in today_expenses:
            table.add_row(
                expense.get("description", "") or expense.get("category", ""),
                f"${expense.get('amount', 0):,.2f}",
                expense.get("category", ""),
                expense.get("payment_method", "")
            )
        
        console.print(table)
    
    # Show upcoming
    if upcoming_expenses:
        console.print("\n[bold yellow]📅 Upcoming:[/bold yellow]")
        table = Table(box=box.ROUNDED)
        table.add_column("Days", style="cyan", justify="right")
        table.add_column("Date", style="blue")
        table.add_column("Description", style="white")
        table.add_column("Amount", style="yellow", justify="right")
        table.add_column("Category", style="magenta")
        
        for expense in upcoming_expenses:
            days_until = expense.get("days_until", 0)
            
            table.add_row(
                str(days_until),
                expense.get("next_occurrence", ""),
                expense.get("description", "") or expense.get("category", ""),
                f"${expense.get('amount', 0):,.2f}",
                expense.get("category", "")
            )
        
        console.print(table)
    
    # Calculate total
    total_upcoming = sum(e.get("amount", 0) for e in expenses)
    console.print(f"\n[bold]Total Amount:[/bold] [yellow]${total_upcoming:,.2f}[/yellow]")

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
        "dashboard": display_dashboard,
        "savings_goals": display_savings_goals,
        "savings_goal_progress": display_savings_goal_progress,
        "assets": display_assets,
        "assets_summary": display_assets_summary,
        "asset_depreciation": display_asset_depreciation,
        "recurring_templates": display_recurring_templates,      # Add this line
        "upcoming_recurring": display_upcoming_recurring,        # Add this line
    }
    
    if display_type in display_functions:
        display_functions[display_type](data)
    else:
        display_error(f"Unknown display type: {display_type}")
        sys.exit(1)

if __name__ == "__main__":
    main()