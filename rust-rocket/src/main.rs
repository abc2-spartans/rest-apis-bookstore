#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_sync_db_pools::{database, diesel};
use rocket_sync_db_pools::diesel::prelude::*;
use rocket::State;
use std::sync::Arc;

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "books"]
pub struct Book {
    pub id: Option<i32>,
    pub title: String,
    pub author: String,
    pub published_year: Option<i32>,
}

table! {
    books (id) {
        id -> Nullable<Integer>,
        title -> Text,
        author -> Text,
        published_year -> Nullable<Integer>,
    }
}

#[get("/")]
fn health() -> Json<rocket::serde::json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "Bookstore API",
        "apiURL": "http://localhost:5000/api/v1/books"
    }))
}

#[get("/api/v1/books")]
async fn get_books(conn: DbConn) -> Json<Vec<Book>> {
    use self::books::dsl::*;
    let result = conn.run(|c| books.load::<Book>(c)).await.unwrap_or_default();
    Json(result)
}

#[get("/api/v1/books/<book_id>")]
async fn get_book(conn: DbConn, book_id: i32) -> Option<Json<Book>> {
    use self::books::dsl::*;
    conn.run(move |c| books.filter(id.eq(book_id)).first::<Book>(c).ok().map(Json)).await
}

#[post("/api/v1/books", format = "json", data = "<book>")]
async fn create_book(conn: DbConn, book: Json<Book>) -> Option<Json<Book>> {
    use self::books::dsl::*;
    let new_book = Book { id: None, ..book.into_inner() };
    let inserted: Result<Book, _> = conn.run(move |c| {
        diesel::insert_into(books).values(&new_book).execute(c).ok()?;
        books.order(id.desc()).first(c).ok()
    }).await;
    inserted.ok().map(Json)
}

#[put("/api/v1/books/<book_id>", format = "json", data = "<book>")]
async fn update_book(conn: DbConn, book_id: i32, book: Json<Book>) -> Option<Json<Book>> {
    use self::books::dsl::*;
    let updated_book = Book { id: Some(book_id), ..book.into_inner() };
    let result: Result<Book, _> = conn.run(move |c| {
        diesel::update(books.filter(id.eq(book_id)))
            .set((title.eq(&updated_book.title), author.eq(&updated_book.author), published_year.eq(updated_book.published_year)))
            .execute(c).ok()?;
        books.filter(id.eq(book_id)).first(c).ok()
    }).await;
    result.ok().map(Json)
}

#[delete("/api/v1/books/<book_id>")]
async fn delete_book(conn: DbConn, book_id: i32) -> rocket::http::Status {
    use self::books::dsl::*;
    let affected = conn.run(move |c| diesel::delete(books.filter(id.eq(book_id))).execute(c)).await.unwrap_or(0);
    if affected == 0 {
        rocket::http::Status::NotFound
    } else {
        rocket::http::Status::NoContent
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![health, get_books, get_book, create_book, update_book, delete_book])
}
