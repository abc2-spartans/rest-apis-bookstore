use poem::EndpointExt;
use poem::{
    error::ResponseError, get, handler, http::StatusCode, listener::TcpListener, web::Data,
    web::Json, web::Path, Result, Route, Server,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::fmt;

// Custom error type that implements ResponseError
#[derive(Debug)]
struct AppError(sqlx::Error);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database error: {}", self.0)
    }
}

impl ResponseError for AppError {
    fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError(err)
    }
}

impl From<AppError> for poem::Error {
    fn from(err: AppError) -> Self {
        poem::Error::from_string(err.to_string(), err.status())
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
struct Book {
    id: Option<i64>,
    title: String,
    author: String,
    published_year: Option<i32>,
}

#[handler]
async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "Bookstore API",
        "apiURL": "http://localhost:5000/api/v1/books"
    }))
}

#[handler]
async fn get_books(pool: Data<&SqlitePool>) -> Result<Json<Vec<Book>>> {
    let books = sqlx::query_as::<_, Book>("SELECT id, title, author, published_year FROM books")
        .fetch_all(pool.0)
        .await
        .map_err(AppError::from)
        .map_err(poem::Error::from)?;
    Ok(Json(books))
}

#[handler]
async fn get_book(pool: Data<&SqlitePool>, Path(id): Path<i64>) -> Result<Json<Book>> {
    let book = sqlx::query_as::<_, Book>(
        "SELECT id, title, author, published_year FROM books WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool.0)
    .await
    .map_err(AppError::from)
    .map_err(poem::Error::from)?;
    Ok(Json(book))
}

#[handler]
async fn create_book(pool: Data<&SqlitePool>, Json(book): Json<Book>) -> Result<Json<Book>> {
    let rec = sqlx::query("INSERT INTO books (title, author, published_year) VALUES (?, ?, ?)")
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.published_year)
        .execute(pool.0)
        .await
        .map_err(AppError::from)
        .map_err(poem::Error::from)?;
    let id = rec.last_insert_rowid();
    let book = Book {
        id: Some(id),
        ..book
    };
    Ok(Json(book))
}

#[handler]
async fn update_book(
    pool: Data<&SqlitePool>,
    Path(id): Path<i64>,
    Json(book): Json<Book>,
) -> Result<Json<Book>> {
    let affected =
        sqlx::query("UPDATE books SET title = ?, author = ?, published_year = ? WHERE id = ?")
            .bind(&book.title)
            .bind(&book.author)
            .bind(&book.published_year)
            .bind(id)
            .execute(pool.0)
            .await
            .map_err(AppError::from)
            .map_err(poem::Error::from)?;
    if affected.rows_affected() == 0 {
        return Err(poem::Error::from_status(poem::http::StatusCode::NOT_FOUND));
    }
    Ok(Json(Book {
        id: Some(id),
        ..book
    }))
}

#[handler]
async fn delete_book(pool: Data<&SqlitePool>, Path(id): Path<i64>) -> Result<()> {
    let affected = sqlx::query("DELETE FROM books WHERE id = ?")
        .bind(id)
        .execute(pool.0)
        .await
        .map_err(AppError::from)
        .map_err(poem::Error::from)?;

    if affected.rows_affected() == 0 {
        return Err(poem::Error::from_status(poem::http::StatusCode::NOT_FOUND));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Resolve absolute path for the database
    let project_root = std::env::current_dir()?;
    let db_path = project_root.join("bookstore.db");

    // Ensure the directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Log database path for debugging
    println!("Using database at: {}", db_path.display());

    // Connect to SQLite with a busy timeout
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(db_path)
                .create_if_missing(true) // Create database if it doesn't exist
                .busy_timeout(std::time::Duration::from_secs(5)),
        )
        .await?;

    // Create the books table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            published_year INTEGER
        )",
    )
    .execute(&pool)
    .await?;

    // Define routes
    let app = Route::new()
        .at("/api/v1/health", get(health))
        .at("/api/v1/books", get(get_books).post(create_book))
        .at("/api/v1/books/:id", get(get_book).put(update_book).delete(delete_book))
        .data(pool);

    // Start server
    println!("Bookstore API listening at http://localhost:5000");
    Server::new(TcpListener::bind("0.0.0.0:5000"))
        .run(app)
        .await?;
    Ok(())
}
