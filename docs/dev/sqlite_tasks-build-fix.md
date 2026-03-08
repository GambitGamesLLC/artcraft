# sqlite_tasks build fix (2026-03-08)

## Symptom

`sqlite_tasks` failed to compile with many SQLx macro errors like:

- `error returned from database: (code: 1) no such table: tasks`
- Follow-on type errors from failed macro expansion (E0282/E0277)

The reproduced output is saved at:

- `build-logs/sqlite_tasks-failure.txt`

## Root cause

`sqlx::query!` / `query_as!` macros in `sqlite_tasks` were compiling against a live SQLite DB path (`DATABASE_URL=sqlite:/tmp/tasks.sqlite`) that does not have the `tasks` schema initialized at compile time.

Because SQLx online validation failed, macro expansion cascaded into additional Rust type errors.

## Minimal local fix applied

Set SQLx compile-time mode to offline in `sqlite_tasks` build script so macro validation uses committed `.sqlx` metadata instead of requiring a live migrated DB.

Changed file:

- `crates/schema/database/sqlite_tasks/build.rs`

Changes:

- Added `cargo:rustc-env=SQLX_OFFLINE=true` in both Unix and Windows paths.

## Verification used

- `cargo check -p sqlite_tasks` passes after this change.

## Notes

- No migration run is required for this fix.
- No sqlx metadata regeneration is required for this specific patch.
- If queries change later, regenerate sqlx offline metadata with project-standard sqlx prepare workflow.
