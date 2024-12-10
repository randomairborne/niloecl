use serde_json::Value;
use twilight_model::application::interaction::{Interaction, InteractionData};

use crate::{handler::FromRequest, into_response_impls::BasicErrorReport, IntoResponse};

#[derive(Debug, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct ModalSubmit<T> {
    pub custom_id: String,
    pub data: T,
}

impl<S: Sync, T: for<'a> serde::Deserialize<'a>> FromRequest<S> for ModalSubmit<T> {
    type Rejection = ModalSubmitError;

    async fn from_request(req: &mut Interaction, _: &S) -> Result<Self, Self::Rejection> {
        let Some(InteractionData::ModalSubmit(ms)) = &req.data else {
            return if req.data.is_some() {
                Err(ModalSubmitError::WrongInteractionData)
            } else {
                Err(ModalSubmitError::NoInteractionData)
            };
        };

        let mut json_map = serde_json::Map::with_capacity(5);

        for component in &ms.components {
            for component in &component.components {
                json_map.insert(
                    component.custom_id.clone(),
                    Value::String(component.value.clone().unwrap_or_else(String::new)),
                );
            }
        }

        Ok(Self {
            custom_id: ms.custom_id.clone(),
            data: serde_json::from_value(Value::Object(json_map))?,
        })
    }
}

#[derive(Debug)]
pub enum ModalSubmitError {
    SerdeJson(serde_json::Error),
    WrongInteractionData,
    NoInteractionData,
}

impl From<serde_json::Error> for ModalSubmitError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

impl std::fmt::Display for ModalSubmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::SerdeJson(_) => "Could not deserialize custom modal struct",
            Self::WrongInteractionData => "Invalid interaction data",
            Self::NoInteractionData => "No interaction data",
        };
        f.write_str(message)
    }
}

impl IntoResponse for ModalSubmitError {
    fn into_response(self) -> twilight_model::http::interaction::InteractionResponse {
        BasicErrorReport(self).into_response()
    }
}
