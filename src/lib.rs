#![recursion_limit = "512"]

mod app;
mod copyright;
mod footer;
mod header;
mod item;
mod list;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl<'a> Into<Href> for &'a Filter {
    fn into(self) -> Href {
        match *self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Completed => "#/completed".into(),
        }
    }
}
