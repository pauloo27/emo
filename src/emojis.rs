use anyhow::Context;
use anyhow::Result as AnyResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Emoji {
    pub emoji: String,
    pub group: String,
    pub subgroups: String,
    pub annotation: String,
    pub tags: String,
    pub openmoji_tags: String,
    pub skintone_combination: String,
    pub skintone_base_emoji: String,
}

pub fn load_emojis() -> AnyResult<Vec<Emoji>> {
    let data = include_str!("./emojis.json");

    serde_json::from_str::<Vec<Emoji>>(data).context("Failed to parse emoji list")
}
