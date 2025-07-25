# Python BlackSheep Bookstore API

A simple REST API for managing books, using BlackSheep and SQLite.

## Quickstart (with uv and pyproject.toml)

```sh
# 1. Create and activate a virtual environment (using uv)
uv venv .venv
source .venv/bin/activate

# 2. Install dependencies from pyproject.toml
uv pip install -r pyproject.toml
# or, for full sync (recommended)
uv pip sync

# 3. Run the server (choose one):
# Option 1: Use the provided script
./dev.sh
# Option 2: Run directly with uvicorn
uvicorn main:app --port 5000
# Option 3: Use uv's runner for uvicorn
uv run -- uvicorn main:app --port 5000
```

> **Note:**
> Do **not** use `uv run main.py` for ASGI apps like BlackSheep. This will run the script as a regular Python file, not as an ASGI app, and can cause errors or unexpected behavior. Always use `uvicorn main:app ...` or `uv run -- uvicorn main:app ...`.

The API will be available at: `http://localhost:5000/`

---

## Alternative: Standard Python venv

```sh
python3.11 -m venv .venv
source .venv/bin/activate
pip install .
./dev.sh
```

---

## Endpoints

- `GET /` — Welcome message
- `GET /health` — Health check
- `GET /api/v1/books` — List all books
- `GET /api/v1/books/{id}` — Get a book by id
- `POST /api/v1/books` — Add a new book (JSON: `{ "title", "author", "published_year" }`)
- `PUT /api/v1/books/{id}` — Update a book
- `DELETE /api/v1/books/{id}` — Delete a book

## DB

- Uses SQLite, file: `bookstore.db` (auto-created)

## Dev

- Live reload: edit `main.py` and restart `./dev.sh`

## Resources

- [BlackSheep Documentation](https://blacksheep.readthedocs.io/en/latest/)
- [SQLite Documentation](https://www.sqlite.org/docs.html)
- [uv Documentation](https://uv.readthedocs.io/en/latest/)
- [uvicorn Documentation](https://www.uvicorn.org/)
- [uv-installation](https://docs.astral.sh/uv/getting-started/installation/#installation-methods)

## Troubleshooting

- If you see errors about duplicate routes, ensure you have only one function per route and restart the server.
- If you change dependencies, re-run `uv pip install -r pyproject.toml` or `uv pip sync` in your virtual environment.
- If you see import/module errors, ensure your virtual environment is activated.
- You can use either `.venv` or `.venv-uv` as your environment folder.
- **Do not use `uv run main.py` for ASGI apps.** Use `uvicorn main:app ...` or `uv run -- uvicorn main:app ...` instead.
