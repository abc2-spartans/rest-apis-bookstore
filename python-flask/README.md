# Python Flask Bookstore API

A simple, async REST API for managing books, built with Flask and SQLite.

---

## Features
- CRUD operations for books
- Uses SQLite (auto-created)
- Simple, self-contained project

---

## Requirements
- Python 3.11+
- [Flask](https://flask.palletsprojects.com/)
- [Poetry](https://python-poetry.org/) (for dependency management)
- (Optional) [uv](https://github.com/astral-sh/uv) for fast installs

---

## Setup & Running

### With Poetry
```sh
poetry install
./dev.sh
```

### With venv and pip (alternative)
```sh
python3.11 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
./dev.sh
```

The API will be available at: `http://localhost:5000/`

---

## Endpoints

- `GET /` or `/health` — Health check
- `GET /api/v1/books` — List all books
- `GET /api/v1/books/<id>` — Get a book by id
- `POST /api/v1/books` — Add a new book (JSON: `{ "title", "author", "published_year" }`)
- `PUT /api/v1/books/<id>` — Update a book
- `DELETE /api/v1/books/<id>` — Delete a book

---

## Example Requests

### Add a Book
```sh
curl -X POST http://localhost:5000/api/v1/books \
  -H 'Content-Type: application/json' \
  -d '{"title": "1984", "author": "George Orwell", "published_year": 1949}'
```

### List Books
```sh
curl http://localhost:5000/api/v1/books
```

### Get Book by ID
```sh
curl http://localhost:5000/api/v1/books/1
```

### Update Book
```sh
curl -X PUT http://localhost:5000/api/v1/books/1 \
  -H 'Content-Type: application/json' \
  -d '{"title": "Animal Farm", "author": "George Orwell", "published_year": 1945}'
```

### Delete Book
```sh
curl -X DELETE http://localhost:5000/api/v1/books/1
```

---

## Database
- Uses SQLite, file: `bookstore.db` (auto-created in project root)
- No setup required; table is created on first run

---

## Development
- Live reload: edit `app.py` and restart `./dev.sh`

---

## Troubleshooting
- If you see import/module errors, ensure your virtual environment is activated.
- If you change dependencies, re-run `poetry install` or `pip install -r requirements.txt`.
- If you see errors about missing DB, ensure the app has permission to write files in the project directory.

---

## Resources
- [Flask Documentation](https://flask.palletsprojects.com/)
- [Poetry Documentation](https://python-poetry.org/docs/)
- [SQLite Documentation](https://www.sqlite.org/docs.html)

