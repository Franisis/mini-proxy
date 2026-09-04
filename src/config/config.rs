use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub listen: String,
    pub upstream: String,
}
