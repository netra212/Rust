+++
title = "Axum SQLx — Stored Procedures (Solution)"
description = "Reference implementation: calling PostgreSQL stored procedures from Rust using SQLx query_as!"
+++


## Background & Objectives

### What are stored procedures?

Stored procedures are SQL logic that lives inside the database rather than in your application code. Three main use cases:

1. **Reusable complex logic** — a query with multiple JOINs, aggregations, or conditions you want to call from several places without duplicating it (e.g. `get_author_stats`)
2. **Performance** — the procedure is compiled and optimised on the DB side once. For heavy computations on large datasets, it's faster than pulling data into Rust and processing it there
3. **Atomicity** — a stored procedure can perform multiple operations (insert + update + audit log) in a single transaction, with no network round-trips between each step

In practice today, many teams prefer keeping logic in application code (easier to version, test, and debug). Stored procedures are most common in **enterprise / legacy / DBA-driven** contexts — which is exactly why it's useful to know how to call them from SQLx.

---

Reference implementation for the stored procedures exercise.

Open the project files and study how stored procedures are:
1. **Defined** in `migrations/20240322000002_create_stored_procedures.sql`
2. **Called** from Rust in `src/domain/stats/stats.rs` using `query_as!`

### Run the Program

```bash
cargo run
```

## Key Concepts

- **Stored procedures** defined in SQL migrations (`CREATE OR REPLACE FUNCTION`)
- **Calling stored procedures** from SQLx: `SELECT * FROM function_name($1)`
- **`query_as!` macro**: maps procedure output columns to a Rust struct via `#[derive(FromRow)]`
- **`RETURNS TABLE`**: how PostgreSQL stored procedures return result sets
- **`fetch_optional` vs `fetch_all`**: single optional row vs multiple rows
