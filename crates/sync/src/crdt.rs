use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use uhlc::NTP64;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SharedOperationCreateData {
	Unique(Map<String, Value>),
	Atomic,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SharedOperationData {
	Create(SharedOperationCreateData),
	Update { field: String, value: Value },
	Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SharedOperation {
	pub record_id: Value, // Uuid,
	pub model: String,
	pub data: SharedOperationData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OwnedOperationData {
	Create(Map<String, Value>),
	Update(Map<String, Value>),
	Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OwnedOperationItem {
	pub id: Value,
	pub data: OwnedOperationData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OwnedOperation {
	pub model: String,
	pub items: Vec<OwnedOperationItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum CRDTOperationType {
	Shared(SharedOperation),
	// Relation(RelationOperation),
	Owned(OwnedOperation),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CRDTOperation {
	pub node: Uuid,
	pub timestamp: NTP64,
	pub id: Uuid,
	#[serde(flatten)]
	pub typ: CRDTOperationType,
}

impl Debug for CRDTOperation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CRDTOperation")
			.field("node", &self.node.to_string())
			.field("timestamp", &self.timestamp.to_string())
			.field("typ", &self.typ)
			.finish()
	}
}
