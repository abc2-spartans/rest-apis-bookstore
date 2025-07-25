# Node.js Express Bookstore API

A simple REST API for managing books, using Express and SQLite.

## Running the API

```sh
cd nodejs
yarn install
yarn start
```

## Endpoints

- `GET /` or `/health` — Health check
- `GET /api/v1/books` — List all books
- `GET /api/v1/books/:id` — Get a book by id
- `POST /api/v1/books` — Add a new book (JSON: `{ title, author, published_year }`)
- `PUT /api/v1/books/:id` — Update a book
- `DELETE /api/v1/books/:id` — Delete a book

## Example Requests(You can also create Postman collection for this API)

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


## DB
- Uses SQLite, file: `bookstore.db` (auto-created)

## Dev
- Live reload: use [nodemon](https://nodemon.io/) for auto-restart on file changes (optional)

```sh
yarn global add nodemon
nodemon server.js
```
