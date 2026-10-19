# Contract Upgrade Migration Framework

Hunty contracts share a migration framework via the `hunty-migration` crate.

## Version detection

Call `initialize_schema(admin)` once after deploy. `get_schema_version()` returns `0` for legacy storage and `CURRENT_SCHEMA_VERSION` after initialization.

## Running migrations

```rust
// Simulate without writes
let report = run_migration(admin, target_version, true);

// Apply migrations
let report = run_migration(admin, target_version, false);
```

`MigrationReport` includes `from_version`, `to_version`, `steps_applied`, `dry_run`, and `succeeded`.

## Rollback

Before each applied migration, the previous version is stored. Call `rollback_migration(admin)` to restore it.

> **Limitation:** `Rollback_migration` only restores the stored schema version. It does *not* undo data transforms. If a data-transforming step (for example `migrate_v2_to_v3`) has run since the rollback point was recorded, `rollback_migration` **will panic and refuse to roll back**. Restoring the version in that case would leave the contract reading new-format data while believing it is on the old schema.

Migration steps are classified as either `VersionOnly` or `Data`:

- `VersionOnly` steps (e.g. bumping the stored version, assigning a new metadata key) are safe to roll back.
- `Data` steps transform existing storage and have no down-migration. Once any `Data` step has applied, rollback is refused.

To roll back across a data step, you must either:

1. Add an explicit down-migration for the step and mark it as `VersionOnly` once the down-path exists, or
2. Restore from a backup taken before the data step applied.

## Per-contract steps

| Contract | v0 → v1 | v1 → v2 | v2 → v3 |
|----------|---------|---------|---------|
| HuntyCore | Backfill `required_clues` from `total_clues` (Data) | Reserved (VersionOnly) | Reserved (Data) |
| RewardManager | Bump schema version (VersionOnly) | Reserved (VersionOnly) | Reserved |
| NftReward | Assign metadata version key (`(NVER, nft_id)`) for legacy NFTs (VersionOnly) | Reserved (VersionOnly) | Reserved |
