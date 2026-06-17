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

## Getting Started

### 1. Clone the repository

```bash id="2v9c8p"
git clone https://github.com/<your-username>/zero2prod.git
cd zero2prod
