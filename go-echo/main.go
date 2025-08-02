package main

import (
	"database/sql"
	"fmt"
	"log"
	"net/http"
	"strconv"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"github.com/labstack/echo/v4"
)

type Book struct {
	ID            int    `json:"id"`
	Title         string `json:"title"`
	Author        string `json:"author"`
	PublishedYear int    `json:"published_year,omitempty"`
}

func main() {
	// Initialize Echo instance
	e := echo.New()

	// Database setup
	db, err := sql.Open("sqlite3", "bookstore.db")
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	// Create books table if not exists
	_, err = db.Exec(`CREATE TABLE IF NOT EXISTS books (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		title TEXT NOT NULL,
		author TEXT NOT NULL,
		published_year INTEGER
	)`)
	if err != nil {
		log.Fatal(err)
	}

	// Routes
	e.GET("/", healthCheck)
	e.GET("/api/v1/books", getBooks(db))
	e.GET("/api/v1/books/:id", getBook(db))
	e.POST("/api/v1/books", createBook(db))
	e.PUT("/api/v1/books/:id", updateBook(db))
	e.DELETE("/api/v1/books/:id", deleteBook(db))

	// Start server
	fmt.Println("Bookstore API listening at http://localhost:5000")
	e.Logger.Fatal(e.Start(":5000"))
}

// Health check endpoint
func healthCheck(c echo.Context) error {
	return c.JSON(http.StatusOK, map[string]interface{}{
		"status":    "healthy",
		"timestamp": time.Now().UTC(),
		"service":   "Bookstore API (Echo)",
		"apiURL":    "http://localhost:5000/api/v1/books",
	})
}

// Get all books
func getBooks(db *sql.DB) echo.HandlerFunc {
	return func(c echo.Context) error {
		rows, err := db.Query("SELECT id, title, author, published_year FROM books")
		if err != nil {
			return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
		}
		defer rows.Close()

		var books []Book
		for rows.Next() {
			var b Book
			if err := rows.Scan(&b.ID, &b.Title, &b.Author, &b.PublishedYear); err != nil {
				return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
			}
			books = append(books, b)
		}
		return c.JSON(http.StatusOK, books)
	}
}

// Get book by ID
func getBook(db *sql.DB) echo.HandlerFunc {
	return func(c echo.Context) error {
		id := c.Param("id")
		var b Book
		err := db.QueryRow("SELECT id, title, author, published_year FROM books WHERE id = ?", id).
			Scan(&b.ID, &b.Title, &b.Author, &b.PublishedYear)

		if err == sql.ErrNoRows {
			return c.JSON(http.StatusNotFound, map[string]string{"error": "Book not found"})
		} else if err != nil {
			return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
		}
		return c.JSON(http.StatusOK, b)
	}
}

// Create a new book
func createBook(db *sql.DB) echo.HandlerFunc {
	return func(c echo.Context) error {
		var b Book
		if err := c.Bind(&b); err != nil {
			return c.JSON(http.StatusBadRequest, map[string]string{"error": "Invalid request"})
		}

		if b.Title == "" || b.Author == "" {
			return c.JSON(http.StatusBadRequest, map[string]string{
				"error": "Title and author are required",
			})
		}

		res, err := db.Exec(
			"INSERT INTO books (title, author, published_year) VALUES (?, ?, ?)",
			b.Title,
			b.Author,
			b.PublishedYear,
		)
		if err != nil {
			return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
		}

		id, _ := res.LastInsertId()
		b.ID = int(id)
		return c.JSON(http.StatusCreated, b)
	}
}

// Update a book
func updateBook(db *sql.DB) echo.HandlerFunc {
	return func(c echo.Context) error {
		id := c.Param("id")
		var b Book
		if err := c.Bind(&b); err != nil {
			return c.JSON(http.StatusBadRequest, map[string]string{"error": "Invalid request"})
		}

		res, err := db.Exec(
			"UPDATE books SET title = ?, author = ?, published_year = ? WHERE id = ?",
			b.Title,
			b.Author,
			b.PublishedYear,
			id,
		)
		if err != nil {
			return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
		}

		rowsAffected, _ := res.RowsAffected()
		if rowsAffected == 0 {
			return c.JSON(http.StatusNotFound, map[string]string{"error": "Book not found"})
		}

		// Convert id to int for the response
		idInt, _ := strconv.Atoi(id)
		b.ID = idInt
		return c.JSON(http.StatusOK, b)
	}
}

// Delete a book
func deleteBook(db *sql.DB) echo.HandlerFunc {
	return func(c echo.Context) error {
		id := c.Param("id")
		res, err := db.Exec("DELETE FROM books WHERE id = ?", id)
		if err != nil {
			return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
		}

		rowsAffected, _ := res.RowsAffected()
		if rowsAffected == 0 {
			return c.JSON(http.StatusNotFound, map[string]string{"error": "Book not found"})
		}

		return c.NoContent(http.StatusNoContent)
	}
}
