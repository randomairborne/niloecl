use std::convert::Infallible;

use twilight_model::application::interaction::Interaction;

use crate::handler::FromRequest;

impl<S: Sync> FromRequest<S> for Interaction {
    type Rejection = Infallible;
    async fn from_request(req: &mut Interaction, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(req.clone())
    }
}

pub struct State<S>(pub S);

impl<S: Clone + Sync> FromRequest<S> for State<S> {
    type Rejection = Infallible;
    async fn from_request(_req: &mut Interaction, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self(state.clone()))
    }
}
