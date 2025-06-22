use crate::openai::model::{
    ChatCompletionRequest, ChatCompletionResponse, JsonSchema2, Message, Response, ResponseFormat,
};
use schemars::schema_for;

pub struct Client {
    pub url: String,
    pub api_token: String,
}

impl Client {
    pub fn new(
        instance_name: &str,
        deployment_name: &str,
        version: &str,
        api_token: &str,
    ) -> Client {
        let url = format!(
            "https://{}.openai.azure.com/openai/deployments/{}/chat/completions?api-version={}",
            instance_name, deployment_name, version
        );

        Client {
            url,
            api_token: api_token.to_string(),
        }
    }

    pub fn prompt(&self, text: String) -> anyhow::Result<Response> {
        let request = self.create_request(text)?;
        let response = reqwest::blocking::Client::new()
            .post(&self.url)
            .json(&request)
            .bearer_auth(&self.api_token)
            .send()?
            .error_for_status()?
            .json::<ChatCompletionResponse>()?;

        let s = serde_json::from_str::<Response>(&response.choices[0].message.content)?;

        Ok(s)
    }

    fn create_request(&self, text: String) -> anyhow::Result<ChatCompletionRequest> {
        let schema = schema_for!(Response);
        let schema_value = serde_json::to_value(&schema)?;
        let format = ResponseFormat {
            f_type: "json_schema".to_string(),
            json_schema: JsonSchema2 {
                name: "parse".to_string(),
                strict: true,
                schema: schema_value,
            },
        };

        let request = ChatCompletionRequest {
            model: "gpt-4.1".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "Extract information from this string".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: text,
                },
            ],
            response_format: Some(format),
        };

        Ok(request)
    }
}
