from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import aiosqlite
from typing import List, Optional
from datetime import datetime

app = FastAPI()
DB_FILE = "bookstore.db"


class Book(BaseModel):
    id: Optional[int] = None
    title: str
    author: str
    published_year: Optional[int] = None


@app.on_event("startup")
async def startup():
    async with aiosqlite.connect(DB_FILE) as db:
        await db.execute("""
            CREATE TABLE IF NOT EXISTS books (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                author TEXT NOT NULL,
                published_year INTEGER
            )
        """)
        await db.commit()


@app.get("/", tags=["Health"])
@app.get("/health", tags=["Health"])
async def health():
    return {
        "status": "healthy",
        "timestamp": datetime.utcnow().isoformat(),
        "service": "Bookstore API",
        "apiURL": "http://localhost:5000/api/v1/books",
    }


@app.get("/api/v1/books", response_model=List[Book])
async def get_books():
    async with aiosqlite.connect(DB_FILE) as db:
        cursor = await db.execute("SELECT id, title, author, published_year FROM books")
        rows = await cursor.fetchall()
        return [
            Book(id=row[0], title=row[1], author=row[2], published_year=row[3])
            for row in rows
        ]


@app.get("/api/v1/books/{id}", response_model=Book)
async def get_book(id: int):
    async with aiosqlite.connect(DB_FILE) as db:
        cursor = await db.execute(
            "SELECT id, title, author, published_year FROM books WHERE id = ?", (id,)
        )
        row = await cursor.fetchone()
        if not row:
            raise HTTPException(status_code=404, detail="Book not found")
        return Book(id=row[0], title=row[1], author=row[2], published_year=row[3])


@app.post("/api/v1/books", response_model=Book, status_code=201)
async def create_book(book: Book):
    async with aiosqlite.connect(DB_FILE) as db:
        cursor = await db.execute(
            "INSERT INTO books (title, author, published_year) VALUES (?, ?, ?)",
            (book.title, book.author, book.published_year),
        )
        await db.commit()
        book.id = cursor.lastrowid
        return book


@app.put("/api/v1/books/{id}", response_model=Book)
async def update_book(id: int, book: Book):
    async with aiosqlite.connect(DB_FILE) as db:
        cursor = await db.execute(
            "UPDATE books SET title = ?, author = ?, published_year = ? WHERE id = ?",
            (book.title, book.author, book.published_year, id),
        )
        await db.commit()
        if cursor.rowcount == 0:
            raise HTTPException(status_code=404, detail="Book not found")
        book.id = id
        return book


@app.delete("/api/v1/books/{id}", status_code=204)
async def delete_book(id: int):
    async with aiosqlite.connect(DB_FILE) as db:
        cursor = await db.execute("DELETE FROM books WHERE id = ?", (id,))
        await db.commit()
        if cursor.rowcount == 0:
            raise HTTPException(status_code=404, detail="Book not found")
        return None
