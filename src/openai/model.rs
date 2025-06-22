use crate::generator::media::{Episode, Media, Movie};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct JsonSchema2 {
    pub strict: bool,
    pub name: String,
    pub schema: Value,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub f_type: String,
    pub json_schema: JsonSchema2,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub response_format: Option<ResponseFormat>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ChoiceMessage {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Choice {
    pub message: ChoiceMessage,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ChatCompletionResponse {
    pub choices: Vec<Choice>,
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Response {
    pub is_movie: bool,
    pub success: bool,
    pub episode: Episode,
    pub movie: Movie,
}

impl From<Response> for Media {
    fn from(val: Response) -> Self {
        if val.is_movie {
            return Media::Movie(val.movie);
        }

        Media::Episode(val.episode)
    }
}
