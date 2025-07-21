# Node.js/npm to Java (with Maven) Equivalents

This guide maps common Node.js/npm commands and concepts to their Java equivalents, with a special focus on Maven as the build tool and dependency manager.

---

## Equivalent of `npm init` in Java (with Maven)

```sh
mvn archetype:generate -DgroupId=com.example -DartifactId=my-app -DarchetypeArtifactId=maven-archetype-quickstart -DinteractiveMode=false
```

- Creates a new Maven project with standard directory structure.
- Shorter version for Spring Boot:

```sh
mvn archetype:generate -DgroupId=com.example -DartifactId=my-app -DarchetypeArtifactId=maven-archetype-webapp
```

- Or use Spring Initializr: https://start.spring.io/

---

## Equivalent of `npm run start` in Java

```sh
mvn spring-boot:run
```

- Runs a Spring Boot application directly.
- For regular Java applications:

```sh
mvn compile exec:java -Dexec.mainClass="com.example.Main"
```

- Or compile and run:

```sh
mvn package
java -jar target/my-app-1.0.jar
```

---

## Equivalent of `npm install <package_name>` in Java (with Maven)

- Add dependency to `pom.xml`:

```xml
<dependency>
    <groupId>com.fasterxml.jackson.core</groupId>
    <artifactId>jackson-core</artifactId>
    <version>2.15.2</version>
</dependency>
```

- Then run:

```sh
mvn dependency:resolve
```

---

## Equivalent of `package.json` in Java

```xml
<!-- pom.xml -->
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>my-app</artifactId>
    <version>1.0.0</version>
    <packaging>jar</packaging>

    <properties>
        <maven.compiler.source>17</maven.compiler.source>
        <maven.compiler.target>17</maven.compiler.target>
    </properties>

    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
            <version>3.1.0</version>
        </dependency>
    </dependencies>
</project>
```

- `pom.xml` contains project metadata, dependencies, and build configuration.

---

## Equivalent of `npm install` in Java (with Maven)

```sh
mvn clean install
```

- Downloads all dependencies listed in `pom.xml`.
- Compiles the project and runs tests.
- Creates local `.m2/repository` cache (like `node_modules`).

---

## Equivalent of `npm uninstall <package_name>` in Java

- Remove dependency from `pom.xml` manually.
- Then run:

```sh
mvn dependency:purge-local-repository
```

- Or simply:

```sh
mvn clean
```

---

## Equivalent of `npm run <script>` in Java

- Maven uses plugins and goals instead of scripts:

```sh
mvn clean compile
mvn test
mvn package
mvn spring-boot:run
```

- Custom goals can be defined in `pom.xml` using plugins.

---

## Equivalent of `npx <tool>` in Java

```sh
mvn exec:java -Dexec.mainClass="com.example.Tool"
```

- Or use Maven plugins directly:

```sh
mvn flyway:migrate
mvn liquibase:update
```

---

## Equivalent of `JSON.stringify()` in Java

```java
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.Map;

ObjectMapper mapper = new ObjectMapper();
Map<String, Object> data = Map.of(
    "name", "John Doe",
    "age", 30
);
String jsonString = mapper.writeValueAsString(data);
```

---

## Equivalent of dotenv (`process.env`) in Java

**Using Spring Boot (application.properties):**

```properties
# application.properties
my.secret=supersecret
```

```java
@Value("${my.secret}")
private String mySecret;
```

**Using dotenv-java library:**

```xml
<dependency>
    <groupId>io.github.cdimascio</groupId>
    <artifactId>dotenv-java</artifactId>
    <version>3.0.0</version>
</dependency>
```

```java
import io.github.cdimascio.dotenv.Dotenv;

Dotenv dotenv = Dotenv.load();
String secret = dotenv.get("MY_SECRET");
```

---

## Starting a Typical Server

**Spring Boot REST API:**

```java
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;
import java.util.Map;

@SpringBootApplication
@RestController
public class Application {

    public static void main(String[] args) {
        SpringApplication.run(Application.class, args);
    }

    @GetMapping("/")
    public Map<String, String> root() {
        return Map.of("message", "Hello World");
    }
}
```

**Spring Boot with separate Controller:**

```java
@RestController
public class BookController {

    @GetMapping("/books")
    public List<String> getBooks() {
        return List.of("Book 1", "Book 2");
    }

    @PostMapping("/books")
    public Map<String, String> createBook(@RequestBody Map<String, String> book) {
        return Map.of("status", "created", "title", book.get("title"));
    }
}
```

---

## Middleware Example (Spring Boot)

**Filter (Servlet-based):**

```java
import javax.servlet.*;
import javax.servlet.http.HttpServletRequest;
import java.io.IOException;

@Component
public class LoggingFilter implements Filter {

    @Override
    public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain)
            throws IOException, ServletException {
        HttpServletRequest httpRequest = (HttpServletRequest) request;
        System.out.println("Request: " + httpRequest.getMethod() + " " + httpRequest.getRequestURI());
        chain.doFilter(request, response);
    }
}
```

**Interceptor (Spring MVC):**

```java
import org.springframework.web.servlet.HandlerInterceptor;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

@Component
public class LoggingInterceptor implements HandlerInterceptor {

    @Override
    public boolean preHandle(HttpServletRequest request, HttpServletResponse response, Object handler) {
        System.out.println("Request: " + request.getMethod() + " " + request.getRequestURI());
        return true;
    }
}
```

---

## Routing Example (Spring Boot)

**Annotation-based routing:**

```java
@RestController
@RequestMapping("/api/v1")
public class BookController {

    @GetMapping("/books")
    public List<String> listBooks() {
        return List.of("Book 1", "Book 2");
    }

    @GetMapping("/books/{id}")
    public Map<String, Object> getBook(@PathVariable Long id) {
        return Map.of("id", id, "title", "Book " + id);
    }

    @PostMapping("/books")
    public ResponseEntity<Map<String, Object>> createBook(@RequestBody Map<String, String> book) {
        Map<String, Object> response = Map.of(
            "id", 1,
            "title", book.get("title"),
            "status", "created"
        );
        return ResponseEntity.status(201).body(response);
    }
}
```

---

## Notes

- Maven is Java's most popular build tool and dependency manager.
- Java is compiled, so `mvn compile` or `mvn package` is required before running.
- Spring Boot is the most popular framework for web applications.
- Use `mvn spring-boot:run` for development with auto-reload.
- Dependencies are cached in `~/.m2/repository` (like `node_modules`).
- Java has strong typing and excellent IDE support.
