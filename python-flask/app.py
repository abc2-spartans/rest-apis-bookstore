from flask import Flask, request, jsonify, abort
from flask_sqlalchemy import SQLAlchemy
from flask_cors import CORS
from datetime import datetime

app = Flask(__name__)
CORS(app)
app.config["SQLALCHEMY_DATABASE_URI"] = "sqlite:///bookstore.db"
app.config["SQLALCHEMY_TRACK_MODIFICATIONS"] = False
db = SQLAlchemy(app)


class Book(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    title = db.Column(db.String(120), nullable=False)
    author = db.Column(db.String(120), nullable=False)
    published_year = db.Column(db.Integer, nullable=True)


def book_to_dict(book):
    return {
        "id": book.id,
        "title": book.title,
        "author": book.author,
        "published_year": book.published_year,
    }


with app.app_context():
    db.create_all()


@app.route("/", methods=["GET"])
@app.route("/health", methods=["GET"])
def health():
    return jsonify(
        {
            "status": "healthy",
            "timestamp": datetime.now(datetime.timezone.utc).isoformat(),
            "service": "Bookstore API",
            "apiURL": "http://localhost:5000/api/v1/books",
        }
    )


@app.route("/api/v1/books", methods=["GET"])
def get_books():
    books = Book.query.all()
    return jsonify([book_to_dict(book) for book in books])


@app.route("/api/v1/books/<int:book_id>", methods=["GET"])
def get_book(book_id):
    book = Book.query.get(book_id)
    if not book:
        abort(404, description="Book not found")
    return jsonify(book_to_dict(book))


@app.route("/api/v1/books", methods=["POST"])
def create_book():
    data = request.get_json()
    if not data or not data.get("title") or not data.get("author"):
        abort(400, description="Title and author required")
    book = Book(
        title=data["title"],
        author=data["author"],
        published_year=data.get("published_year"),
    )
    db.session.add(book)
    db.session.commit()
    return jsonify(book_to_dict(book)), 201


@app.route("/api/v1/books/<int:book_id>", methods=["PUT"])
def update_book(book_id):
    book = Book.query.get(book_id)
    if not book:
        abort(404, description="Book not found")
    data = request.get_json()
    book.title = data.get("title", book.title)
    book.author = data.get("author", book.author)
    book.published_year = data.get("published_year", book.published_year)
    db.session.commit()
    return jsonify(book_to_dict(book))


@app.route("/api/v1/books/<int:book_id>", methods=["DELETE"])
def delete_book(book_id):
    book = Book.query.get(book_id)
    if not book:
        abort(404, description="Book not found")
    db.session.delete(book)
    db.session.commit()
    return "", 204


if __name__ == "__main__":
    app.run(debug=True, port=5000)
