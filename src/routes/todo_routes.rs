use mysql::*;
use mysql::prelude::*;
use crate::models::todo::Todo;

// Add a new todo to the database
pub fn add_todo(conn: &mut PooledConn, title: &str) {
    conn.exec_drop(
        "INSERT INTO todos (title, completed) VALUES (:title, false)",
        params! { "title" => title }
    ).expect("Failed to insert todo");
}

// List all todos from the database
pub fn list_todos(conn: &mut PooledConn) -> Vec<Todo> {
    conn.query_map(
        "SELECT id, title, completed FROM todos",
        |(id, title, completed)| Todo { id, title, completed }
    ).expect("Failed to fetch todos")
}

// Mark a todo as complete
pub fn mark_complete(conn: &mut PooledConn, id: u32) {
    conn.exec_drop(
        "UPDATE todos SET completed = true WHERE id = :id",
        params! { "id" => id }
    ).expect("Failed to update todo");
}

// Delete a todo
pub fn delete_todo(conn: &mut PooledConn, id: u32) {
    conn.exec_drop(
        "DELETE FROM todos WHERE id = :id",
        params! { "id" => id }
    ).expect("Failed to delete todo");
}
