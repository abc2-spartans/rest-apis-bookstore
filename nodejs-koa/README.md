# Node.js Koa Bookstore API

A simple REST API for managing books, using Koa and SQLite.

## Running the API

```sh
cd nodejs-koa
yarn install
yarn start # or node server.js
```

## Endpoints

- `GET /` or `/health` — Health check
- `GET /api/v1/books` — List all books
- `GET /api/v1/books/:id` — Get a book by id
- `POST /api/v1/books` — Add a new book (JSON: `{ title, author, published_year }`)
- `PUT /api/v1/books/:id` — Update a book
- `DELETE /api/v1/books/:id` — Delete a book

## DB
- Uses SQLite, file: `bookstore.db` (auto-created)

## Dev
- Live reload: use [nodemon](https://nodemon.io/) for auto-restart on file changes (optional)

```sh
yarn global add nodemon
nodemon server.js
```
