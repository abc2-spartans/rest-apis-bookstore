# Bookstore REST API (.NET)

This is a sample RESTful Bookstore API implemented in **.NET (C#)**. It demonstrates best practices for building REST APIs, including CRUD operations, database integration, and clean architecture.

## Features
- CRUD operations for books
- Entity Framework Core with SQLite
- RESTful routing and controllers
- Service and repository layers
- API documentation via OpenAPI/Swagger
- Database migrations

## Project Structure
- `Controllers/` – API endpoint controllers
- `Models/` – Data models/entities
- `Data/` – Database context and configuration
- `Services/` – Business logic and services
- `Migrations/` – Database migrations
- `Program.cs` – Application entry point
- `appsettings.json` – Configuration

## Getting Started

### Prerequisites
- [.NET 6 SDK or newer](https://dotnet.microsoft.com/download)

### Running the API
```bash
# Navigate to the dotnet directory
cd dotnet

# Restore dependencies
 dotnet restore

# Apply migrations (optional if DB already exists)
dotnet ef database update

# Run the API
dotnet run
```

The API will be available at `https://localhost:5001` or `http://localhost:5000` by default.

### Testing Endpoints
- Use [Postman](https://www.postman.com/) or `curl` to test the API.
- Import the Postman collection from `../DOCS/postman-collection.json` for ready-made requests.

## Configuration
- Edit `appsettings.json` for database and server configuration.

## Database
- Uses SQLite for development (see `bookstore.db`).
- Migrations are managed via Entity Framework Core.

## License
See the root `LICENSE` file for license information.

---

For more information, see the main [README](../README.md) and [API Spec](../DOCS/api-spec.md).
