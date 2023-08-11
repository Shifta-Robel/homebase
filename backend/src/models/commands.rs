use serde::Deserialize;

#[derive(Deserialize)]
pub struct Command {
    pub command: String,
}
