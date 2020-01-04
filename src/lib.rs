#![allow(unused_must_use)]

#[macro_use]
pub mod utils;
pub mod bookmarks;
pub mod config;
pub mod gopher;
pub mod help;
pub mod history;
pub mod menu;
pub mod text;
pub mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PLATFORM: &str = env!("PLATFORM");
pub const BUG_URL: &str = "https://github.com/dvkt/phetch/issues/new";
