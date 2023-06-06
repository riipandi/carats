pub mod cmd;
pub mod handler;
pub mod routes;
pub mod schema;
pub mod server;
pub mod utils;

pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
