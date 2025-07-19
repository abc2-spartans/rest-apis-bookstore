const express = require("express");
const sqlite3 = require("sqlite3").verbose();
const path = require("path");

const app = express();
const PORT = process.env.PORT || 4000;

// Middleware
app.use(express.json());

// SQLite DB setup
const dbPath = path.join(__dirname, "bookstore.db");
const db = new sqlite3.Database(dbPath, (err) => {
  if (err) {
    console.error("Failed to connect to database:", err.message);
  } else {
    console.log("Connected to the SQLite database.");
  }
});

db.serialize(() => {
  db.run(`CREATE TABLE IF NOT EXISTS books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    published_year INTEGER
  )`);
});

// Health check
app.get(["/", "/health"], (req, res) => {
  res.json({
    status: "healthy",
    timestamp: new Date().toISOString(),
    service: "Bookstore API",
    apiURL: `http://localhost:${PORT}/api/v1/books`,
  });
});

// CRUD endpoints for books
app.get("/api/v1/books", (req, res) => {
  db.all("SELECT * FROM books", [], (err, rows) => {
    if (err) return res.status(500).json({ error: err.message });
    res.json(rows);
  });
});

app.get("/api/v1/books/:id", (req, res) => {
  db.get("SELECT * FROM books WHERE id = ?", [req.params.id], (err, row) => {
    if (err) return res.status(500).json({ error: err.message });
    if (!row) return res.status(404).json({ error: "Book not found" });
    res.json(row);
  });
});

app.post("/api/v1/books", (req, res) => {
  const { title, author, published_year } = req.body;
  if (!title || !author)
    return res.status(400).json({ error: "Title and author required" });
  db.run(
    "INSERT INTO books (title, author, published_year) VALUES (?, ?, ?)",
    [title, author, published_year],
    function (err) {
      if (err) return res.status(500).json({ error: err.message });
      res.status(201).json({ id: this.lastID, title, author, published_year });
    }
  );
});

app.put("/api/v1/books/:id", (req, res) => {
  const { title, author, published_year } = req.body;
  db.run(
    "UPDATE books SET title = ?, author = ?, published_year = ? WHERE id = ?",
    [title, author, published_year, req.params.id],
    function (err) {
      if (err) return res.status(500).json({ error: err.message });
      if (this.changes === 0)
        return res.status(404).json({ error: "Book not found" });
      res.json({ id: req.params.id, title, author, published_year });
    }
  );
});

app.delete("/api/v1/books/:id", (req, res) => {
  db.run("DELETE FROM books WHERE id = ?", [req.params.id], function (err) {
    if (err) return res.status(500).json({ error: err.message });
    if (this.changes === 0)
      return res.status(404).json({ error: "Book not found" });
    res.status(204).send();
  });
});

app.listen(PORT, () => {
  console.log(`Bookstore API listening at http://localhost:${PORT}`);
});
