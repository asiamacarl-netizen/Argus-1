use argus_contracts::{stable_id, ExecutionRequest, FeatureSnapshot, PositionUpdate};

pub fn position_to_feature(position: PositionUpdate, feature_version: u64) -> FeatureSnapshot {
    let mut snapshot = FeatureSnapshot {
        position,
        feature_version,
        snapshot_hash: String::new(),
        confidence_bps: 0,
        risk_bps: 10_000,
        canonical: true,
    };
    snapshot.snapshot_hash = stable_id("feature-snapshot", &snapshot.position);
    snapshot
}

pub fn feature_to_request(
    snapshot: &FeatureSnapshot,
    now: i64,
    block: u64,
    replay_epoch: u64,
) -> Option<ExecutionRequest> {
    let health = snapshot.position.health_factor_bps?;
    if !snapshot.canonical || health >= snapshot.position.liquidation_threshold_bps {
        return None;
    }

    let mut request = ExecutionRequest {
        request_id: String::new(),
        timestamp: now,
        chain_id: snapshot.position.key.chain_id,
        protocol: snapshot.position.key.protocol.clone(),
        market_id: snapshot.position.key.market_id.clone(),
        borrower: snapshot.position.key.borrower.clone(),
        debt_amount: snapshot.position.debt_amount,
        collateral_amount: snapshot.position.collateral_amount,
        expected_net_profit: 0,
        confidence_bps: snapshot.confidence_bps,
        risk_bps: snapshot.risk_bps,
        expiration_block: block.saturating_add(2),
        feature_snapshot_version: snapshot.feature_version,
        feature_snapshot_hash: snapshot.snapshot_hash.clone(),
        replay_epoch,
    };
    request.request_id = stable_id("execution-request", &request);
    Some(request)
}
