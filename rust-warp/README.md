# Rust Wrap Bookstore API

A simple REST API for managing books, using Wrap and SQLite.

## Running the API

### Standard Run
```sh
cd rust-wrap
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

## Endpoints

- `GET /` — Health check
- `GET /api/v1/books` — List all books
- `GET /api/v1/books/:id` — Get a book by id
- `POST /api/v1/books` — Add a new book (JSON: `{ "title", "author", "published_year" }`)
- `PUT /api/v1/books/:id` — Update a book
- `DELETE /api/v1/books/:id` — Delete a book

## DB
- Uses SQLite, file: `bookstore.db` (auto-created)

## Dev
- Auto-migration on startup

## Resources
- [Warp Framework](https://docs.rs/warp/latest/warp/)
- [Crates Registry](https://crates.io/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

