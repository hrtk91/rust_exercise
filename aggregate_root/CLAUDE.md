# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Build and Test
- `cargo build` - Build the project
- `cargo test` - Run all tests including integration tests
- `cargo test integration_test` - Run the specific integration test in handler.rs
- `cargo check` - Quick syntax and type checking
- `cargo clippy` - Run linting

### Database Operations
- Database migrations are in `migrations/` directory
- Uses SQLx with SQLite backend
- Set `DATABASE_URL` environment variable or defaults to `sqlite::memory:`
- Migration runs automatically in tests via `sqlx::migrate!()`

## Architecture Overview

This is a Rust DDD (Domain-Driven Design) exercise implementing an aggregate root pattern with users and departments.

### Layer Structure
- **Kernel** (`src/kernel.rs`): Domain entities and repository interfaces
  - `UserAggregateRoot` entity with embedded `Department` collection
  - `UserAggregateRepository` trait defining domain operations
- **Adapter** (`src/adapter.rs`): Infrastructure layer with SQLx repository implementation
  - SQLite connection pool with LazyLock initialization
  - Complex JOIN query handling to reconstruct aggregate from normalized tables
- **Handler** (`src/handler.rs`): Application layer with CQRS pattern
  - Query/Command separation with dedicated DTOs
  - Handlers for `fetch_user_by_id` and `create_user` operations

### Key Design Patterns
- **Aggregate Root**: User entity aggregates Department entities
- **Repository Pattern**: Abstract database operations behind traits
- **CQRS**: Separate query and command models with DTOs
- **Transaction Management**: Uses SQLx transactions for data consistency

### Database Schema
- `users` table with basic user information
- `departments` table with foreign key to users
- Aggregate reconstruction via LEFT JOIN in queries

### Testing Strategy
Integration test in `handler.rs` uses in-memory SQLite database with full migration setup.