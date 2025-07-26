# Java Spring Boot Bookstore API

A RESTful API for managing books, built with Spring Boot and H2 in-memory database using Spring Data JPA.

## Running the API

### Prerequisites
- Java 11 or higher
- Maven 3.6+ (if using Maven) or Gradle 6+ (if using Gradle)

### Standard Run
```bash
cd javaspringboot

# If using Maven
# First, compile and package (optional, but recommended)
mvn clean compile
# Then run the application
mvn spring-boot:run

# If using Gradle
# First, build (optional, but recommended)
./gradlew build
# Then run the application
./gradlew bootRun

# Or build and run the JAR directly
mvn clean package
java -jar target/bookstore-app-*.jar
```

### Development Mode
For automatic reloading during development, you can use Spring Boot DevTools (if included in dependencies):
```bash
mvn spring-boot:run -Dspring-boot.run.profiles=dev
```

The server will start on `http://localhost:8080`

## Endpoints

### Health Check
- `GET /` — Root health check
- `GET /health` — Health endpoint
- `GET /api/v1` — API version health check

### Book Management
- `GET /api/books` — Get all books
- `GET /api/books/{id}` — Get a book by ID
- `POST /api/books` — Create a new book
- `PUT /api/books/{id}` — Update a book by ID
- `DELETE /api/books/{id}` — Delete a book by ID

### Request/Response Format
**Book JSON Structure:**
```json
{
  "id": 1,
  "title": "The Great Gatsby",
  "author": "F. Scott Fitzgerald",
  "publishedYear": 1925
}
```

## Database & ORM

- **Database**: H2 in-memory database (for development/testing)
- **ORM**: Spring Data JPA with Hibernate
- **Database Console**: Available at `http://localhost:8080/h2-console`
  - JDBC URL: `jdbc:h2:mem:testdb`
  - Username: `sa`
  - Password: `password`
- **Auto-DDL**: Enabled (`spring.jpa.hibernate.ddl-auto=update`)

## Project Structure

```
src/
├── main/
│   ├── java/com/example/bookstore/
│   │   ├── BookstoreApp.java              # Main application class
│   │   ├── controller/
│   │   │   └── BookController.java        # REST controller
│   │   ├── entity/
│   │   │   └── Book.java                  # JPA entity
│   │   ├── repository/
│   │   │   └── BookRepository.java        # Data repository
│   │   ├── service/
│   │   │   └── BookService.java           # Business logic
│   │   └── exception/
│   │       ├── BookNotFoundException.java
│   │       └── GlobalExceptionHandler.java
│   └── resources/
│       └── application.properties         # Configuration
└── test/
    └── java/com/example/bookstore/
        ├── BookControllerTest.java        # Controller tests
        └── BookServiceTest.java           # Service tests
```

## Configuration

The application can be configured via `src/main/resources/application.properties`:

```properties
# Server Configuration
server.port=8080

# Database Configuration
spring.datasource.url=jdbc:h2:mem:testdb
spring.datasource.username=sa
spring.datasource.password=password

# JPA Configuration
spring.jpa.hibernate.ddl-auto=update
spring.jpa.show-sql=true
```

## Testing

Run tests using:
```bash
# Maven
mvn test

# Gradle
./gradlew test
```

## Building

Build the application:
```bash
# Maven
mvn clean package

# Gradle
./gradlew build
```

## Resources

- [Spring Boot Documentation](https://docs.spring.io/spring-boot/docs/current/reference/htmlsingle/)
- [Spring Data JPA Reference](https://docs.spring.io/spring-data/jpa/docs/current/reference/html/)
- [H2 Database Documentation](https://www.h2database.com/html/main.html)
- [Spring Boot Testing Guide](https://spring.io/guides/gs/testing-web/)
- [Building REST Services with Spring](https://spring.io/guides/gs/rest-service/)
