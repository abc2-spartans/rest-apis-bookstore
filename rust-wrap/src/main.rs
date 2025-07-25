use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use rusqlite::{params, Connection, Result};
use wrap::{Request, Response, Server};

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    id: Option<i32>,
    title: String,
    author: String,
    published_year: Option<i32>,
}

fn main() -> Result<()> {
    let db = Connection::open("bookstore.db")?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            published_year INTEGER
        )",
        [],
    )?;
    let db = Mutex::new(db);

    let mut server = Server::new();

    // Health check
    server.at("/").get(|_req: Request| async move {
        let resp = serde_json::json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "service": "Bookstore API",
            "apiURL": "http://localhost:5000/api/v1/books"
        });
        Response::json(&resp)
    });

    // Get all books
    server.at("/api/v1/books").get(|_req: Request| async move {
        let db = Connection::open("bookstore.db").unwrap();
        let mut stmt = db.prepare("SELECT id, title, author, published_year FROM books").unwrap();
        let book_iter = stmt
            .query_map([], |row| {
                Ok(Book {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    author: row.get(2)?,
                    published_year: row.get(3)?,
                })
            })
            .unwrap();
        let books: Vec<Book> = book_iter.filter_map(Result::ok).collect();
        Response::json(&books)
    });

    // Get book by id
    server.at("/api/v1/books/:id").get(|req: Request| async move {
        let id: i32 = req.param("id").unwrap().parse().unwrap();
        let db = Connection::open("bookstore.db").unwrap();
        let mut stmt = db.prepare("SELECT id, title, author, published_year FROM books WHERE id = ?1").unwrap();
        let book = stmt
            .query_row([id], |row| {
                Ok(Book {
                    id: row.get(0).ok(),
                    title: row.get(1).unwrap(),
                    author: row.get(2).unwrap(),
                    published_year: row.get(3).ok(),
                })
            })
            .ok();
        match book {
            Some(b) => Response::json(&b),
            None => Response::not_found(),
        }
    });

    // Create book
    server.at("/api/v1/books").post(|mut req: Request| async move {
        let book: Book = req.body_json().await.unwrap();
        let db = Connection::open("bookstore.db").unwrap();
        db.execute(
            "INSERT INTO books (title, author, published_year) VALUES (?1, ?2, ?3)",
            params![book.title, book.author, book.published_year],
        ).unwrap();
        Response::created()
    });

    // Update book
    server.at("/api/v1/books/:id").put(|mut req: Request| async move {
        let id: i32 = req.param("id").unwrap().parse().unwrap();
        let book: Book = req.body_json().await.unwrap();
        let db = Connection::open("bookstore.db").unwrap();
        let affected = db.execute(
            "UPDATE books SET title = ?1, author = ?2, published_year = ?3 WHERE id = ?4",
            params![book.title, book.author, book.published_year, id],
        ).unwrap();
        if affected == 0 {
            Response::not_found()
        } else {
            Response::json(&book)
        }
    });

    // Delete book
    server.at("/api/v1/books/:id").delete(|req: Request| async move {
        let id: i32 = req.param("id").unwrap().parse().unwrap();
        let db = Connection::open("bookstore.db").unwrap();
        let affected = db.execute("DELETE FROM books WHERE id = ?1", params![id]).unwrap();
        if affected == 0 {
            Response::not_found()
        } else {
            Response::empty()
        }
    });

    println!("Bookstore API listening at http://localhost:8080");
    server.run("0.0.0.0:8080").unwrap();
    Ok(())
}
