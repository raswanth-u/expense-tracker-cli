"""Test credit card CRUD operations."""

import pytest
from conftest import run_cli, run_cli_display


class TestCardAdd:
    """Test card add command."""
    
    def test_add_card_basic(self, test_user):
        """Test adding a credit card with required fields."""
        result = run_cli(
            "card", "add",
            "--user-id", str(test_user["id"]),
            "--name", "My Card",
            "--last-four", "1234",
            "--limit", "5000.00",
            "--billing-day", "15"
        )
        
        assert result["success"] is True
        assert result["card"]["card_name"] == "My Card"
        assert float(result["card"]["credit_limit"]) == 5000.00
        assert result["card"]["billing_day"] == 15
    
    def test_add_card_with_tags(self, test_user):
        """Test adding a card with tags."""
        result = run_cli(
            "card", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Tagged Card",
            "--last-four", "5678",
            "--limit", "10000.00",
            "--tags", "travel,rewards"
        )
        
        assert result["success"] is True
        assert result["card"]["tags"] == "travel,rewards"


class TestCardList:
    """Test card list command."""
    
    def test_list_empty(self, test_user):
        """Test listing when no cards exist."""
        result = run_cli("card", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 0
    
    def test_list_cards(self, test_user):
        """Test listing multiple cards."""
        # Create cards
        run_cli(
            "card", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Card 1",
            "--last-four", "1111",
            "--limit", "5000.00"
        )
        run_cli(
            "card", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Card 2",
            "--last-four", "2222",
            "--limit", "10000.00"
        )
        
        result = run_cli("card", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 2


class TestCardView:
    """Test card view command."""
    
    def test_view_card(self, test_credit_card):
        """Test viewing a single card."""
        result = run_cli("card", "view", str(test_credit_card["id"]))
        
        assert result["success"] is True
        assert result["card"]["id"] == test_credit_card["id"]
        assert result["card"]["card_name"] == "Test Card"


class TestCardUpdate:
    """Test card update command."""
    
    def test_update_limit(self, test_credit_card):
        """Test updating credit limit."""
        result = run_cli(
            "card", "update", str(test_credit_card["id"]),
            "--limit", "8000.00"
        )
        
        assert result["success"] is True
        assert float(result["card"]["credit_limit"]) == 8000.00
    
    def test_update_name(self, test_credit_card):
        """Test updating card name."""
        result = run_cli(
            "card", "update", str(test_credit_card["id"]),
            "--name", "Renamed Card"
        )
        
        assert result["success"] is True
        assert result["card"]["card_name"] == "Renamed Card"


class TestCardDelete:
    """Test card delete command."""
    
    def test_delete_card(self, test_user):
        """Test deleting a card."""
        add_result = run_cli(
            "card", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Delete Me",
            "--last-four", "9999",
            "--limit", "1000.00"
        )
        card_id = add_result["id"]
        
        result = run_cli("card", "delete", str(card_id), "--force")
        
        assert result["success"] is True
        
        # Verify card is gone
        list_result = run_cli("card", "list", "--user-id", str(test_user["id"]))
        ids = [c["id"] for c in list_result["cards"]]
        assert card_id not in ids


class TestCardPayment:
    """Test card payment command."""
    
    def test_make_payment(self, test_user, test_credit_card, test_account):
        """Test making a payment to a credit card."""
        # First create an expense on the card
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Shopping",
            "--payment", "credit_card",
            "--card-id", str(test_credit_card["id"])
        )
        
        # Now make a payment from savings account
        result = run_cli(
            "card", "pay", str(test_credit_card["id"]),
            "--amount", "50.00",
            "--from-account", str(test_account["id"])
        )
        
        assert result["success"] is True


class TestCardStatement:
    """Test card statement command."""
    
    def test_statement(self, test_user, test_credit_card):
        """Test viewing card statement."""
        # Create some expenses
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Food",
            "--payment", "credit_card",
            "--card-id", str(test_credit_card["id"])
        )
        
        result = run_cli("card", "statement", str(test_credit_card["id"]),
                        "--month", "2026-01")
        
        assert result["success"] is True


class TestCardUtilization:
    """Test card utilization command."""
    
    def test_utilization(self, test_user, test_credit_card):
        """Test viewing utilization for all cards."""
        result = run_cli("card", "utilization")
        
        assert result["success"] is True


class TestCardTransactions:
    """Test card transactions command."""
    
    def test_transactions(self, test_credit_card):
        """Test viewing card transactions."""
        result = run_cli("card", "transactions", str(test_credit_card["id"]))
        
        assert result["success"] is True


class TestCardDisplay:
    """Test card display output (non-JSON)."""
    
    def test_list_display(self, test_user):
        """Test that card list displays without errors."""
        run_cli(
            "card", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Display Card",
            "--last-four", "0000",
            "--limit", "5000.00"
        )
        
        returncode, stdout, stderr = run_cli_display(
            "card", "list", "--user-id", str(test_user["id"])
        )
        
        assert returncode == 0
    
    def test_view_display(self, test_credit_card):
        """Test that card view displays without errors."""
        returncode, stdout, stderr = run_cli_display(
            "card", "view", str(test_credit_card["id"])
        )
        
        assert returncode == 0


class TestCardSummary:
    """Test credit card summary functionality."""

    def test_cards_utilization(self, test_credit_card):
        """Test getting utilization of all credit cards."""
        result = run_cli("card", "utilization", "--months", "1")

        assert result["success"] is True

    def test_card_with_expense_utilization(self, test_user, test_credit_card):
        """Test card utilization after expense."""
        # Add expense to credit card
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "500.00",
            "--category", "Shopping",
            "--date", "2026-01-15",
            "--payment", "credit_card",
            "--card-id", str(test_credit_card["id"])
        )

        # Check utilization for all cards
        result = run_cli("card", "utilization", "--months", "1")

        assert result["success"] is True

    def test_card_statement_december(self, test_user, test_credit_card):
        """Test card statement for December (edge case)."""
        result = run_cli("card", "statement", str(test_credit_card["id"]), "--month", "2025-12")

        assert result["success"] is True


class TestCardWithExpenses:
    """Test credit card operations with expenses."""

    def test_card_view_with_utilization(self, test_user, test_credit_card):
        """Test viewing a card shows utilization details."""
        # Add expense to credit card
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "1000.00",
            "--category", "Shopping",
            "--date", "2026-01-15",
            "--payment", "credit_card",
            "--card-id", str(test_credit_card["id"])
        )

        # View the card (should show utilization)
        result = run_cli("card", "view", str(test_credit_card["id"]))
        assert result["success"] is True
        # Card view returns 'card' key, not 'credit_card'
        assert "card" in result or "statistics" in result

    def test_card_transactions_list(self, test_user, test_credit_card):
        """Test listing transactions for a specific card."""
        # Add expenses
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Food",
            "--date", "2026-01-10",
            "--payment", "credit_card",
            "--card-id", str(test_credit_card["id"])
        )
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "200.00",
            "--category", "Entertainment",
            "--date", "2026-01-12",
            "--payment", "credit_card",
            "--card-id", str(test_credit_card["id"])
        )

        # Get transactions
        result = run_cli("card", "transactions", str(test_credit_card["id"]))
        assert result["success"] is True
