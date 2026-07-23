use alloy_primitives::U256;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};

pub type Amount = U256;
pub type BlockNumber = u64;
pub const SCHEMA_VERSION: u16 = 1;

pub fn ser_amount<S: Serializer>(value: &Amount, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
}

pub fn de_amount<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Amount, D::Error> {
    String::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)
}

pub fn stable_id<T: Serialize>(domain: &str, value: &T) -> String {
    let mut hasher = Sha256::new();
    hasher.update(domain.as_bytes());
    hasher.update([0]);
    hasher.update(serde_json::to_vec(value).expect("canonical serialization"));
    format!("{:x}", hasher.finalize())
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PositionKey {
    pub chain_id: u64,
    pub protocol: String,
    pub market_id: String,
    pub borrower: String,
    pub collateral_asset: String,
    pub debt_asset: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Envelope<T> {
    pub schema_version: u16,
    pub message_id: String,
    pub event_type: String,
    pub producer: String,
    pub replay_epoch: u64,
    pub source_block: BlockNumber,
    pub source_block_hash: String,
    pub canonical: bool,
    pub payload: T,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PositionUpdate {
    pub key: PositionKey,
    pub position_version: u64,
    pub source_event_id: String,
    pub source_timestamp: i64,
    pub block_number: BlockNumber,
    #[serde(serialize_with = "ser_amount", deserialize_with = "de_amount")]
    pub collateral_amount: Amount,
    #[serde(serialize_with = "ser_amount", deserialize_with = "de_amount")]
    pub debt_amount: Amount,
    pub health_factor_bps: Option<u64>,
    pub liquidation_threshold_bps: u64,
    pub state_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeatureSnapshot {
    pub position: PositionUpdate,
    pub feature_version: u64,
    pub snapshot_hash: String,
    pub confidence_bps: u64,
    pub risk_bps: u64,
    pub canonical: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionRequest {
    pub request_id: String,
    pub timestamp: i64,
    pub chain_id: u64,
    pub protocol: String,
    pub market_id: String,
    pub borrower: String,
    #[serde(serialize_with = "ser_amount", deserialize_with = "de_amount")]
    pub debt_amount: Amount,
    #[serde(serialize_with = "ser_amount", deserialize_with = "de_amount")]
    pub collateral_amount: Amount,
    pub expected_net_profit: i128,
    pub confidence_bps: u64,
    pub risk_bps: u64,
    pub expiration_block: BlockNumber,
    pub feature_snapshot_version: u64,
    pub feature_snapshot_hash: String,
    pub replay_epoch: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReorgNotice {
    pub chain_id: u64,
    pub fork_block: BlockNumber,
    pub orphaned_block_hashes: Vec<String>,
    pub affected_keys: Vec<PositionKey>,
    pub replay_epoch: u64,
}
