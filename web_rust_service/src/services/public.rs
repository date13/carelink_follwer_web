use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloWorld {
  name: String,
  age: i32,
}

impl HelloWorld {
  pub fn new(name: String, age: i32) -> Self {
    Self { name, age }
  }
}