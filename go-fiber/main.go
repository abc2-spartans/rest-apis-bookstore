package main

import (
	"database/sql"
	"fmt"
	"log"
	"strconv"
	"time"

	"github.com/gofiber/fiber/v2"
	_ "github.com/mattn/go-sqlite3"
)

type Book struct {
	ID            int    `json:"id"`
	Title         string `json:"title"`
	Author        string `json:"author"`
	PublishedYear int    `json:"published_year"`
}

func main() {
	app := fiber.New()
	// SQLite setup
	db, err := sql.Open("sqlite3", "bookstore.db")
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	_, err = db.Exec(`CREATE TABLE IF NOT EXISTS books (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		title TEXT NOT NULL,
		author TEXT NOT NULL,
		published_year INTEGER
	)`)
	if err != nil {
		log.Fatal(err)
	}

	// Health check
	app.Get("/", func(c *fiber.Ctx) error {
		return c.JSON(fiber.Map{
			"status":    "healthy",
			"timestamp": time.Now().UTC(),
			"service":   "Bookstore API",
			"apiURL":    "http://localhost:3000/api/v1/books",
		})
	})

	// Get all books
	app.Get("/api/v1/books", func(c *fiber.Ctx) error {
		rows, err := db.Query("SELECT id, title, author, published_year FROM books")
		if err != nil {
			return c.Status(500).JSON(fiber.Map{"error": err.Error()})
		}
		defer rows.Close()
		books := []Book{}
		for rows.Next() {
			var b Book
			if err := rows.Scan(&b.ID, &b.Title, &b.Author, &b.PublishedYear); err != nil {
				return c.Status(500).JSON(fiber.Map{"error": err.Error()})
			}
			books = append(books, b)
		}
		return c.JSON(books)
	})

	// Get book by ID
	app.Get("/api/v1/books/:id", func(c *fiber.Ctx) error {
		id := c.Params("id")
		var b Book
		err := db.QueryRow("SELECT id, title, author, published_year FROM books WHERE id = ?", id).Scan(&b.ID, &b.Title, &b.Author, &b.PublishedYear)
		if err == sql.ErrNoRows {
			return c.Status(404).JSON(fiber.Map{"error": "Book not found"})
		} else if err != nil {
			return c.Status(500).JSON(fiber.Map{"error": err.Error()})
		}
		return c.JSON(b)
	})

	// Add new book
	app.Post("/api/v1/books", func(c *fiber.Ctx) error {
		var b Book
		if err := c.BodyParser(&b); err != nil {
			return c.Status(400).JSON(fiber.Map{"error": "Invalid request"})
		}
		if b.Title == "" || b.Author == "" {
			return c.Status(400).JSON(fiber.Map{"error": "Title and author required"})
		}
		res, err := db.Exec("INSERT INTO books (title, author, published_year) VALUES (?, ?, ?)", b.Title, b.Author, b.PublishedYear)
		if err != nil {
			return c.Status(500).JSON(fiber.Map{"error": err.Error()})
		}
		id, _ := res.LastInsertId()
		b.ID = int(id)
		return c.Status(201).JSON(b)
	})

	// Update book
	app.Put("/api/v1/books/:id", func(c *fiber.Ctx) error {
		id := c.Params("id")
		var b Book
		if err := c.BodyParser(&b); err != nil {
			return c.Status(400).JSON(fiber.Map{"error": "Invalid request"})
		}
		res, err := db.Exec("UPDATE books SET title = ?, author = ?, published_year = ? WHERE id = ?", b.Title, b.Author, b.PublishedYear, id)
		if err != nil {
			return c.Status(500).JSON(fiber.Map{"error": err.Error()})
		}
		n, _ := res.RowsAffected()
		if n == 0 {
			return c.Status(404).JSON(fiber.Map{"error": "Book not found"})
		}
		idInt, _ := strconv.Atoi(id)
		b.ID = idInt
		return c.JSON(b)
	})

	// Delete book
	app.Delete("/api/v1/books/:id", func(c *fiber.Ctx) error {
		id := c.Params("id")
		res, err := db.Exec("DELETE FROM books WHERE id = ?", id)
		if err != nil {
			return c.Status(500).JSON(fiber.Map{"error": err.Error()})
		}
		n, _ := res.RowsAffected()
		if n == 0 {
			return c.Status(404).JSON(fiber.Map{"error": "Book not found"})
		}
		return c.SendStatus(204)
	})

	fmt.Println("Bookstore API listening at http://localhost:5000")
	log.Fatal(app.Listen(":5000"))
}
