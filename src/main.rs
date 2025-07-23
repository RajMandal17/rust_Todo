// Rust Todo API with MariaDB for Java Developers
// ==============================================
// Prerequisites:
// - Rust toolchain (install via rustup)
// - MariaDB server running locally
// - Database setup: CREATE DATABASE todo_db; CREATE TABLE todos (...)
// - Dependencies: mysql, serde, serde_json, warp, tokio
//
// This app exposes REST endpoints for CRUD operations on todos.
//
// Java Comparison:
// - DB connection: JDBC vs mysql crate
// - Model: POJO vs struct
// - API: Spring Boot vs warp
// - Async: CompletableFuture vs tokio
// - Error handling: try/catch vs Result

mod models;
mod routes;

use mysql::*;
use mysql::prelude::*;
use warp::Filter;
use std::sync::{Arc, Mutex};
use models::todo::Todo;
use routes::todo_routes::{add_todo, list_todos, mark_complete, delete_todo};

// Main function starts the warp server and sets up routes
// Java: Spring Boot main method
#[tokio::main]
async fn main() {
    // Replace with your MariaDB credentials
    let url = "mysql://user:password@localhost:3306/todo_db";
    let pool = Pool::new(url).expect("Failed to create pool.");
    let pool = Arc::new(Mutex::new(pool));

    // GET /todos - list all todos
    let pool_filter = warp::any().map(move || pool.clone());
    let list = warp::path("todos")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(list_todos);

    // POST /todos - add a todo
    let add = warp::path("todos")
        .and(warp::post())
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(add_todo);

    // PUT /todos/{id} - mark complete
    let complete = warp::path!("todos" / u32)
        .and(warp::put())
        .and(pool_filter.clone())
        .and_then(mark_complete);

    // DELETE /todos/{id} - delete todo
    let delete = warp::path!("todos" / u32)
        .and(warp::delete())
        .and(pool_filter.clone())
        .and_then(delete_todo);

    let routes = list.or(add).or(complete).or(delete);
    println!("Starting server on http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
