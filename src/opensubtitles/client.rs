use crate::hasher::MovieHash;
use crate::opensubtitles::model::{Response, Subtitle};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub struct OpenSubtitlesClient {
    api_key: String,
}

impl OpenSubtitlesClient {
    pub fn new(api_key: impl ToString) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    pub fn search_by_moviehash(&self, moviehash: &MovieHash) -> anyhow::Result<Vec<Subtitle>> {
        let params = [
            ("moviehash", moviehash.to_string()),
            ("moviehash_match", "only".to_string()),
        ];

        self.make_paged_request(
            "https://api.opensubtitles.com/api/v1/subtitles",
            params.into(),
        )
    }

    fn make_paged_request<T>(
        &self,
        url: &str,
        mut params: HashMap<&str, String>,
    ) -> anyhow::Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let mut data = vec![];
        let mut page = 1;
        loop {
            params.insert("page", page.to_string());
            let url = reqwest::Url::parse_with_params(url, &params)?;
            let response = reqwest::blocking::Client::new()
                .get(url.clone())
                .header("Api-Key", &self.api_key)
                .header("User-Agent", "")
                .send()?
                .error_for_status()?
                .json::<Response<T>>()?;

            for item in response.data {
                data.push(item);
            }

            if response.total_pages <= response.page {
                break;
            }

            page += 1;
        }

        Ok(data)
    }
}
