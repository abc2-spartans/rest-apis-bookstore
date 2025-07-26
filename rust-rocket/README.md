# Rust Rocket Bookstore API

A simple REST API for managing books, built with Rocket web framework and SQLite database using Diesel ORM.

## Running the API

### Standard Run
```sh
cd rust-rocket
cargo run
```

### Live Reload (Development)
For automatic reloading when files change:

1. Install cargo-watch (if not already installed):
   ```sh
   cargo install cargo-watch
   ```

2. Run with live reload:
   ```sh
   cargo watch -x run
   ```

The server will automatically restart when you make changes to the source code.

## Endpoints

- `GET /` — Health check
- `GET /api/v1/books` — List all books
- `GET /api/v1/books/<id>` — Get a book by id
- `POST /api/v1/books` — Add a new book (JSON: `{ "title", "author", "published_year" }`)
- `PUT /api/v1/books/<id>` — Update a book
- `DELETE /api/v1/books/<id>` — Delete a book

## Database & ORM
- **Database**: SQLite, file: `bookstore.db` (auto-created)
- **ORM**: [Diesel](https://diesel.rs/) - A safe, extensible ORM and Query Builder for Rust
- **Connection Pool**: `rocket_sync_db_pools` for database connection management
- **Configuration**: Database settings in `Rocket.toml`
- **Schema**: Defined using Diesel's `table!` macro in `src/main.rs`


## Resources
- [Rocket Documentation](https://rocket.rs/guide/)
- [Diesel Getting Started Guide](https://diesel.rs/guides/getting-started)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Serde Documentation](https://serde.rs/)
- [SQLite Documentation](https://www.sqlite.org/docs.html)
