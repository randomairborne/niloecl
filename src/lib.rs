#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
//! # niloecl

//! niloECL (Nilo's Extractor Command Library) implements the Axum
//! [extractor machinery](https://docs.rs/axum/latest/axum/extract/index.html) and handler system for
//! [Twilight](https://twilight.rs)'s
//! [`Interaction`](https://docs.rs/twilight-model/0.16.0-rc.1/twilight_model/application/interaction/struct.Interaction.html)
//! and [`InteractionResponse.`](https://docs.rs/twilight-model/0.16.0-rc.1/twilight_model/http/interaction/struct.InteractionResponse.html).

mod extract;
mod handler;
mod into_response;

pub use extract::State;
pub use handler::{make as make_handler, FromRequest, Handler, HandlerFunction};
pub use into_response::IntoResponse;