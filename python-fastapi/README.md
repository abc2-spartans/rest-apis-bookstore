# Python FastAPI Bookstore API

A simple, async REST API for managing books, built with FastAPI and SQLite.

---

## Features
- CRUD operations for books
- Async database access with aiosqlite
- Auto-creates SQLite DB on startup
- OpenAPI docs at `/docs`

---

## Requirements
- Python 3.12+
- [FastAPI](https://fastapi.tiangolo.com/)
- [Uvicorn](https://www.uvicorn.org/)
- [aiosqlite](https://aiosqlite.omnilib.dev/)
- (Optional) [uv](https://github.com/astral-sh/uv) for fast installs

---

## Setup & Running

```sh
# 1. Create and activate a virtual environment
python -m venv .venv
source .venv/bin/activate

# 2. Install dependencies
pip install fastapi uvicorn aiosqlite

# (Optional: use uv for faster installs)
# uv venv .venv
# source .venv/bin/activate
# uv pip install fastapi uvicorn aiosqlite

# 3. Start the server
# Option 1: Use the provided dev script
./dev.sh
# Option 2: Run directly with uvicorn
uvicorn main:app --reload --port 5000
```

The API will be available at [http://localhost:5000](http://localhost:5000)

---

## Endpoints

- `GET /` or `/health` — Health check
- `GET /api/v1/books` — List all books
- `GET /api/v1/books/{id}` — Get a book by id
- `POST /api/v1/books` — Add a new book (JSON: `{ "title", "author", "published_year" }`)
- `PUT /api/v1/books/{id}` — Update a book
- `DELETE /api/v1/books/{id}` — Delete a book

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
- Live reload: `uvicorn main:app --reload --port 5000`
- API docs: [http://localhost:5000/docs](http://localhost:5000/docs)

---

## Testing
- You can use [HTTPie](https://httpie.io/), [curl](https://curl.se/), or [Postman](https://www.postman.com/) to test endpoints.
- Example: `http GET :5000/api/v1/books` (with HTTPie)

---

## Contributing
Pull requests and issues are welcome! For major changes, please open an issue first to discuss what you would like to change.

---

## Resources

- [FastAPI Documentation](https://fastapi.tiangolo.com/)
- [Uvicorn Documentation](https://www.uvicorn.org/)
- [aiosqlite Documentation](https://aiosqlite.omnilib.dev/)

## License
MIT
