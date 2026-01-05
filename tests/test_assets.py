"""Test asset CRUD operations."""

import pytest
from conftest import run_cli, run_cli_display


class TestAssetAdd:
    """Test asset add command."""
    
    def test_add_asset_basic(self, test_user):
        """Test adding an asset with required fields."""
        result = run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Car",
            "--asset-type", "vehicle",
            "--purchase-value", "30000.00",
            "--current-value", "25000.00",
            "--purchase-date", "2024-01-15"
        )
        
        assert result["success"] is True
        assert result["asset"]["name"] == "Car"
        assert result["asset"]["asset_type"] == "vehicle"
        assert float(result["asset"]["current_value"]) == 25000.00
        assert float(result["asset"]["purchase_value"]) == 30000.00
    
    def test_add_asset_with_description(self, test_user):
        """Test adding an asset with description."""
        result = run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Laptop",
            "--asset-type", "electronics",
            "--purchase-value", "2000.00",
            "--current-value", "1500.00",
            "--purchase-date", "2024-06-01",
            "--description", "MacBook Pro 14 inch"
        )
        
        assert result["success"] is True
        assert result["asset"]["description"] == "MacBook Pro 14 inch"
    
    def test_add_asset_with_location(self, test_user):
        """Test adding an asset with location."""
        result = run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Investment Property",
            "--asset-type", "property",
            "--purchase-value", "300000.00",
            "--current-value", "350000.00",
            "--purchase-date", "2020-03-15",
            "--location", "123 Main St, Downtown"
        )
        
        assert result["success"] is True
        assert result["asset"]["location"] == "123 Main St, Downtown"
    
    def test_add_asset_with_tags(self, test_user):
        """Test adding an asset with tags."""
        result = run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Gold Ring",
            "--asset-type", "jewelry",
            "--purchase-value", "5000.00",
            "--current-value", "6000.00",
            "--purchase-date", "2023-12-25",
            "--tags", "gift,gold,valuable"
        )
        
        assert result["success"] is True
        assert result["asset"]["tags"] == "gift,gold,valuable"


class TestAssetList:
    """Test asset list command."""
    
    def test_list_empty(self, test_user):
        """Test listing when no assets exist."""
        result = run_cli("asset", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 0
    
    def test_list_assets(self, test_user):
        """Test listing multiple assets."""
        # Create assets
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Car",
            "--asset-type", "vehicle",
            "--purchase-value", "30000.00",
            "--current-value", "25000.00",
            "--purchase-date", "2024-01-15"
        )
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "House",
            "--asset-type", "property",
            "--purchase-value", "400000.00",
            "--current-value", "450000.00",
            "--purchase-date", "2020-01-01"
        )
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Watch",
            "--asset-type", "jewelry",
            "--purchase-value", "5000.00",
            "--current-value", "6000.00",
            "--purchase-date", "2023-06-01"
        )
        
        result = run_cli("asset", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 3
    
    def test_list_with_summary(self, test_user):
        """Test that list returns summary totals."""
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Asset A",
            "--asset-type", "other",
            "--purchase-value", "10000.00",
            "--current-value", "12000.00",
            "--purchase-date", "2024-01-01"
        )
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Asset B",
            "--asset-type", "other",
            "--purchase-value", "5000.00",
            "--current-value", "4000.00",
            "--purchase-date", "2024-06-01"
        )
        
        result = run_cli("asset", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert "summary" in result
        assert result["summary"]["total_purchase_value"] == 15000.0
        assert result["summary"]["total_current_value"] == 16000.0
        assert result["summary"]["net_change"] == 1000.0
    
    def test_list_filter_by_type(self, test_user):
        """Test filtering assets by type."""
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Car 1",
            "--asset-type", "vehicle",
            "--purchase-value", "20000.00",
            "--current-value", "18000.00",
            "--purchase-date", "2024-01-01"
        )
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "House",
            "--asset-type", "property",
            "--purchase-value", "300000.00",
            "--current-value", "350000.00",
            "--purchase-date", "2020-01-01"
        )
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Car 2",
            "--asset-type", "vehicle",
            "--purchase-value", "25000.00",
            "--current-value", "22000.00",
            "--purchase-date", "2023-06-01"
        )
        
        result = run_cli("asset", "list", "--user-id", str(test_user["id"]), "--asset-type", "vehicle")
        
        assert result["success"] is True
        assert result["count"] == 2
        for asset in result["assets"]:
            assert asset["asset_type"] == "vehicle"


class TestAssetView:
    """Test asset view command."""
    
    def test_view_asset(self, test_user):
        """Test viewing a single asset."""
        add_result = run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "View Test Asset",
            "--asset-type", "vehicle",
            "--purchase-value", "30000.00",
            "--current-value", "25000.00",
            "--purchase-date", "2024-01-15"
        )
        asset_id = add_result["asset"]["id"]
        
        result = run_cli("asset", "view", str(asset_id))
        
        assert result["success"] is True
        assert result["asset"]["id"] == asset_id
        assert result["asset"]["name"] == "View Test Asset"
    
    def test_view_asset_with_analysis(self, test_user):
        """Test view shows value analysis."""
        add_result = run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Analysis Test",
            "--asset-type", "vehicle",
            "--purchase-value", "25000.00",
            "--current-value", "20000.00",
            "--purchase-date", "2024-01-15"
        )
        asset_id = add_result["asset"]["id"]
        
        result = run_cli("asset", "view", str(asset_id))
        
        assert result["success"] is True
        assert "analysis" in result
        assert result["analysis"]["value_change"] == -5000.0
        assert result["analysis"]["change_percent"] == -20.0


class TestAssetUpdate:
    """Test asset update command."""
    
    def test_update_name(self, test_user):
        """Test updating asset name."""
        add_result = run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Original Name",
            "--asset-type", "vehicle",
            "--purchase-value", "20000.00",
            "--current-value", "18000.00",
            "--purchase-date", "2024-01-01"
        )
        asset_id = add_result["asset"]["id"]
        
        result = run_cli("asset", "update", str(asset_id), "--name", "Updated Asset Name")
        
        assert result["success"] is True
        assert result["asset"]["name"] == "Updated Asset Name"
    
    def test_update_current_value(self, test_user):
        """Test updating asset current value."""
        add_result = run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Value Update Test",
            "--asset-type", "investment",
            "--purchase-value", "10000.00",
            "--current-value", "11000.00",
            "--purchase-date", "2024-01-01"
        )
        asset_id = add_result["asset"]["id"]
        
        result = run_cli("asset", "update", str(asset_id), "--current-value", "12500.00")
        
        assert result["success"] is True
        assert float(result["asset"]["current_value"]) == 12500.00
    
    def test_update_location(self, test_user):
        """Test updating asset location."""
        add_result = run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Location Test",
            "--asset-type", "property",
            "--purchase-value", "200000.00",
            "--current-value", "220000.00",
            "--purchase-date", "2022-01-01"
        )
        asset_id = add_result["asset"]["id"]
        
        result = run_cli("asset", "update", str(asset_id), "--location", "456 Oak Ave")
        
        assert result["success"] is True
        assert result["asset"]["location"] == "456 Oak Ave"


class TestAssetValue:
    """Test asset value update command."""
    
    def test_update_value(self, test_user):
        """Test updating asset value directly."""
        add_result = run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Value Command Test",
            "--asset-type", "investment",
            "--purchase-value", "10000.00",
            "--current-value", "11000.00",
            "--purchase-date", "2024-01-01"
        )
        asset_id = add_result["asset"]["id"]
        
        result = run_cli("asset", "value", str(asset_id), "15000.00")
        
        assert result["success"] is True
        assert result["new_value"] == 15000.0
        assert result["id"] == asset_id


class TestAssetDepreciation:
    """Test asset depreciation analysis."""
    
    def test_depreciation_analysis(self, test_user):
        """Test depreciation analysis shows assets."""
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Depreciation Test Car",
            "--asset-type", "vehicle",
            "--purchase-value", "30000.00",
            "--current-value", "20000.00",
            "--purchase-date", "2023-01-01"
        )
        
        result = run_cli("asset", "depreciation", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert "depreciation" in result
        assert result["depreciation"]["total_assets"] == 1
        assert len(result["depreciation"]["assets"]) == 1
        assert result["depreciation"]["assets"][0]["name"] == "Depreciation Test Car"


class TestAssetSummary:
    """Test asset summary command."""
    
    def test_summary_by_type(self, test_user):
        """Test summary grouped by asset type."""
        # Add vehicle
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Car",
            "--asset-type", "vehicle",
            "--purchase-value", "30000.00",
            "--current-value", "25000.00",
            "--purchase-date", "2024-01-01"
        )
        # Add property
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "House",
            "--asset-type", "property",
            "--purchase-value", "300000.00",
            "--current-value", "350000.00",
            "--purchase-date", "2020-01-01"
        )
        
        result = run_cli("asset", "summary", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert "summary" in result
        assert result["summary"]["total_assets"] == 2
        assert result["summary"]["total_purchase_value"] == 330000.0
        assert result["summary"]["total_current_value"] == 375000.0
        assert "by_type" in result["summary"]
        assert "vehicle" in result["summary"]["by_type"]
        assert "property" in result["summary"]["by_type"]
    
    def test_summary_gain_loss(self, test_user):
        """Test summary shows total gain/loss."""
        # Add appreciating asset
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Investment",
            "--asset-type", "investment",
            "--purchase-value", "10000.00",
            "--current-value", "15000.00",
            "--purchase-date", "2024-01-01"
        )
        # Add depreciating asset
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Electronics",
            "--asset-type", "electronics",
            "--purchase-value", "3000.00",
            "--current-value", "1000.00",
            "--purchase-date", "2024-01-01"
        )
        
        result = run_cli("asset", "summary", "--user-id", str(test_user["id"]))
        
        # Total gain/loss: +5000 - 2000 = +3000
        assert result["success"] is True
        assert result["summary"]["total_gain_loss"] == 3000.0


class TestAssetDelete:
    """Test asset delete command."""
    
    def test_delete_asset(self, test_user):
        """Test deleting an asset."""
        add_result = run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "To Delete",
            "--asset-type", "other",
            "--purchase-value", "1000.00",
            "--current-value", "900.00",
            "--purchase-date", "2024-06-01"
        )
        asset_id = add_result["asset"]["id"]
        
        result = run_cli("asset", "delete", str(asset_id), "--force")
        
        assert result["success"] is True
        assert result["id"] == asset_id
        
        # Verify asset is gone
        list_result = run_cli("asset", "list", "--user-id", str(test_user["id"]))
        ids = [a["id"] for a in list_result.get("assets", [])]
        assert asset_id not in ids


class TestAssetDisplay:
    """Test asset display output (non-JSON)."""
    
    def test_list_display(self, test_user):
        """Test that asset list displays without errors."""
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Display Test Asset",
            "--asset-type", "vehicle",
            "--purchase-value", "40000.00",
            "--current-value", "35000.00",
            "--purchase-date", "2024-01-01"
        )
        
        returncode, stdout, stderr = run_cli_display("asset", "list", "--user-id", str(test_user["id"]))
        
        assert returncode == 0
        assert "Display Test Asset" in stdout or "35000" in stdout
    
    def test_view_display(self, test_user):
        """Test that asset view displays without errors."""
        add_result = run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "View Display Asset",
            "--asset-type", "property",
            "--purchase-value", "500000.00",
            "--current-value", "550000.00",
            "--purchase-date", "2020-01-01"
        )
        asset_id = add_result["asset"]["id"]
        
        returncode, stdout, stderr = run_cli_display("asset", "view", str(asset_id))
        
        assert returncode == 0
    
    def test_summary_display(self, test_user):
        """Test that asset summary displays without errors."""
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Summary Display",
            "--asset-type", "investment",
            "--purchase-value", "10000.00",
            "--current-value", "12000.00",
            "--purchase-date", "2024-01-01"
        )
        
        returncode, stdout, stderr = run_cli_display("asset", "summary", "--user-id", str(test_user["id"]))
        
        assert returncode == 0
    
    def test_depreciation_display(self, test_user):
        """Test that asset depreciation displays without errors."""
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Depreciation Display",
            "--asset-type", "vehicle",
            "--purchase-value", "30000.00",
            "--current-value", "25000.00",
            "--purchase-date", "2024-01-01"
        )
        
        returncode, stdout, stderr = run_cli_display("asset", "depreciation", "--user-id", str(test_user["id"]))
        
        assert returncode == 0
