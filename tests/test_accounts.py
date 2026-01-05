"""Test savings account CRUD operations."""

import pytest
from conftest import run_cli, run_cli_display


class TestAccountAdd:
    """Test account add command."""
    
    def test_add_account_basic(self, test_user):
        """Test adding an account with required fields."""
        result = run_cli(
            "account", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Savings Account",
            "--bank", "Test Bank",
            "--last-four", "1234"
        )
        
        assert result["success"] is True
        assert result["account"]["account_name"] == "Savings Account"
        assert result["account"]["bank_name"] == "Test Bank"
        assert result["account"]["account_type"] == "savings"  # default
    
    def test_add_account_with_balance(self, test_user):
        """Test adding an account with initial balance."""
        result = run_cli(
            "account", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Balance Account",
            "--bank", "Balance Bank",
            "--last-four", "5678",
            "--balance", "5000.00"
        )
        
        assert result["success"] is True
        assert result["initial_deposit"] == 5000.00
    
    def test_add_checking_account(self, test_user):
        """Test adding a checking account."""
        result = run_cli(
            "account", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Checking",
            "--bank", "Check Bank",
            "--last-four", "9012",
            "--account-type", "checking"
        )
        
        assert result["success"] is True
        assert result["account"]["account_type"] == "checking"


class TestAccountList:
    """Test account list command."""
    
    def test_list_empty(self, test_user):
        """Test listing when no accounts exist."""
        result = run_cli("account", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 0
        assert len(result["accounts"]) == 0
    
    def test_list_accounts(self, test_user):
        """Test listing multiple accounts."""
        # Create accounts
        run_cli(
            "account", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Account 1",
            "--bank", "Bank A",
            "--last-four", "1111"
        )
        run_cli(
            "account", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Account 2",
            "--bank", "Bank B",
            "--last-four", "2222",
            "--account-type", "checking"
        )
        
        result = run_cli("account", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 2
        assert len(result["accounts"]) == 2


class TestAccountView:
    """Test account view command."""
    
    def test_view_account(self, test_account):
        """Test viewing a single account."""
        result = run_cli("account", "view", str(test_account["id"]))
        
        assert result["success"] is True
        assert result["account_id"] == test_account["id"]
        assert "summary" in result


class TestAccountUpdate:
    """Test account update command."""
    
    def test_update_name(self, test_account):
        """Test updating account name."""
        result = run_cli(
            "account", "update", str(test_account["id"]),
            "--name", "Updated Account Name"
        )
        
        assert result["success"] is True
        assert result["account"]["account_name"] == "Updated Account Name"
    
    def test_update_min_balance(self, test_account):
        """Test updating minimum balance."""
        result = run_cli(
            "account", "update", str(test_account["id"]),
            "--min-balance", "500.00"
        )
        
        assert result["success"] is True
        assert float(result["account"]["minimum_balance"]) == 500.00
    
    def test_update_interest_rate(self, test_account):
        """Test updating interest rate."""
        result = run_cli(
            "account", "update", str(test_account["id"]),
            "--interest-rate", "3.5"
        )
        
        assert result["success"] is True
        assert float(result["account"]["interest_rate"]) == 3.5


class TestAccountDeposit:
    """Test account deposit command."""
    
    def test_deposit(self, test_account):
        """Test depositing to account."""
        # Get initial balance from account view summary
        view_result = run_cli("account", "view", str(test_account["id"]))
        initial_balance = float(view_result["summary"]["current_balance"])
        
        result = run_cli(
            "account", "deposit", str(test_account["id"]),
            "--amount", "500.00",
            "--description", "Test deposit"
        )
        
        assert result["success"] is True
        # Verify balance increased
        view_result = run_cli("account", "view", str(test_account["id"]))
        new_balance = float(view_result["summary"]["current_balance"])
        assert new_balance == initial_balance + 500.00


class TestAccountWithdraw:
    """Test account withdraw command."""
    
    def test_withdraw(self, test_account):
        """Test withdrawing from account."""
        # Get balance before withdrawal
        view_result = run_cli("account", "view", str(test_account["id"]))
        initial_balance = float(view_result["summary"]["current_balance"])
        
        result = run_cli(
            "account", "withdraw", str(test_account["id"]),
            "--amount", "200.00",
            "--description", "Test withdrawal"
        )
        
        assert result["success"] is True
        # Verify balance decreased
        view_result = run_cli("account", "view", str(test_account["id"]))
        new_balance = float(view_result["summary"]["current_balance"])
        assert new_balance == initial_balance - 200.00


class TestAccountTransactions:
    """Test account transactions command."""
    
    def test_list_transactions(self, test_account):
        """Test listing account transactions."""
        # Create transaction via deposit
        run_cli(
            "account", "deposit", str(test_account["id"]),
            "--amount", "100.00",
            "--description", "Deposit 1"
        )
        
        result = run_cli("account", "transactions", str(test_account["id"]))
        
        assert result["success"] is True
        assert "transactions" in result


class TestAccountDelete:
    """Test account delete command."""
    
    def test_delete_account(self, test_user):
        """Test deleting an account."""
        # Create a fresh account to delete
        add_result = run_cli(
            "account", "add",
            "--user-id", str(test_user["id"]),
            "--name", "To Delete",
            "--bank", "Delete Bank",
            "--last-four", "9999"
        )
        account_id = add_result["account"]["id"]
        
        result = run_cli("account", "delete", str(account_id), "--force")
        
        assert result["success"] is True
        
        # Verify account is gone
        list_result = run_cli("account", "list", "--user-id", str(test_user["id"]))
        ids = [a["id"] for a in list_result.get("accounts", [])]
        assert account_id not in ids


class TestAccountDisplay:
    """Test account display output (non-JSON)."""
    
    def test_list_display(self, test_account, test_user):
        """Test that account list displays without errors."""
        returncode, stdout, stderr = run_cli_display("account", "list", "--user-id", str(test_user["id"]))
        
        assert returncode == 0
    
    def test_view_display(self, test_account):
        """Test that account view displays without errors."""
        returncode, stdout, stderr = run_cli_display("account", "view", str(test_account["id"]))
        
        assert returncode == 0


class TestAccountSummary:
    """Test account summary functionality."""

    def test_all_accounts_summary(self, test_user, test_account):
        """Test getting summary of all accounts."""
        # Create another account
        run_cli(
            "account", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Second Account",
            "--bank", "Another Bank",
            "--last-four", "5678",
            "--balance", "1000.00"
        )

        result = run_cli("account", "summary")
        assert result["success"] is True

    def test_account_with_transactions_summary(self, test_user):
        """Test account summary with transactions."""
        # Create account
        add_result = run_cli(
            "account", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Transaction Account",
            "--bank", "Test Bank",
            "--last-four", "9012",
            "--balance", "500.00"
        )
        account_id = add_result["account"]["id"]

        # Add deposit
        run_cli("account", "deposit", str(account_id), "--amount", "200.00", "--description", "Deposit")

        # Add withdrawal
        run_cli("account", "withdraw", str(account_id), "--amount", "50.00", "--description", "Withdrawal")

        # Get summary
        result = run_cli("account", "summary")
        assert result["success"] is True
