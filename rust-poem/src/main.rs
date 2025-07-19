use poem::{get, handler, listener::TcpListener, post, put, delete, Route, Server, web::Data, web::Path, web::Json, Result};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

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
        "apiURL": "http://localhost:7000/api/v1/books"
    }))
}

#[handler]
async fn get_books(pool: Data<&SqlitePool>) -> Result<Json<Vec<Book>>> {
    let books = sqlx::query_as::<_, Book>("SELECT id, title, author, published_year FROM books")
        .fetch_all(pool.0)
        .await?;
    Ok(Json(books))
}

#[handler]
async fn get_book(pool: Data<&SqlitePool>, Path(id): Path<i64>) -> Result<Json<Book>> {
    let book = sqlx::query_as::<_, Book>("SELECT id, title, author, published_year FROM books WHERE id = ?")
        .bind(id)
        .fetch_one(pool.0)
        .await?;
    Ok(Json(book))
}

#[handler]
async fn create_book(pool: Data<&SqlitePool>, Json(book): Json<Book>) -> Result<Json<Book>> {
    let rec = sqlx::query!("INSERT INTO books (title, author, published_year) VALUES (?, ?, ?)", book.title, book.author, book.published_year)
        .execute(pool.0)
        .await?;
    let id = rec.last_insert_rowid();
    let book = Book { id: Some(id), ..book };
    Ok(Json(book))
}

#[handler]
async fn update_book(pool: Data<&SqlitePool>, Path(id): Path<i64>, Json(book): Json<Book>) -> Result<Json<Book>> {
    let affected = sqlx::query!("UPDATE books SET title = ?, author = ?, published_year = ? WHERE id = ?", book.title, book.author, book.published_year, id)
        .execute(pool.0)
        .await?;
    if affected.rows_affected() == 0 {
        return Err(poem::Error::from_status(poem::http::StatusCode::NOT_FOUND));
    }
    Ok(Json(Book { id: Some(id), ..book }))
}

#[handler]
async fn delete_book(pool: Data<&SqlitePool>, Path(id): Path<i64>) -> Result<()> {
    let affected = sqlx::query!("DELETE FROM books WHERE id = ?", id)
        .execute(pool.0)
        .await?;
    if affected.rows_affected() == 0 {
        return Err(poem::Error::from_status(poem::http::StatusCode::NOT_FOUND));
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:bookstore.db").await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            published_year INTEGER
        )"
    ).execute(&pool).await?;

    let app = Route::new()
        .at("/", get(health))
        .at("/api/v1/books", get(get_books).post(create_book))
        .at("/api/v1/books/:id", get(get_book).put(update_book).delete(delete_book))
        .data(pool);

    println!("Bookstore API listening at http://localhost:7000");
    Server::new(TcpListener::bind("0.0.0.0:7000")).run(app).await?;
    Ok(())
}
