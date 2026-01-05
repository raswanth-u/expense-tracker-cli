"""Test user CRUD operations."""

import pytest
from conftest import run_cli, run_cli_display


class TestUserAdd:
    """Test user add command."""
    
    def test_add_user_basic(self):
        """Test adding a user with required fields."""
        result = run_cli("user", "add", "--name", "John Doe", "--email", "john@example.com", "--role", "member")
        
        assert result["success"] is True
        assert result["user"]["name"] == "John Doe"
        assert result["user"]["email"] == "john@example.com"
        assert result["user"]["role"] == "member"
        assert result["user"]["is_active"] is True
        assert result["user"]["id"] is not None
    
    def test_add_user_with_role(self):
        """Test adding a user with custom role."""
        result = run_cli("user", "add", "--name", "Admin User", "--email", "admin@example.com", "--role", "admin")
        
        assert result["success"] is True
        assert result["user"]["role"] == "admin"
    
    def test_add_user_member_role(self):
        """Test adding a user with member role."""
        result = run_cli("user", "add", "--name", "Member User", "--email", "member@example.com", "--role", "member")
        
        assert result["success"] is True
        assert result["user"]["role"] == "member"


class TestUserList:
    """Test user list command."""
    
    def test_list_empty(self):
        """Test listing when no users exist."""
        result = run_cli("user", "list")
        
        assert result["success"] is True
        # List returns {"data": [...], "success": true}
        assert len(result["data"]) == 0
    
    def test_list_users(self):
        """Test listing multiple users."""
        # Create users
        run_cli("user", "add", "--name", "User 1", "--email", "user1@example.com")
        run_cli("user", "add", "--name", "User 2", "--email", "user2@example.com")
        run_cli("user", "add", "--name", "User 3", "--email", "user3@example.com")
        
        result = run_cli("user", "list")
        
        assert result["success"] is True
        assert len(result["data"]) == 3
        
        names = [u["name"] for u in result["data"]]
        assert "User 1" in names
        assert "User 2" in names
        assert "User 3" in names


class TestUserView:
    """Test user view command."""
    
    def test_view_user(self, test_user):
        """Test viewing a single user."""
        result = run_cli("user", "view", str(test_user["id"]))
        
        assert result["success"] is True
        # View returns {"data": {...}, "stats": {...}, "success": true}
        assert result["data"]["id"] == test_user["id"]
        assert result["data"]["name"] == "Test User"
        assert result["data"]["email"] == "test@example.com"


class TestUserUpdate:
    """Test user update command."""
    
    def test_update_name(self, test_user):
        """Test updating user name."""
        result = run_cli("user", "update", str(test_user["id"]), "--name", "Updated Name")
        
        assert result["success"] is True
        assert result["user"]["name"] == "Updated Name"
        assert result["user"]["email"] == "test@example.com"  # unchanged
    
    def test_update_email(self, test_user):
        """Test updating user email."""
        result = run_cli("user", "update", str(test_user["id"]), "--email", "newemail@example.com")
        
        assert result["success"] is True
        assert result["user"]["email"] == "newemail@example.com"
    
    def test_update_role(self, test_user):
        """Test updating user role to admin."""
        result = run_cli("user", "update", str(test_user["id"]), "--role", "admin")
        
        assert result["success"] is True
        assert result["user"]["role"] == "admin"
    
    def test_update_multiple_fields(self, test_user):
        """Test updating multiple fields at once."""
        result = run_cli(
            "user", "update", str(test_user["id"]),
            "--name", "New Name",
            "--email", "new@example.com",
            "--role", "admin"
        )
        
        assert result["success"] is True
        assert result["user"]["name"] == "New Name"
        assert result["user"]["email"] == "new@example.com"
        assert result["user"]["role"] == "admin"


class TestUserDelete:
    """Test user delete command."""
    
    def test_delete_user(self, test_user):
        """Test deleting a user with --force."""
        result = run_cli("user", "delete", str(test_user["id"]), "--force")
        
        assert result["success"] is True
        assert result["message"] == f"User {test_user['id']} deleted successfully"


class TestUserStats:
    """Test user stats command."""
    
    def test_user_stats(self, test_user):
        """Test getting user statistics."""
        result = run_cli("user", "stats", str(test_user["id"]))
        
        assert result["success"] is True
        # Stats should exist even if empty
        assert "stats" in result or "data" in result


class TestUserDisplay:
    """Test user display output (non-JSON)."""
    
    def test_list_display(self, test_user):
        """Test that user list displays without errors."""
        returncode, stdout, stderr = run_cli_display("user", "list")
        
        assert returncode == 0
        assert "Test User" in stdout
        assert "test@example.com" in stdout
    
    def test_view_display(self, test_user):
        """Test that user view displays without errors."""
        returncode, stdout, stderr = run_cli_display("user", "view", str(test_user["id"]))
        
        assert returncode == 0
        # Should contain user info
        assert "Test User" in stdout or "test@example.com" in stdout
