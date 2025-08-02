# Go Echo Bookstore API

A simple, efficient REST API for managing books, built with Go and the Echo web framework.

## Features

- CRUD operations for books
- SQLite database
- RESTful API endpoints
- JSON request/response
- Error handling
- Health check endpoint

## Prerequisites

- Go 1.21 or higher
- SQLite3

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rest-apis-bookstore/go-echo
   ```

2. Download dependencies:
   ```bash
   go mod tidy
   ```

## Running the Application

```bash
go run main.go
```

The API will be available at `http://localhost:5000`

## API Endpoints

- `GET /` - Health check
- `GET /api/v1/books` - Get all books
- `GET /api/v1/books/:id` - Get a specific book
- `POST /api/v1/books` - Create a new book
- `PUT /api/v1/books/:id` - Update a book
- `DELETE /api/v1/books/:id` - Delete a book

## Example Requests

### Create a new book
```bash
curl -X POST http://localhost:5000/api/v1/books \
  -H "Content-Type: application/json" \
  -d '{"title":"The Go Programming Language","author":"Alan A. A. Donovan & Brian W. Kernighan","published_year":2015}'
```

### Get all books
```bash
curl http://localhost:5000/api/v1/books
```

## Project Structure

- `main.go` - Application entry point and route definitions
- `go.mod` - Go module definition and dependencies

## Live Reload with Air

For a better development experience, you can use [Air](https://github.com/cosmtrek/air) for live reloading.

### Install Air
```bash
go install github.com/cosmtrek/air@latest
```

### Run with Air
```bash
air
```

This will automatically reload your application when you make changes to the code.


## Dependencies

- [Echo](https://echo.labstack.com/) - High performance web framework
- [go-sqlite3](https://github.com/mattn/go-sqlite3) - SQLite3 driver for Go

## License

MIT
