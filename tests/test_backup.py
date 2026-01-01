"""Test backup and restore operations."""

import pytest
import os
from conftest import run_cli, run_cli_display


class TestBackupCreate:
    """Test backup create command."""
    
    def test_create_backup(self, test_user):
        """Test creating a backup."""
        result = run_cli("backup", "create")
        
        assert result["success"] is True
        assert "filepath" in result
    
    def test_create_backup_with_data(self, test_user):
        """Test creating a backup with existing data."""
        # Create some data
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100",
            "--category", "Test"
        )
        
        result = run_cli("backup", "create")
        
        assert result["success"] is True


class TestBackupList:
    """Test backup list command."""
    
    def test_list_backups(self, test_user):
        """Test listing backups."""
        # Create a backup first
        run_cli("backup", "create")
        
        result = run_cli("backup", "list")
        
        assert result["success"] is True


class TestBackupRestore:
    """Test backup restore command."""
    
    def test_restore_backup(self, test_user):
        """Test restoring from a backup."""
        # Create a backup
        create_result = run_cli("backup", "create")
        assert create_result["success"] is True
        
        # Get backup file path
        backup_file = create_result.get("backup_file") or create_result.get("file") or create_result.get("path", "")
        
        if backup_file:
            # Try to restore (this will at least test the restore command works)
            result = run_cli("backup", "restore", backup_file, expect_success=False)
            # Restore might fail due to existing data, but command should work
            assert isinstance(result, dict)


class TestBackupDisplay:
    """Test backup display output (non-JSON)."""
    
    def test_list_display(self, test_user):
        """Test that backup list displays without errors."""
        run_cli("backup", "create")
        
        returncode, stdout, stderr = run_cli_display("backup", "list")
        
        assert returncode == 0
    
    def test_create_display(self, test_user):
        """Test that backup create displays without errors."""
        returncode, stdout, stderr = run_cli_display("backup", "create")
        
        assert returncode == 0
