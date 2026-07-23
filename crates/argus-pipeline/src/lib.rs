use argus_contracts::{stable_id, ExecutionRequest, FeatureSnapshot, PositionUpdate};

pub fn position_to_feature(position: PositionUpdate, feature_version: u64) -> FeatureSnapshot {
  let mut out=FeatureSnapshot { position, feature_version, snapshot_hash:String::new(), confidence_bps:0, risk_bps:10_000, canonical:true };
  out.snapshot_hash=stable_id("feature-snapshot", &out.position); out
}

pub fn feature_to_request(f: &FeatureSnapshot, now: i64, block: u64, replay_epoch: u64) -> Option<ExecutionRequest> {
  let health=f.position.health_factor_bps?;
  if !f.canonical || !f.position.state_hash.is_empty() && health >= f.position.liquidation_threshold_bps { return None; }
  let mut out=ExecutionRequest { request_id:String::new(), timestamp:now, chain_id:f.position.key.chain_id, protocol:f.position.key.protocol.clone(), market_id:f.position.key.market_id.clone(), body:vec![] };
  out.request_id=stable_id("execution-request", &out); Some(out)
}
