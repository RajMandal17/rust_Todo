// Import traits for serialization and deserialization.
// In Java, you might use libraries like Jackson or Gson for JSON conversion.
use serde::{Serialize, Deserialize};

// Todo model, similar to a Java POJO (Plain Old Java Object)
// #[derive(...)] automatically implements useful traits:
// - Debug: like Java's toString(), lets you print the struct for debugging
// - Serialize/Deserialize: like Java's @JsonSerialize/@JsonDeserialize, enables JSON conversion
// - Clone: like Java's Cloneable, allows copying the struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    // 'pub' makes fields accessible outside the module, similar to 'public' in Java
    pub id: u32,         // Like 'public int id;' in Java
    pub title: String,   // Like 'public String title;' in Java
    pub completed: bool, // Like 'public boolean completed;' in Java
}
// In Java, you would write:
// public class Todo {
//     public int id;
//     public String title;
//     public boolean completed;
// }
