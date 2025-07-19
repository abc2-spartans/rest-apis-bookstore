# Rust Poem Bookstore API

A simple REST API for managing books, using Poem and SQLite.

## Running the API

```sh
cd rust-poem
cargo run
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
