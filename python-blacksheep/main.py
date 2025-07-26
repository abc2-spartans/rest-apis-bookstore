from blacksheep import Application, Response
from blacksheep.server.responses import json
import aiosqlite
from typing import Optional
from datetime import datetime

app = Application()
DB_FILE = "bookstore.db"


class Book:
    def __init__(
        self,
        id: Optional[int],
        title: str,
        author: str,
        published_year: Optional[int],
    ):
        self.id = id
        self.title = title
        self.author = author
        self.published_year = published_year

    def to_dict(self):
        return {
            "id": self.id,
            "title": self.title,
            "author": self.author,
            "published_year": self.published_year,
        }


async def init_db():
    async with aiosqlite.connect(DB_FILE) as db:
        await db.execute(
            """
            CREATE TABLE IF NOT EXISTS books (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                author TEXT NOT NULL,
                published_year INTEGER
            )
        """
        )
        await db.commit()


@app.on_start
async def on_start():
    await init_db()


@app.router.route("/", methods=["GET"])
async def root(request):
    return json({"message": "Server is healthy and running"})

@app.router.route("/api/v1", methods=["GET"])
async def root(request):
    return json({"message": "Welcome to the Bookstore API!"})


@app.router.route("/health", methods=["GET"])
@app.router.route("/api/v1/health", methods=["GET"])
async def health_check(request):
    return json(
        {
            "status": "healthy",
            "timestamp": datetime.utcnow().isoformat(),
            "service": "Bookstore API",
            "apiURL": "http://localhost:5000/api/v1/books",
        }
    )


@app.router.route("/api/v1/books", methods=["GET"])
async def get_books(request):
    async with aiosqlite.connect(DB_FILE) as db:
        cursor = await db.execute("SELECT id, title, author, published_year FROM books")
        rows = await cursor.fetchall()
        books = [
            Book(
                id=row[0], title=row[1], author=row[2], published_year=row[3]
            ).to_dict()
            for row in rows
        ]
        return json(books)


@app.router.route("/api/v1/books/{id}", methods=["GET"])
async def get_book(request, id: int):
    async with aiosqlite.connect(DB_FILE) as db:
        cursor = await db.execute(
            "SELECT id, title, author, published_year FROM books WHERE id = ?",
            (id,),
        )
        row = await cursor.fetchone()
        if not row:
            return Response(
                404,
                content=b'{"error": "Book not found"}',
                content_type=b"application/json",
            )
        book = Book(
            id=row[0],
            title=row[1],
            author=row[2],
            published_year=row[3],
        )
        return json(book.to_dict())


@app.router.route("/api/v1/books", methods=["POST"])
async def create_book(request):
    data = await request.json()
    title = data.get("title")
    author = data.get("author")
    published_year = data.get("published_year")
    if not title or not author:
        return Response(
            400,
            content=b'{"error": "Title and author required"}',
            content_type=b"application/json",
        )
    async with aiosqlite.connect(DB_FILE) as db:
        cursor = await db.execute(
            "INSERT INTO books (title, author, published_year) VALUES (?, ?, ?)",
            (title, author, published_year),
        )
        await db.commit()
        book_id = cursor.lastrowid
        return json(
            {
                "id": book_id,
                "title": title,
                "author": author,
                "published_year": published_year,
            },
            201,
        )


@app.router.route("/api/v1/books/{id}", methods=["PUT"])
async def update_book(request, id: int):
    data = await request.json()
    title = data.get("title")
    author = data.get("author")
    published_year = data.get("published_year")
    async with aiosqlite.connect(DB_FILE) as db:
        cursor = await db.execute(
            "UPDATE books SET title = ?, author = ?, published_year = ? WHERE id = ?",
            (title, author, published_year, id),
        )
        await db.commit()
        if cursor.rowcount == 0:
            return Response(
                404,
                content=b'{"error": "Book not found"}',
                content_type=b"application/json",
            )
        return json(
            {
                "id": id,
                "title": title,
                "author": author,
                "published_year": published_year,
            }
        )


@app.router.route("/api/v1/books/{id}", methods=["DELETE"])
async def delete_book(request, id: int):
    async with aiosqlite.connect(DB_FILE) as db:
        cursor = await db.execute("DELETE FROM books WHERE id = ?", (id,))
        await db.commit()
        if cursor.rowcount == 0:
            return Response(
                404,
                content=b'{"error": "Book not found"}',
                content_type=b"application/json",
            )
        return Response(204)


if __name__ == "__main__":
    import uvicorn

    uvicorn.run("main:app", host="0.0.0.0", port=5000)
