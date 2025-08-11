# Multi-Stack Bookstore REST APIs

Welcome to the **Multi-Stack Bookstore REST APIs** repository! This comprehensive project demonstrates how to build consistent RESTful APIs across multiple programming languages and frameworks. Each implementation provides a complete bookstore API with CRUD operations, allowing developers to compare approaches and learn best practices across different technology stacks.

## Purpose

This repository aims to:

- **Demonstrate RESTful API patterns** across multiple languages and frameworks
- **Provide practical examples** for developers learning new technology stacks
- **Enable easy comparison** of implementation approaches between languages
- **Offer migration guides** for developers switching between technologies
- **Showcase modern tooling** and best practices for each ecosystem

## Repository Structure

The repository is organized by language and framework combinations:

```
rest-api-bookstore/
├── /DOCS/                     # Documentation and migration guides
│   ├── NODE_TO_PYTHON.md     # Node.js to Python migration guide
│   ├── NODE_TO_JAVA.md       # Node.js to Java migration guide
│   ├── NODE_TO_RUST.md       # Node.js to Rust migration guide
│   ├── NODE_TO_DOTNET.md     # Node.js to .NET migration guide
│   ├── NODE_TO_GO.md         # Node.js to Go migration guide
│   ├── api-spec.md           # API specification (OpenAPI/Swagger)
│   └── postman-collection.json # Postman collection for testing
├── /nodejs-express/           # Node.js with Express framework
├── /nodejs-koa/              # Node.js with Koa framework
├── /nodejs-nest/             # Node.js with NestJS framework
├── /python-flask/            # Python with Flask framework
├── /python-fastapi/          # Python with FastAPI framework
├── /python-blacksheep/       # Python with BlackSheep framework
├── /javaspringboot/          # Java with Spring Boot framework
├── /dotnet/                  # .NET with ASP.NET Core
├── /go-fiber/                # Go with Fiber framework
├── /go-echo/                 # Go with Echo framework
├── /rust-rocket/             # Rust with Rocket framework
├── /rust-poem/               # Rust with Poem framework
├── /rust-wrap/               # Rust with Warp framework
├── README.md                 # This file
├── LICENSE                   # MIT License
└── .gitignore                # Git ignore file
```

## Available Implementations

The following languages and frameworks are currently supported:

### Node.js Implementations
| Framework | Folder | Description |
|-----------|--------|-------------|
| Express | [/nodejs-express](/nodejs-express) | Minimal and flexible web framework |
| Koa | [/nodejs-koa](/nodejs-koa) | Next-generation web framework |
| NestJS | [/nodejs-nest](/nodejs-nest) | Progressive Node.js framework |

### Python Implementations
| Framework | Folder | Description |
|-----------|--------|-------------|
| Flask | [/python-flask](/python-flask) | Lightweight WSGI web framework |
| FastAPI | [/python-fastapi](/python-fastapi) | Modern, fast web framework |
| BlackSheep | [/python-blacksheep](/python-blacksheep) | Fast ASGI web framework |

### Go Implementations
| Framework | Folder | Description |
|-----------|--------|-------------|
| Fiber | [/go-fiber](/go-fiber) | Express-inspired web framework |
| Echo | [/go-echo](/go-echo) | High performance, minimalist Go web framework |

### Rust Implementations
| Framework | Folder | Description |
|-----------|--------|-------------|
| Rocket | [/rust-rocket](/rust-rocket) | Type-safe web framework |
| Poem | [/rust-poem](/rust-poem) | Full-featured web framework |
| Warp | [/rust-wrap](/rust-wrap) | Composable web framework |

### Java Implementations
| Framework | Folder | Description |
|-----------|--------|-------------|
| Spring Boot | [/javaspringboot](/javaspringboot) | Production-ready Spring framework |

### .NET Implementations
| Framework | Folder | Description |
|-----------|--------|-------------|
| ASP.NET Core | [/dotnet](/dotnet) | Cross-platform .NET framework |

Each implementation contains:
- Complete RESTful API with CRUD operations for `books`
- Dedicated `README.md` with setup and run instructions
- SQLite database integration
- Consistent API endpoints and responses

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

## Migration Guides

Switching between technology stacks? Our comprehensive migration guides help you understand the equivalents and differences:

- **[Node.js to Python](/DOCS/NODE_TO_PYTHON.md)** - npm/Node.js concepts mapped to Python/uv
- **[Node.js to Java](/DOCS/NODE_TO_JAVA.md)** - npm/Node.js concepts mapped to Java/Maven
- **[Node.js to Rust](/DOCS/NODE_TO_RUST.md)** - npm/Node.js concepts mapped to Rust/Cargo
- **[Node.js to .NET](/DOCS/NODE_TO_DOTNET.md)** - npm/Node.js concepts mapped to .NET/dotnet CLI
- **[Node.js to Go](/DOCS/NODE_TO_GO.md)** - npm/Node.js concepts mapped to Go/go mod

Each guide covers:
- Package management equivalents
- Project initialization
- Dependency installation
- Running applications
- Environment variables
- JSON handling
- Server setup examples
- Middleware patterns
- Routing examples

## Getting Started

Follow these steps to get started:

1. **Choose Your Stack**:
   - Browse the [Available Implementations](#available-implementations) section
   - Select a language and framework combination that interests you
   - Navigate to the corresponding folder

2. **Follow Setup Instructions**:
   - Each implementation has a detailed `README.md` with setup steps
   - Install the required dependencies and tools
   - Run the development server

3. **Test the API**:
   - Use the provided cURL examples in each README
   - Import `/DOCS/postman-collection.json` into Postman for comprehensive testing
   - All implementations expose the same API endpoints for consistency

4. **Compare Implementations**:
   - Try multiple frameworks within the same language
   - Use the migration guides to understand differences between languages
   - Compare code structure, performance, and developer experience

5. **Learn from Migration Guides**:
   - If you're familiar with Node.js, use our migration guides to quickly understand other languages
   - Each guide provides practical examples and equivalent commands

## Prerequisites

Before running the code, ensure you have the following installed (specific versions are listed in each language’s README):

- **Node.js** (v22 or later): For the Node.js implementation.
- **Java JDK** (v17 or later): For the Java implementation.
- **Python** (v3.12 or later): For the Python implementation.
- **Docker**: Optional, for running containerized versions.
- **Postman**: For testing APIs (or use cURL).

## Setup Instructions

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/your-username/rest-api-bookstore.git
   cd rest-api-bookstore
   ```

2. **Choose an Implementation**:

   ```bash
   cd nodejs-express     # Node.js with Express
   cd python-flask       # Python with Flask
   cd javaspringboot     # Java with Spring Boot
   cd dotnet             # .NET with ASP.NET Core
   # ... or any other implementation
   ```

3. **Follow Implementation-Specific Instructions**:

   - Each folder contains a detailed `README.md` with setup steps
   - Install required dependencies and tools
   - Run the development server
   
   **Example for Node.js Express**:
   ```bash
   cd nodejs-express
   yarn install
   yarn start
   ```
   
   **Example for Python Flask**:
   ```bash
   cd python-flask
   poetry install
   ./dev.sh
   ```

4. **Test the API**:
   - Use Postman with the provided collection or run cURL commands like:
     ```bash
     curl http://localhost:5000/api/v1/books
     ```
   - Example POST request:
     ```bash
     curl -X POST http://localhost:5000/api/v1/books \
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
  curl http://localhost:5000/api/v1/books
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
  curl -X POST http://localhost:5000/api/v1/books \
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
