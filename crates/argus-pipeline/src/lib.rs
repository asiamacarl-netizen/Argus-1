use argus_contracts::{stable_id, ExecutionRequest, FeatureSnapshot, PositionUpdate};

pub fn position_to_feature(position: PositionUpdate, feature_version: u64) -> FeatureSnapshot {
  let mut out=FeatureSnapshot { position, feature_version, snapshot_hash:String::new(), confidence_bps:0, risk_bps:10_000, canonical:true };
  out.snapshot_hash=stable_id("feature-snapshot", &out.position); out
}

pub fn feature_to_request(f: &FeatureSnapshot, now: i64, block: u64, replay_epoch: u64) -> Option<ExecutionRequest> {
  let health=f.position.health_factor_bps?;
  if !f.canonical || !f.position.state_hash.is_empty() && health >= f.position.liquidation_threshold_bps { return None; }
  let mut out=ExecutionRequest { request_id:String::new(), timestamp:now, chain_id:f.position.key.chain_id, protocol:f.position.key.protocol.clone(), market_id:f.position.key.market_id.clone(), borrower:f.position.key.borrower.clone(), debt_amount:f.position.debt_amount, collateral_amount:f.position.collateral_amount, expected_net_profit:0, confidence_bps:f.confidence_bps, risk_bps:f.risk_bps, expiration_block:block.saturating_add(2), feature_snapshot_version:f.feature_version, feature_snapshot_hash:f.snapshot_hash.clone(), replay_epoch };
  out.request_id=stable_id("execution-request", &out); Some(out)
}
