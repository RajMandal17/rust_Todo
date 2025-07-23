// Import MySQL crate and prelude for database operations
// In Java, you would import java.sql.Connection, PreparedStatement, ResultSet, etc.
use mysql::*;
use mysql::prelude::*;
use crate::models::todo::Todo; // Import the Todo struct from the models module

// Add a new todo to the database
// Java equivalent: Use PreparedStatement to execute an INSERT query
pub fn add_todo(conn: &mut PooledConn, title: &str) {
    conn.exec_drop(
        "INSERT INTO todos (title, completed) VALUES (:title, false)",
        params! { "title" => title }
    ).expect("Failed to insert todo");
    // In Java:
    // PreparedStatement stmt = conn.prepareStatement("INSERT INTO todos (title, completed) VALUES (?, false)");
    // stmt.setString(1, title);
    // stmt.executeUpdate();
}

// List all todos from the database
// Java equivalent: Use ResultSet to fetch rows from a SELECT query
pub fn list_todos(conn: &mut PooledConn) -> Vec<Todo> {
    conn.query_map(
        "SELECT id, title, completed FROM todos",
        |(id, title, completed)| Todo { id, title, completed }
    ).expect("Failed to fetch todos")
    // In Java:
    // PreparedStatement stmt = conn.prepareStatement("SELECT id, title, completed FROM todos");
    // ResultSet rs = stmt.executeQuery();
    // while (rs.next()) { ... }
}

// Mark a todo as complete
// Java equivalent: Use PreparedStatement to execute an UPDATE query
pub fn mark_complete(conn: &mut PooledConn, id: u32) {
    conn.exec_drop(
        "UPDATE todos SET completed = true WHERE id = :id",
        params! { "id" => id }
    ).expect("Failed to update todo");
    // In Java:
    // PreparedStatement stmt = conn.prepareStatement("UPDATE todos SET completed = true WHERE id = ?");
    // stmt.setInt(1, id);
    // stmt.executeUpdate();
}

// Delete a todo
// Java equivalent: Use PreparedStatement to execute a DELETE query
pub fn delete_todo(conn: &mut PooledConn, id: u32) {
    conn.exec_drop(
        "DELETE FROM todos WHERE id = :id",
        params! { "id" => id }
    ).expect("Failed to delete todo");
    // In Java:
    // PreparedStatement stmt = conn.prepareStatement("DELETE FROM todos WHERE id = ?");
    // stmt.setInt(1, id);
    // stmt.executeUpdate();
}
