# Python Flask Bookstore API

A simple REST API for managing books, using Flask and SQLite.

## Running the API

```sh
poetry install
./dev.sh
```

The API will be available at: `http://localhost:5000/`

## Endpoints

- `GET /` or `/health` — Health check
- `GET /api/v1/books` — List all books
- `GET /api/v1/books/<id>` — Get a book by id
- `POST /api/v1/books` — Add a new book (JSON: `{ "title", "author", "published_year" }`)
- `PUT /api/v1/books/<id>` — Update a book
- `DELETE /api/v1/books/<id>` — Delete a book

## DB
- Uses SQLite, file: `bookstore.db` (auto-created)

## Dev
- Live reload: edit `app.py` and restart `./dev.sh`
