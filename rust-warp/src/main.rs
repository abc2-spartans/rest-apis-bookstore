use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;
use rusqlite::{params, Connection, Result};
use warp::Filter;
use std::convert::Infallible;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Book {
    id: Option<i32>,
    title: String,
    author: String,
    published_year: Option<i32>,
}

type Db = Arc<Mutex<Connection>>;

#[tokio::main]
async fn main() -> Result<()> {
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
    let db: Db = Arc::new(Mutex::new(db));

    // Shared health response function
    let health_response = || {
        let resp = serde_json::json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "service": "Bookstore API",
            "apiURL": "http://localhost:5000/api/v1/books"
        });
        warp::reply::json(&resp)
    };

    // Health check endpoints
    let health_root = warp::path::end()
        .and(warp::get())
        .map(health_response);

    let health_check = warp::path("health")
        .and(warp::path::end())
        .and(warp::get())
        .map(health_response);

    let health_api = warp::path!("api" / "v1")
        .and(warp::get())
        .map(health_response);

    // Get all books
    let db_filter = warp::any().map(move || db.clone());
    
    let get_books = warp::path!("api" / "v1" / "books")
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(get_all_books);

    // Get book by ID
    let get_book = warp::path!("api" / "v1" / "books" / i32)
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(get_book_by_id);

    // Create book
    let create_book = warp::path!("api" / "v1" / "books")
        .and(warp::post())
        .and(warp::body::json())
        .and(db_filter.clone())
        .and_then(create_new_book);

    // Update book
    let update_book = warp::path!("api" / "v1" / "books" / i32)
        .and(warp::put())
        .and(warp::body::json())
        .and(db_filter.clone())
        .and_then(update_existing_book);

    // Delete book
    let delete_book = warp::path!("api" / "v1" / "books" / i32)
        .and(warp::delete())
        .and(db_filter.clone())
        .and_then(delete_existing_book);

    let routes = health_root
        .or(health_check)
        .or(health_api)
        .or(get_books)
        .or(get_book)
        .or(create_book)
        .or(update_book)
        .or(delete_book);

    println!("Bookstore API listening at http://localhost:5000");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 5000))
        .await;
    
    Ok(())
}

async fn get_all_books(db: Db) -> Result<impl warp::Reply, Infallible> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, title, author, published_year FROM books").unwrap();
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
    Ok(warp::reply::json(&books))
}

async fn get_book_by_id(id: i32, db: Db) -> Result<impl warp::Reply, Infallible> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, title, author, published_year FROM books WHERE id = ?1").unwrap();
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
        Some(b) => Ok(warp::reply::with_status(warp::reply::json(&b), warp::http::StatusCode::OK)),
        None => Ok(warp::reply::with_status(warp::reply::json(&serde_json::json!({"error": "Book not found"})), warp::http::StatusCode::NOT_FOUND)),
    }
}

async fn create_new_book(book: Book, db: Db) -> Result<impl warp::Reply, Infallible> {
    let conn = db.lock().unwrap();
    let result = conn.execute(
        "INSERT INTO books (title, author, published_year) VALUES (?1, ?2, ?3)",
        params![book.title, book.author, book.published_year],
    );
    
    match result {
        Ok(_) => Ok(warp::reply::with_status(warp::reply::json(&book), warp::http::StatusCode::CREATED)),
        Err(_) => Ok(warp::reply::with_status(warp::reply::json(&serde_json::json!({"error": "Failed to create book"})), warp::http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn update_existing_book(id: i32, book: Book, db: Db) -> Result<impl warp::Reply, Infallible> {
    let conn = db.lock().unwrap();
    let affected = conn.execute(
        "UPDATE books SET title = ?1, author = ?2, published_year = ?3 WHERE id = ?4",
        params![book.title, book.author, book.published_year, id],
    ).unwrap_or(0);
    
    if affected == 0 {
        Ok(warp::reply::with_status(warp::reply::json(&serde_json::json!({"error": "Book not found"})), warp::http::StatusCode::NOT_FOUND))
    } else {
        let updated_book = Book { id: Some(id), ..book };
        Ok(warp::reply::with_status(warp::reply::json(&updated_book), warp::http::StatusCode::OK))
    }
}

async fn delete_existing_book(id: i32, db: Db) -> Result<impl warp::Reply, Infallible> {
    let conn = db.lock().unwrap();
    let affected = conn.execute("DELETE FROM books WHERE id = ?1", params![id]).unwrap_or(0);
    
    if affected == 0 {
        Ok(warp::reply::with_status(warp::reply::json(&serde_json::json!({"error": "Book not found"})), warp::http::StatusCode::NOT_FOUND))
    } else {
        Ok(warp::reply::with_status(warp::reply::json(&serde_json::json!({"message": "Book deleted successfully"})), warp::http::StatusCode::OK))
    }
}