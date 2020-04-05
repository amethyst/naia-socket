
mod internal_shared;

pub mod shared;

#[cfg(feature = "Client")]
mod client;

#[cfg(feature = "Client")]
pub use client::Client as Client;

#[cfg(feature = "Server")]
mod server;

#[cfg(feature = "Server")]
pub use server::Server as Server;

//pub mod error;
//pub use crate::error::GaiaError as Error;
//pub type Result<T> = ::std::result::Result<T, Error>;
