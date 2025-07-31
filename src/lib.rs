#![doc = "A fast, asynchronous Rust library for the BSV PayMail protocol."]

pub mod client;
pub mod errors;
pub mod models;
pub mod protocols;
pub mod resolver;
pub mod server;
pub mod utils;

pub use client::PaymailClient;
pub use errors::PaymailError;
