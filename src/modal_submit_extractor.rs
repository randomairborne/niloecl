use serde_json::Value;
use twilight_model::application::interaction::{
    modal::ModalInteractionComponent, Interaction, InteractionData,
};

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

        let mut json_map = serde_json::Map::with_capacity(ms.components.len());

        for component in &ms.components {
            map_component_extend(component, &mut json_map);
        }

        Ok(Self {
            custom_id: ms.custom_id.clone(),
            data: serde_json::from_value(Value::Object(json_map))?,
        })
    }
}

macro_rules! jsonify_array {
    ($map:expr, $val:expr) => {{
        $map.insert(
            $val.custom_id.clone(),
            Value::Array(
                $val.values
                    .iter()
                    .map(|v| Value::String(v.to_string()))
                    .collect(),
            ),
        );
    }};
}

fn map_component_extend(
    component: &ModalInteractionComponent,
    map: &mut serde_json::Map<String, Value>,
) {
    match component {
        ModalInteractionComponent::Label(mil) => map_component_extend(&mil.component, map),
        ModalInteractionComponent::ActionRow(ar) => {
            for subcomponent in &ar.components {
                map_component_extend(subcomponent, map);
            }
        }
        ModalInteractionComponent::StringSelect(ss) => jsonify_array!(map, ss),
        ModalInteractionComponent::UserSelect(us) => jsonify_array!(map, us),
        ModalInteractionComponent::RoleSelect(rs) => jsonify_array!(map, rs),
        ModalInteractionComponent::MentionableSelect(ms) => jsonify_array!(map, ms),
        ModalInteractionComponent::TextInput(ti) => {
            map.insert(ti.custom_id.clone(), Value::String(ti.value.clone()));
        }
        ModalInteractionComponent::ChannelSelect(cs) => jsonify_array!(map, cs),
        ModalInteractionComponent::FileUpload(fu) => jsonify_array!(map, fu),
        ModalInteractionComponent::TextDisplay(_) | ModalInteractionComponent::Unknown(_) => {}
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
