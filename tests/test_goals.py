"""Test savings goal CRUD operations."""

import pytest
from conftest import run_cli, run_cli_display


class TestGoalAdd:
    """Test goal add command."""
    
    def test_add_goal_basic(self, test_user):
        """Test adding a goal with required fields."""
        result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Emergency Fund",
            "--target", "10000.00",
            "--deadline", "2026-12-31"
        )
        
        assert result["success"] is True
        assert result["goal"]["name"] == "Emergency Fund"
        assert float(result["goal"]["target_amount"]) == 10000.00
        assert float(result["goal"]["current_amount"]) == 0.00
    
    def test_add_goal_with_current_amount(self, test_user):
        """Test adding a goal with existing savings."""
        result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Vacation Fund",
            "--target", "5000.00",
            "--deadline", "2026-12-31",
            "--current", "1500.00"
        )
        
        assert result["success"] is True
        assert float(result["goal"]["current_amount"]) == 1500.00
    
    def test_add_goal_with_deadline(self, test_user):
        """Test adding a goal with deadline date."""
        result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Car Down Payment",
            "--target", "8000.00",
            "--deadline", "2027-06-30"
        )
        
        assert result["success"] is True
        assert "2027-06-30" in result["goal"]["deadline"]
    
    def test_add_goal_with_description(self, test_user):
        """Test adding a goal with description."""
        result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Home Fund",
            "--target", "50000.00",
            "--deadline", "2030-01-01",
            "--description", "Saving for house down payment"
        )
        
        assert result["success"] is True
        assert result["goal"]["description"] == "Saving for house down payment"


class TestGoalList:
    """Test goal list command."""
    
    def test_list_empty(self, test_user):
        """Test listing when no goals exist."""
        result = run_cli("goal", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 0
    
    def test_list_goals(self, test_user):
        """Test listing multiple goals."""
        # Create goals
        run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Goal 1",
            "--target", "1000.00",
            "--deadline", "2026-12-31"
        )
        run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Goal 2",
            "--target", "2000.00",
            "--deadline", "2027-06-30"
        )
        run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Goal 3",
            "--target", "3000.00",
            "--deadline", "2028-01-01"
        )
        
        result = run_cli("goal", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 3
        
    def test_list_with_summary(self, test_user):
        """Test that list returns summary totals."""
        run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Goal A",
            "--target", "1000.00",
            "--deadline", "2026-12-31",
            "--current", "500.00"
        )
        run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Goal B",
            "--target", "2000.00",
            "--deadline", "2027-06-30",
            "--current", "1000.00"
        )
        
        result = run_cli("goal", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert "summary" in result
        assert result["summary"]["total_target"] == 3000.0
        assert result["summary"]["total_current"] == 1500.0
        assert result["summary"]["overall_progress_percent"] == 50.0


class TestGoalView:
    """Test goal view command."""
    
    def test_view_goal(self, test_user):
        """Test viewing a single goal with progress details."""
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "View Test Goal",
            "--target", "5000.00",
            "--deadline", "2026-12-31",
            "--current", "1000.00"
        )
        goal_id = add_result["goal"]["id"]
        
        result = run_cli("goal", "view", str(goal_id))
        
        assert result["success"] is True
        assert "progress" in result
        assert result["progress"]["goal_id"] == goal_id
        assert result["progress"]["goal_name"] == "View Test Goal"
        assert result["progress"]["target_amount"] == 5000.0
        assert result["progress"]["current_amount"] == 1000.0
        assert result["progress"]["progress_percentage"] == 20.0
        assert result["progress"]["remaining_amount"] == 4000.0
    
    def test_view_goal_required_savings(self, test_user):
        """Test view shows required savings calculations."""
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Savings Rate Goal",
            "--target", "10000.00",
            "--deadline", "2027-01-01",
            "--current", "2000.00"
        )
        goal_id = add_result["goal"]["id"]
        
        result = run_cli("goal", "view", str(goal_id))
        
        assert result["success"] is True
        assert "required_savings" in result["progress"]
        assert "daily" in result["progress"]["required_savings"]
        assert "weekly" in result["progress"]["required_savings"]
        assert "monthly" in result["progress"]["required_savings"]


class TestGoalUpdate:
    """Test goal update command."""
    
    def test_update_target(self, test_user):
        """Test updating goal target amount."""
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Update Target",
            "--target", "5000.00",
            "--deadline", "2026-12-31"
        )
        goal_id = add_result["goal"]["id"]
        
        result = run_cli("goal", "update", str(goal_id), "--target", "7500.00")
        
        assert result["success"] is True
        assert float(result["goal"]["target_amount"]) == 7500.00
    
    def test_update_deadline(self, test_user):
        """Test updating goal deadline."""
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Update Deadline",
            "--target", "5000.00",
            "--deadline", "2026-12-31"
        )
        goal_id = add_result["goal"]["id"]
        
        result = run_cli("goal", "update", str(goal_id), "--deadline", "2027-06-30")
        
        assert result["success"] is True
        assert "2027-06-30" in result["goal"]["deadline"]
    
    def test_update_multiple_fields(self, test_user):
        """Test updating multiple goal fields at once."""
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Multi Update",
            "--target", "5000.00",
            "--deadline", "2026-12-31"
        )
        goal_id = add_result["goal"]["id"]
        
        # Update both target and deadline
        result = run_cli(
            "goal", "update", str(goal_id),
            "--target", "10000.00",
            "--deadline", "2027-06-30"
        )
        
        assert result["success"] is True
        assert float(result["goal"]["target_amount"]) == 10000.00
        assert "2027-06-30" in result["goal"]["deadline"]


class TestGoalContribute:
    """Test goal contribute command."""
    
    def test_contribute_to_goal(self, test_user):
        """Test contributing to a goal."""
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Contribute Test",
            "--target", "1000.00",
            "--deadline", "2026-12-31",
            "--current", "200.00"
        )
        goal_id = add_result["goal"]["id"]
        
        result = run_cli("goal", "contribute", str(goal_id), "--amount", "150.00")
        
        assert result["success"] is True
        assert result["contribution"]["amount"] == 150.0
        assert result["contribution"]["new_current_amount"] == 350.0
        assert result["goal"]["current_amount"] == 350.0
    
    def test_multiple_contributions(self, test_user):
        """Test multiple contributions to a goal."""
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Multi Contribute",
            "--target", "1000.00",
            "--deadline", "2026-12-31"
        )
        goal_id = add_result["goal"]["id"]
        
        contributions = [100.00, 50.00, 75.00, 125.00]
        
        for amount in contributions:
            run_cli("goal", "contribute", str(goal_id), "--amount", f"{amount:.2f}")
        
        # Verify total
        view_result = run_cli("goal", "view", str(goal_id))
        total = view_result["progress"]["current_amount"]
        assert total == sum(contributions)
    
    def test_reach_goal(self, test_user):
        """Test reaching goal target via contribution."""
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Reach Goal",
            "--target", "500.00",
            "--deadline", "2026-12-31",
            "--current", "400.00"
        )
        goal_id = add_result["goal"]["id"]
        
        # Contribute remaining amount
        result = run_cli("goal", "contribute", str(goal_id), "--amount", "100.00")
        
        assert result["success"] is True
        assert result["goal"]["current_amount"] >= result["goal"]["target_amount"]
        
        # Verify progress is at 100%
        view_result = run_cli("goal", "view", str(goal_id))
        assert view_result["progress"]["progress_percentage"] >= 100.0


class TestGoalProgress:
    """Test goal progress command."""
    
    def test_progress_all_goals(self, test_user):
        """Test viewing progress of all goals."""
        # Create multiple goals
        run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Progress Goal 1",
            "--target", "1000.00",
            "--deadline", "2026-12-31",
            "--current", "250.00"
        )
        run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Progress Goal 2",
            "--target", "2000.00",
            "--deadline", "2027-06-30",
            "--current", "1000.00"
        )
        
        result = run_cli("goal", "progress", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 2
        assert "goals_progress" in result
        
        # Verify progress calculations
        goals = result["goals_progress"]
        goal1 = next(g for g in goals if g["name"] == "Progress Goal 1")
        goal2 = next(g for g in goals if g["name"] == "Progress Goal 2")
        
        assert goal1["progress_percent"] == 25.0
        assert goal1["remaining"] == 750.0
        assert goal2["progress_percent"] == 50.0
        assert goal2["remaining"] == 1000.0


class TestGoalDelete:
    """Test goal delete command."""
    
    def test_delete_goal(self, test_user):
        """Test deleting a goal."""
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "To Delete",
            "--target", "1000.00",
            "--deadline", "2026-12-31"
        )
        goal_id = add_result["goal"]["id"]
        
        result = run_cli("goal", "delete", str(goal_id), "--force")
        
        assert result["success"] is True
        assert result["deleted_id"] == goal_id
        
        # Verify goal is gone
        list_result = run_cli("goal", "list", "--user-id", str(test_user["id"]))
        ids = [g["id"] for g in list_result.get("goals", [])]
        assert goal_id not in ids


class TestGoalDisplay:
    """Test goal display output (non-JSON)."""
    
    def test_list_display(self, test_user):
        """Test that goal list displays without errors."""
        run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Display Test Goal",
            "--target", "5000.00",
            "--deadline", "2026-12-31",
            "--current", "1000.00"
        )
        
        returncode, stdout, stderr = run_cli_display("goal", "list", "--user-id", str(test_user["id"]))
        
        assert returncode == 0
        assert "Display Test Goal" in stdout or "5000" in stdout
    
    def test_view_display(self, test_user):
        """Test that goal view displays without errors."""
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "View Display Goal",
            "--target", "2000.00",
            "--deadline", "2026-12-31"
        )
        goal_id = add_result["goal"]["id"]
        
        returncode, stdout, stderr = run_cli_display("goal", "view", str(goal_id))
        
        assert returncode == 0
        assert "View Display Goal" in stdout or "2000" in stdout
    
    def test_progress_display(self, test_user):
        """Test that goal progress displays without errors."""
        run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Progress Display",
            "--target", "3000.00",
            "--deadline", "2026-12-31",
            "--current", "900.00"
        )
        
        returncode, stdout, stderr = run_cli_display("goal", "progress", "--user-id", str(test_user["id"]))
        
        assert returncode == 0
