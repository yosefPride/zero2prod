# Zero To Production (Rust)

A backend web service built while working through *Zero To Production in Rust*.

This project focuses on building a production-grade web API using Rust, Actix Web, and modern backend engineering practices including testing, CI, and security tooling.

---

## Tech Stack

- Rust (stable)
- Actix Web
- Tokio (async runtime)
- SQLx (database access)
- PostgreSQL
- Docker (for local development)
- GitHub Actions (CI)

---

## Features

- RESTful API design
- Async web server with Actix Web
- Database integration with PostgreSQL
- Fully tested endpoints
- CI pipeline with automated checks
- Code formatting and lint enforcement

---

## Project Structure

```text
src/
  main.rs        # application entry point
  routes/        # HTTP handlers
  domain/        # business logic
  db/            # database access layer
  config/        # configuration management
tests/           # integration tests
```

---

## Getting Started

### 1. Clone the repository

```bash id="2v9c8p"
git clone https://github.com/yosefPride/zero2prod.git
cd zero2prod
```
---

### 2. Install dependencies

Make sure you have:

* Rust (stable toolchain)
* PostgreSQL
* sqlx-cli (optional, for migrations)

### 3. Run the application
cargo run

The server will start locally (default: http://127.0.0.1:8000).

---

## Running Tests
cargo test

--- 

## Code Quality
### Formatting
```bash
cargo fmt -- --check
```
### Linting
```bash
cargo clippy -- -D warnings
```

---

## CI Pipeline

Every push and pull request triggers GitHub Actions:

Build the project
* Run tests
* Check formatting
* Run Clippy linting

CI must pass before merging.

---

## Optional Tools
### Test Coverage
* cargo tarpaulin --ignore-tests
* Security Audit
* cargo audit

---

## Development Philosophy
* correctness first
* explicit error handling
* reproducible builds
* strict CI enforcement
* minimal assumptions about runtime environment

---

## Status

Work in progress (learning project).

---

## License

This project is for educational purposes.
