use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Subtitle {
    // pub id: String,
    pub attributes: Attributes,
}

#[derive(Deserialize, Debug)]
pub struct Attributes {
    pub feature_details: FeatureDetail,
}

#[derive(Deserialize, Debug)]
pub struct Movie {
    pub year: u32,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct Episode {
    pub title: String,
    pub season_number: i32,
    pub episode_number: i32,
    pub parent_title: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "feature_type")]
pub enum FeatureDetail {
    Movie(Movie),
    Episode(Episode),
}

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub total_pages: u32,
    // pub total_count: u32,
    // pub per_page: u32,
    pub page: u32,
    pub data: Vec<T>,
}
