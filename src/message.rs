use serde::{Serialize, Deserialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
	pub(crate) user_id: String,
	pub(crate) sender_type: String,
	pub(crate) name: String,
	pub(crate) text: String
}
