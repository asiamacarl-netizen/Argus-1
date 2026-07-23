use argus_contracts::ExecutionRequest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Decision {
    PaperAccepted,
    Rejected(String),
    ManualApprovalRequired,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaperExecution {
    pub request_id: String,
    pub decision: Decision,
    pub reason: String,
}

pub fn evaluate(request: &ExecutionRequest, current_block: u64, live_mode: bool) -> PaperExecution {
    if live_mode {
        return rejected(request, "live signer disabled in baseline", "external signer requires independent audit");
    }
    if request.request_id.is_empty() {
        return rejected(request, "missing request id", "request identity is required");
    }
    if request.expiration_block < current_block {
        return rejected(request, "expired", "request validity window closed");
    }
    if request.replay_epoch == 0 || request.feature_snapshot_version == 0 {
        return rejected(request, "invalid provenance", "missing replay or snapshot identity");
    }
    PaperExecution {
        request_id: request.request_id.clone(),
        decision: Decision::PaperAccepted,
        reason: "validated for simulation only; no transaction is signed or broadcast".into(),
    }
}

fn rejected(request: &ExecutionRequest, decision: &str, reason: &str) -> PaperExecution {
    PaperExecution {
        request_id: request.request_id.clone(),
        decision: Decision::Rejected(decision.into()),
        reason: reason.into(),
    }
}
