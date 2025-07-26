# Rust Poem Bookstore API

A simple REST API for managing books, using Poem and SQLite.

## Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) must be installed.
  - You can install Rust and Cargo with:
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    ```
- SQLite is used as the database. Most systems come with SQLite preinstalled, but if you encounter issues, install the native library:
  - **Ubuntu/Debian:** `sudo apt-get install libsqlite3-dev`
  - **Mac:** `brew install sqlite3`
  
## Running the API

### Standard Run
```sh
cd rust-poem
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
- `GET /api/v1/books/:id` — Get a book by id
- `POST /api/v1/books` — Add a new book (JSON: `{ "title", "author", "published_year" }`)
- `PUT /api/v1/books/:id` — Update a book
- `DELETE /api/v1/books/:id` — Delete a book

## DB
- Uses SQLite, file: `bookstore.db` (auto-created)

## Dev
- Auto-migration on startup
