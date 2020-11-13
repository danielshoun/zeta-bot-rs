use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
	pub(crate) user_id: String,
	pub(crate) sender_type: String,
	pub(crate) name: String,
	pub(crate) text: String
}
