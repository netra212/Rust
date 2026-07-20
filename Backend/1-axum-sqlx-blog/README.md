+++
title = "Axum SQLX Blog with PostgreSQL"
description = "Blog application with authors and posts using database relationships"
+++


## Background & Objectives

Build a blog application with Axum, SQLX, and PostgreSQL. This exercise teaches database relationships, foreign keys, and CRUD operations across related entities.

## Specs

Complete the TODOs in:
- `src/domain/author/author.rs` - Implement **create**, **update**, **delete** for authors
- `src/domain/blog/blog.rs` - Implement **get_by_id**, **get_all_by_author_id**, **create**, **update**, **delete** for blogs

Study the reference implementations:
- `Author::get_all` and `Author::get_by_id` - Shows basic SQLX queries
- `Blog::get_all` - Shows ordering by timestamp

### Run the Program

```bash
cargo run
```

### Test the Program

```bash
cargo test
```

Test with `requests.http` or curl.

## Key Concepts

- Database migrations with SQLX
- Foreign key relationships (blogs → authors)
- CRUD operations for related entities
- Async database operations and connection pooling