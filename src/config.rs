use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Replies {
    pub replies: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub replies: HashMap<String, Replies>,
    pub banned_games: Vec<String>,
}
