#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
pub mod app;
mod domain;
pub mod errors;
mod infra;
mod web;

pub use infra::db;
