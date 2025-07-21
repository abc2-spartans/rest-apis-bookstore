# Node.js/npm to Rust (with Cargo) Equivalents

This guide maps common Node.js/npm commands and concepts to their Rust equivalents, using Cargo as the package manager and build tool.

---

## Equivalent of `npm init` in Rust

```sh
cargo init
```
- Creates a new Rust project in the current directory with `Cargo.toml` and `src/main.rs`.
- For a new directory:

```sh
cargo new my_project
```
- Creates a new directory with a complete Rust project structure.

---

## Equivalent of `npm run start` in Rust

```sh
cargo run
```
- Compiles and runs your Rust project (main function in `src/main.rs`).
- For development with auto-reload, use `cargo-watch`:

```sh
cargo install cargo-watch
cargo watch -x run
```

---

## Equivalent of `npm install <package_name>` in Rust

```sh
cargo add <crate_name>
```
- Adds a dependency to your `Cargo.toml` and downloads it.
- Manual alternative: Edit `Cargo.toml` and run `cargo build`.

---

## Equivalent of `package.json` in Rust

```toml
# Cargo.toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
```
- `Cargo.toml` contains project metadata, dependencies, and build configuration.

---

## Equivalent of `npm install` in Rust

```sh
cargo build
```
- Downloads and compiles all dependencies listed in `Cargo.toml`.
- Creates `Cargo.lock` file (equivalent to `package-lock.json`).

---

## Equivalent of `npm uninstall <package_name>` in Rust

```sh
cargo remove <crate_name>
```
- Removes the dependency from `Cargo.toml`.
- Manual alternative: Remove from `Cargo.toml` and run `cargo build`.

---

## Equivalent of `npm run <script>` in Rust

- Rust doesn't have built-in scripts like `package.json`.
- Use `Makefile`, shell scripts, or `cargo-make`:

```sh
cargo install cargo-make
```
Then create `Makefile.toml` with custom tasks.

---

## Equivalent of `npx <tool>` in Rust

```sh
cargo install <tool>
<tool>
```
- Or run directly:

```sh
cargo run --bin <tool>
```

---

## Equivalent of `JSON.stringify()` in Rust

```rust
use serde_json;

let data = serde_json::json!({
    "name": "John Doe",
    "age": 30
});
let json_string = data.to_string();
```

---

## Equivalent of dotenv (`process.env`) in Rust

Install:
```sh
cargo add dotenv
```
Use:
```rust
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let secret = env::var("MY_SECRET").expect("MY_SECRET not set");
    println!("{}", secret);
}
```

---

## Starting a Typical Server

**Axum (async web framework):**
```rust
use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<Value> {
    Json(json!({ "message": "Hello World" }))
}
```

**Warp (async web framework):**
```rust
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path::end()
        .map(|| warp::reply::json(&serde_json::json!({
            "message": "Hello World"
        })));

    warp::serve(hello)
        .run(([0, 0, 0, 0], 3000))
        .await;
}
```

---

## Middleware Example (Axum/Warp)

**Axum:**
```rust
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::http::Request;

async fn log_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    println!("Request: {} {}", request.method(), request.uri());
    next.run(request).await
}

// Apply to router
let app = Router::new()
    .route("/", get(root))
    .layer(middleware::from_fn(log_middleware));
```

**Warp:**
```rust
use warp::Filter;

fn with_log() -> impl Filter<Extract = (), Error = std::convert::Infallible> + Clone {
    warp::any().map(|| {
        println!("Request received");
    })
}

let routes = warp::path::end()
    .and(with_log())
    .map(|| "Hello World");
```

---

## Routing Example (Axum/Warp)

**Axum:**
```rust
use axum::{routing::get, Router, Json};

let app = Router::new()
    .route("/books", get(list_books));

async fn list_books() -> Json<Vec<&'static str>> {
    Json(vec!["Book 1", "Book 2"])
}
```

**Warp:**
```rust
use warp::Filter;

let books = warp::path("books")
    .and(warp::get())
    .map(|| warp::reply::json(&vec!["Book 1", "Book 2"]));
```

---

## Notes
- Cargo is Rust's built-in package manager and build tool.
- Rust is compiled, so `cargo build` is required before running.
- Popular web frameworks: Axum, Warp, Actix-web, Rocket.
- Use `cargo-watch` for development auto-reload.
- Rust has excellent performance and memory safety guarantees.
