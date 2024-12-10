use twilight_model::{
    channel::message::MessageFlags,
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{embed::EmbedBuilder, InteractionResponseDataBuilder};

use crate::IntoResponse;

pub struct BasicErrorReport<T>(pub T);

impl<T: std::fmt::Display> IntoResponse for BasicErrorReport<T> {
    fn into_response(self) -> InteractionResponse {
        let embed = EmbedBuilder::new().description(self.0.to_string()).build();
        let data = InteractionResponseDataBuilder::new()
            .flags(MessageFlags::EPHEMERAL)
            .embeds([embed])
            .build();
        InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(data),
        }
    }
}
