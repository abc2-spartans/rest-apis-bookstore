# Python FastAPI Bookstore API

A simple REST API for managing books, using FastAPI and SQLite.

## Running the API

```sh
uv venv .venv
source .venv/bin/activate
uv pip install fastapi uvicorn aiosqlite
uvicorn main:app --reload --port 4300
```

## Endpoints

- `GET /` or `/health` — Health check
- `GET /api/v1/books` — List all books
- `GET /api/v1/books/{id}` — Get a book by id
- `POST /api/v1/books` — Add a new book (JSON: `{ "title", "author", "published_year" }`)
- `PUT /api/v1/books/{id}` — Update a book
- `DELETE /api/v1/books/{id}` — Delete a book

## DB
- Uses SQLite, file: `bookstore.db` (auto-created)

## Dev
- Live reload: `uvicorn main:app --reload --port 4300`
