use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceConfig {
	pub kind: String,
	pub name: String,
	pub namespace: String,
	pub key: String,
	pub value: String,
}
