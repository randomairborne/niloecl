use std::convert::Infallible;

use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};

pub trait IntoResponse {
    fn into_response(self) -> InteractionResponse;
}

impl<T: IntoResponse, E: IntoResponse> IntoResponse for Result<T, E> {
    fn into_response(self) -> InteractionResponse {
        self.map_or_else(IntoResponse::into_response, IntoResponse::into_response)
    }
}

impl IntoResponse for InteractionResponse {
    fn into_response(self) -> InteractionResponse {
        self
    }
}

/// this is inconstructible. Should be unreachable.
impl IntoResponse for Infallible {
    fn into_response(self) -> InteractionResponse {
        InteractionResponse {
            kind: InteractionResponseType::Pong,
            data: None,
        }
    }
}
