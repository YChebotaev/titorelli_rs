use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct TrainExample {
    pub r#type: String,
    pub text: String,
}
