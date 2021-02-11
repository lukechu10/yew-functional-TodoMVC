#![recursion_limit = "512"]

mod app;
mod copyright;
mod footer;
mod header;
mod item;
mod list;
mod macros;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::EnumIter;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TodoStatus {
    Active,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoEntry {
    /// The name of the todo.
    pub name: String,
    pub status: TodoStatus,
    pub id: Uuid,
}
impl TodoEntry {
    pub fn new(name: String) -> Self {
        Self {
            name,
            status: TodoStatus::Active,
            id: Uuid::new_v4(),
        }
    }

    pub fn toggle_status(&mut self) {
        self.status = if self.status == TodoStatus::Active {
            TodoStatus::Completed
        } else {
            TodoStatus::Active
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Filter::All => write!(f, "#/"),
            Filter::Active => write!(f, "#/active"),
            Filter::Completed => write!(f, "#/completed"),
        }
    }
}
