# Active Context

This document contains the current work focus, recent changes, and next steps.

## Current Work Focus

- Refactoring the backend for better modularity and removing unnecessary code.

## Recent Changes

- Created `models.rs` to define data models.
- Created `db.rs` to handle database interactions.
- Refactored `handlers.rs` to use the new `db.rs` module.
- Removed unnecessary `root` and `test_supabase` handlers and routes.
- Updated `main.rs` to include the new modules.

## Next Steps

- The backend refactoring is complete.
