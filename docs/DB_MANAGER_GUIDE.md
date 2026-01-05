# Database Manager (db_manager.py) Guide

This guide documents all operations available in `db_manager.py` and demonstrates backup/restore workflows including schema migration scenarios.

## Overview

The `db_manager.py` utility provides database management capabilities:
- **info**: Display database statistics and table row counts
- **backup**: Create timestamped SQL backups with SHA256 checksums
- **restore**: Restore databases from backup files with checksum verification
- **list-backups**: List all available backup files
- **truncate**: Clear all data from tables (preserving schema)

## Prerequisites

```bash
# Activate virtual environment
source /home/life/projects/expense_tracker_app/venv/bin/activate

# Navigate to app directory
cd /home/life/projects/expense_tracker_app/expenses-app-v1
```

## Environment Configuration

The tool supports multiple environments:
- `dev` - Development database (default)
- `staging` - Staging database
- `prod` - Production database

---

## Command Reference

### 1. Database Info

Display database statistics including size, Alembic version, and row counts for all tables.

```bash
python db_manager.py info --env dev
```

**Example Output:**
```
==================================================
  DATABASE INFO: DEV
==================================================

✅ Connected to dev database
Database Size: 8693 kB
Alembic Version: 002_modify_column_type

Tables:
  - alembic_version: 1 rows
  - asset: 0 rows
  - budget: 20 rows
  - creditcard: 20 rows
  - expense: 20 rows
  - user: 21 rows
  ...
ℹ️  Connection closed
```

### 2. Create Backup

Create a timestamped backup with automatic SHA256 checksum generation.

```bash
python db_manager.py backup --env dev
```

**Example Output:**
```
ℹ️  Creating dev database backup...
✅ Backup created: backups/backup_dev_20260105_091756.sql (0.05 MB)
✅ Checksum saved to backups/backup_dev_20260105_091756.sql.sha256
```

**Backup Location:** `expenses-app-v1/backups/backup_{env}_{timestamp}.sql`

### 3. List Backups

Display all available backup files with size and checksum status.

```bash
python db_manager.py list-backups --env dev
```

**Example Output:**
```
==================================================
  AVAILABLE BACKUPS: DEV
==================================================

  1. backup_dev_20260105_091756.sql
     Size: 0.05 MB
     Checksum: ✅ Available

  2. backup_dev_20260105_091756_modified.sql
     Size: 0.05 MB
     Checksum: ❌ Missing
```

### 4. Restore Backup

Restore a database from a backup file. Includes checksum verification.

```bash
python db_manager.py restore --env dev --file backups/backup_dev_20260105_091756.sql
```

**Example Output:**
```
⚠️  ⚠️  This will restore data to dev database!
⚠️     Backup file: backups/backup_dev_20260105_091756.sql
✅ Checksum verified
ℹ️  Restoring database...
✅ Database restored from backups/backup_dev_20260105_091756.sql
```

### 5. Truncate Tables

Clear all data from tables while preserving schema structure.

```bash
python db_manager.py truncate --env dev
```

**⚠️ Warning:** This permanently deletes all data. Use with caution!

---

## Workflow Examples

### Workflow 1: Basic Backup and Restore

```bash
# Step 1: Check current state
python db_manager.py info --env dev

# Step 2: Create backup
python db_manager.py backup --env dev

# Step 3: Verify backup exists
python db_manager.py list-backups --env dev

# Step 4: (Optional) Make changes or truncate
python db_manager.py truncate --env dev

# Step 5: Restore from backup
python db_manager.py restore --env dev --file backups/backup_dev_20260105_091756.sql

# Step 6: Verify restoration
python db_manager.py info --env dev
```

### Workflow 2: Backup Before Schema Migration

This workflow demonstrates handling schema changes with existing data.

```bash
# Step 1: Populate test data (20 entries per table)
python populate_test_data.py

# Step 2: Verify data
python db_manager.py info --env dev

# Step 3: Create pre-migration backup
python db_manager.py backup --env dev
# Output: backups/backup_dev_20260105_091756.sql

# Step 4: Clear tables for schema modification
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  -c "TRUNCATE TABLE expense, budget, creditcard CASCADE;"

# Step 5: Apply schema changes
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  -c "ALTER TABLE expense RENAME COLUMN description TO notes;"
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  -c "ALTER TABLE expense ADD COLUMN priority VARCHAR(20) DEFAULT 'normal';"

# Step 6: Verify schema changes
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  -c "\d expense"
```

### Workflow 3: Restore with Schema Mismatch

When the backup schema differs from current schema, you need to modify the backup file.

**Problem:** Backup has `description` column, table now has `notes` column.

**Solution:** Modify the backup file to match new schema:

```bash
# Step 1: Create a modified copy of backup
cd backups
cp backup_dev_20260105_091756.sql backup_dev_20260105_091756_modified.sql

# Step 2: Replace old column name with new
sed -i 's/description/notes/g' backup_dev_20260105_091756_modified.sql

# Step 3: Restore modified backup
docker exec -i expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  < backup_dev_20260105_091756_modified.sql

# Step 4: Verify restoration
python db_manager.py info --env dev

# Step 5: Check data in new column structure
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  -c "SELECT id, notes, priority FROM expense LIMIT 5;"
```

**Expected Result:**
```
  id  |     notes      | priority 
------+----------------+----------
 1851 | Test expense 1 | normal
 1852 | Test expense 2 | normal
```

### Workflow 4: Revert Schema Changes

If you need to undo schema modifications:

```bash
# Remove added column
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  -c "ALTER TABLE expense DROP COLUMN priority;"

# Rename column back
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  -c "ALTER TABLE expense RENAME COLUMN notes TO description;"

# Verify original schema
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  -c "\d expense"
```

---

## Database Connection Details

| Environment | Host | Port | Database | User |
|-------------|------|------|----------|------|
| dev | localhost | 5433 | expenses_db | expense_admin |
| staging | localhost | 5434 | expenses_db | expense_admin |
| prod | localhost | 5435 | expenses_db | expense_admin |

---

## Direct PostgreSQL Commands

For advanced operations, connect directly to the database:

```bash
# Connect interactively
docker exec -it expenses-app-v1-db-1 psql -U expense_admin -d expenses_db

# Run single command
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db -c "YOUR SQL HERE"
```

### Useful SQL Commands

```sql
-- List all tables
\dt

-- Describe table structure
\d expense

-- Count rows in all tables
SELECT schemaname, relname, n_live_tup 
FROM pg_stat_user_tables 
ORDER BY n_live_tup DESC;

-- Truncate all data tables
TRUNCATE TABLE expense, budget, creditcard, savingsgoal, 
               recurringexpensetemplate, debitcard, asset,
               creditcardtransaction, savingsaccount, 
               savingsaccounttransaction CASCADE;

-- Check foreign key constraints
SELECT
    tc.constraint_name,
    tc.table_name,
    kcu.column_name,
    ccu.table_name AS foreign_table_name,
    ccu.column_name AS foreign_column_name
FROM information_schema.table_constraints AS tc
JOIN information_schema.key_column_usage AS kcu
    ON tc.constraint_name = kcu.constraint_name
JOIN information_schema.constraint_column_usage AS ccu
    ON ccu.constraint_name = tc.constraint_name
WHERE tc.constraint_type = 'FOREIGN KEY';
```

---

## Troubleshooting

### Issue: Restore fails with "column does not exist"

**Cause:** Schema has changed since backup was created.

**Solution:** Modify backup file to match current schema:
```bash
sed -i 's/old_column_name/new_column_name/g' backup_file.sql
```

### Issue: Checksum verification fails

**Cause:** Backup file was modified after creation.

**Solution:** 
1. Check if you have an unmodified backup
2. Regenerate checksum: `sha256sum backup_file.sql > backup_file.sql.sha256`

### Issue: Foreign key constraint violations during truncate

**Solution:** Use CASCADE option:
```sql
TRUNCATE TABLE expense CASCADE;
```

### Issue: Restore shows "relation already exists" errors

**Cause:** Backup includes CREATE TABLE statements and tables already exist.

**Solution:** These errors are typically harmless - the data is still restored. If you need a clean restore:
```bash
# Drop and recreate database
docker exec expenses-app-v1-db-1 psql -U expense_admin -c "DROP DATABASE expenses_db; CREATE DATABASE expenses_db;"
# Then restore
```

---

## Best Practices

1. **Always backup before schema changes** - Create a backup before any ALTER TABLE operations
2. **Verify backups** - Use `list-backups` to confirm checksum availability
3. **Test restores** - Periodically test that backups can be restored successfully
4. **Use staging first** - Test backup/restore procedures on staging before production
5. **Document schema changes** - Keep track of column renames and additions for backup modifications
6. **Store checksums** - Always keep the .sha256 files alongside backups
7. **Clean old backups** - Periodically remove old backups to save disk space

---

## Session Log: Complete db_manager.py Test

Below is the complete session demonstrating all db_manager.py operations:

```bash
# 1. Populate test data
python populate_test_data.py
# Result: Created 20 entries in each table

# 2. Verify data
python db_manager.py info --env dev
# Result: expense: 20 rows, budget: 20 rows, creditcard: 20 rows, etc.

# 3. Create backup
python db_manager.py backup --env dev
# Result: backups/backup_dev_20260105_091756.sql (0.05 MB)

# 4. Truncate all tables
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  -c "TRUNCATE TABLE expense, recurringexpensetemplate, budget, creditcard, 
      creditcardtransaction, savingsgoal, savingsaccount, 
      savingsaccounttransaction, debitcard, asset CASCADE;"
# Result: TRUNCATE TABLE

# 5. Verify truncation
python db_manager.py info --env dev
# Result: All tables show 0 rows

# 6. Modify schema
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  -c "ALTER TABLE expense RENAME COLUMN description TO notes;"
# Result: ALTER TABLE

docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  -c "ALTER TABLE expense ADD COLUMN priority VARCHAR(20) DEFAULT 'normal';"
# Result: ALTER TABLE

# 7. Attempt restore (fails silently for mismatched columns)
python db_manager.py restore --env dev --file backups/backup_dev_20260105_091756.sql
# Result: expense table empty due to column mismatch

# 8. Create modified backup
cp backups/backup_dev_20260105_091756.sql backups/backup_dev_20260105_091756_modified.sql
sed -i 's/description/notes/g' backups/backup_dev_20260105_091756_modified.sql

# 9. Restore modified backup
docker exec -i expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  < backups/backup_dev_20260105_091756_modified.sql
# Result: Data restored with column mapping

# 10. Verify restoration
python db_manager.py info --env dev
# Result: expense: 20 rows ✓

docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  -c "SELECT id, notes, priority FROM expense LIMIT 5;"
# Result: Shows data in 'notes' column with 'priority' = 'normal'

# 11. Revert schema (optional)
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db \
  -c "ALTER TABLE expense DROP COLUMN priority; 
      ALTER TABLE expense RENAME COLUMN notes TO description;"
# Result: Original schema restored
```

---

*Last Updated: January 5, 2026*
*Tested with: PostgreSQL 15, Python 3.11, expenses-app-v1*
