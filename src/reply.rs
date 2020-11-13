use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Reply {
    pub(crate) bot_id: String,
    pub(crate) text: String
}