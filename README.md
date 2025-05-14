# RESTful Bookstore API Tutorials

Welcome to the **RESTful Bookstore API Tutorials** repository! This project is designed for students and developers learning to build RESTful APIs in multiple programming languages. Each language implementation provides a consistent, beginner-friendly example of a RESTful API with CRUD (Create, Read, Update, Delete) operations for a bookstore, specifically managing a `books` resource.

## Purpose

This repository aims to:

- Teach the fundamentals of RESTful API design and implementation.
- Provide hands-on examples in popular programming languages: Node.js, Python, Go, Rust, Java and dotNet.
- Allow students to compare how REST APIs are built across different languages and frameworks for a bookstore application.
- Offer shared resources, such as API specifications and testing tools, to enhance learning.

## Repository Structure

The repository is organized as follows:

```
rest-api-bookstore/
├── /docs                   # Shared resources and documentation
│   ├── rest-principles.md # Guide on RESTful API concepts
│   ├── api-spec.md        # API specification for bookstore (OpenAPI/Swagger)
│   └── postman-collection.json # Postman collection for testing bookstore APIs
├── /nodejs                # Node.js implementation (Express)
│   ├── /src               # Source code for bookstore API
│   ├── package.json       # Dependencies
│   ├── README.md          # Node.js-specific instructions
│   └── Dockerfile         # Optional Docker setup
├── /java                  # Java implementation (Spring Boot)
│   ├── /src               # Source code for bookstore API
│   ├── pom.xml            # Maven dependencies
│   ├── README.md          # Java-specific instructions
│   └── Dockerfile         # Optional Docker setup
├── /python                # Python implementation (FastAPI)
│   ├── /src               # Source code for bookstore API
│   ├── requirements.txt   # Dependencies
│   ├── README.md          # Python-specific instructions
│   └── Dockerfile         # Optional Docker setup
├── README.md              # This file
├── LICENSE                # MIT License
└── .gitignore             # Git ignore file
```

## Available Languages

The following languages and frameworks are currently supported:

| Language | Framework   | Folder             | Setup Instructions          |
| -------- | ----------- | ------------------ | --------------------------- |
| Node.js  | Express     | [/nodejs](/nodejs) | [README](/nodejs/README.md) |
| Python   | FastAPI     | [/python](/python) | [README](/python/README.md) |
| Go       | Spring Boot | [/go](/go)         | [README](/go/README.md)     |
| RUST     | Spring Boot | [/rust](/rust)     | [README](/rust/README.md)   |
| Java     | Spring Boot | [/java](/java)     | [README](/java/README.md)   |
| .net     | Spring Boot | [/dotnet](/dotnet) | [README](/dotnet/README.md) |

Each language folder contains:

- A complete RESTful API for a bookstore with CRUD operations for `books`.
- A dedicated `README.md` with setup and run instructions.
- A `Dockerfile` (optional) for containerized deployment.

## Bookstore API Overview

All implementations provide a RESTful API for managing a bookstore’s `books` resource. The API supports the following CRUD operations:

| Method   | Endpoint     | Description                   |
| -------- | ------------ | ----------------------------- |
| `GET`    | `/books`     | Retrieve a list of all books. |
| `POST`   | `/books`     | Create a new book.            |
| `GET`    | `/books/:id` | Retrieve a book by its ID.    |
| `PUT`    | `/books/:id` | Update an existing book.      |
| `DELETE` | `/books/:id` | Delete a book by its ID.      |

**Book Data Structure** (JSON example):

```json
{
  "id": 1,
  "title": "The Great Gatsby",
  "author": "F. Scott Fitzgerald",
  "isbn": "978-0743273565",
  "price": 9.99,
  "stock": 50
}
```

- The API uses JSON for request and response bodies.
- Standard HTTP status codes are used (e.g., `200 OK`, `201 Created`, `404 Not Found`).
- Each implementation uses in-memory storage or a lightweight database (e.g., SQLite, JSON file) to keep it beginner-friendly.

## Getting Started

Follow these steps to get started:

1. **Learn REST Basics**:

   - Read [/docs/rest-principles.md](/docs/rest-principles.md) to understand RESTful API concepts, including HTTP methods, status codes, and best practices.
   - Review [/docs/api-spec.md](/docs/api-spec.md) for the bookstore API specification, which defines the endpoints and data structure.

2. **Choose a Language**:

   - Navigate to the folder of your preferred language (`/nodejs`, `/go`, or `/python` etc).
   - Follow the setup instructions in the respective `README.md`.

3. **Test the APIs**:

   - Import [/docs/postman-collection.json](/docs/postman-collection.json) into Postman to test the bookstore API endpoints.
   - Alternatively, use cURL commands provided in each language’s README.

4. **Explore and Compare**:
   - Each implementation follows the same bookstore API spec for consistency.
   - Compare how Node.js (Express), Java (Spring Boot), and Python (FastAPI) handle routing, data validation, and CRUD operations.

## Prerequisites

Before running the code, ensure you have the following installed (specific versions are listed in each language’s README):

- **Node.js** (v16 or later): For the Node.js implementation.
- **Java JDK** (v17 or later): For the Java implementation.
- **Python** (v3.8 or later): For the Python implementation.
- **Docker**: Optional, for running containerized versions.
- **Postman**: For testing APIs (or use cURL).

## Setup Instructions

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/your-username/rest-api-bookstore.git
   cd rest-api-bookstore
   ```

2. **Navigate to a Language Folder**:

   ```bash
   cd nodejs  # or java, or python
   ```

3. **Follow Language-Specific Instructions**:

   - Refer to the `README.md` in the chosen language folder for detailed setup steps (e.g., installing dependencies, running the server).
   - Example for Node.js:
     ```bash
     cd nodejs
     npm install
     npm start
     ```

4. **Test the API**:
   - Use Postman with the provided collection or run cURL commands like:
     ```bash
     curl http://localhost:3000/books
     ```
   - Example POST request:
     ```bash
     curl -X POST http://localhost:3000/books \
     -H "Content-Type: application/json" \
     -d '{"title":"1984","author":"George Orwell","isbn":"978-0451524935","price":8.99,"stock":100}'
     ```

## Shared Resources

- **REST Principles**: [/docs/rest-principles.md](/docs/rest-principles.md) explains key REST concepts like statelessness, HTTP methods, and best practices for API design.
- **API Specification**: [/docs/api-spec.md](/docs/api-spec.md) provides the OpenAPI/Swagger definition for the bookstore API, detailing endpoints and data structures.
- **Postman Collection**: [/docs/postman-collection.json](/docs/postman-collection.json) includes pre-configured requests for testing all bookstore endpoints.

## Example Usage

Here’s how you might interact with the bookstore API (Node.js example, port 3000):

- **Get all books**:

  ```bash
  curl http://localhost:3000/books
  ```

  Response:

  ```json
  [
    {
      "id": 1,
      "title": "The Great Gatsby",
      "author": "F. Scott Fitzgerald",
      "isbn": "978-0743273565",
      "price": 9.99,
      "stock": 50
    }
  ]
  ```

- **Create a new book**:
  ```bash
  curl -X POST http://localhost:3000/books \
  -H "Content-Type: application/json" \
  -d '{"title":"1984","author":"George Orwell","isbn":"978-0451524935","price":8.99,"stock":100}'
  ```
  Response:
  ```json
  {
    "id": 2,
    "title": "1984",
    "author": "George Orwell",
    "isbn": "978-0451524935",
    "price": 8.99,
    "stock": 100
  }
  ```

## Contributing

We welcome contributions! To contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Make your changes (e.g., add a new language, improve the bookstore API, or update docs).
4. Submit a pull request with a clear description of your changes.

Please follow the [Contributing Guidelines](CONTRIBUTING.md) (to be added) and ensure code consistency with existing implementations.

## Issues and Feedback

- Found a bug? Create an issue on the [Issues](https://github.com/your-username/rest-api-bookstore/issues) page.
- Have questions or suggestions? Join the [Discussions](https://github.com/your-username/rest-api-bookstore/discussions) for community engagement.

## License

This project is licensed under the [MIT License](LICENSE), making it freely available for educational use.

## Contact

For questions or support, reach out via GitHub Issues or Discussions. Happy learning, and enjoy building your bookstore API!
