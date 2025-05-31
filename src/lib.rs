#![deny(
    clippy::correctness,
    clippy::nursery,
    warnings,
    clippy::pedantic,
    clippy::all
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::unused_async,
    clippy::missing_panics_doc,
    clippy::future_not_send,
    clippy::too_many_lines,
    clippy::manual_let_else,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]

pub mod components;
pub mod consts;
pub mod resources;
pub mod systems;
pub mod utils;
