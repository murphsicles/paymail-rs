#![doc = "A fast, asynchronous Rust library for the BSV PayMail protocol."]

pub mod client;
pub mod utils;
pub mod errors;
pub mod models;
pub mod protocols;
pub mod resolver;
pub mod server;

pub use client::PaymailClient;
pub use errors::PaymailError;
