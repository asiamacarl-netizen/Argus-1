use anyhow::Result;
use argus_contracts::Envelope;
use redis::{aio::ConnectionManager, AsyncCommands};
use serde::{de::DeserializeOwned, Serialize};

pub const RAW: &str = "argus.events.raw.v1";
pub const POSITION: &str = "argus.events.position.v1";
pub const FEATURES: &str = "argus.features.snapshot.v1";
pub const REQUESTS: &str = "argus.execution.requests.v1";
pub const REORG: &str = "argus.reorg.v1";

pub struct EventBus { conn: ConnectionManager }
impl EventBus {
  pub async fn connect(url: &str) -> Result<Self> { Ok(Self { conn: redis::Client::open(url)?.get_connection_manager().await? }) }
  pub async fn publish<T: Serialize>(&mut self, stream: &str, msg: &Envelope<T>) -> Result<String> { let bytes=rmp_serde::to_vec_named(msg)?; Ok(self.conn.xadd(stream, "payload", bytes).await?) }
  pub async fn ensure_group(&mut self, stream: &str, group: &str) -> Result<()> { let _: redis::RedisResult<()> = redis::cmd("XGROUP").arg("CREATE").arg(stream).arg(group).arg("0").arg("MKSTREAM").query_async(&mut self.conn).await; Ok(()) }
  pub fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<Envelope<T>> { Ok(rmp_serde::from_slice(bytes)?) }
}
