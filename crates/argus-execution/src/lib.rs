use argus_contracts::ExecutionRequest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Decision { PaperAccepted, Rejected(String), ManualApprovalRequired }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaperExecution { pub request_id: String, pub decision: Decision, pub reason: String }

pub fn evaluate(request: &ExecutionRequest, current_block: u64, live_mode: bool) -> PaperExecution {
  if live_mode { return PaperExecution { request_id:request.request_id.clone(), decision:Decision::Rejected("live signer disabled in baseline".into()), reason:"fail closed: external signer requires independent audit".into() }; }
  if request.expiration_block < current_block { return PaperExecution { request_id:request.request_id.clone(), decision:Decision::Rejected("expired".into()), reason:"request validity window closed".into() }; }
  if request.replay_epoch == 0 || request.feature_snapshot_version == 0 { return PaperExecution { request_id:request.request_id.clone(), decision:Decision::Rejected("invalid provenance".into()), reason:"missing replay or snapshot identity".into() }; }
  PaperExecution { request_id:request.request_id.clone(), decision:Decision::PaperAccepted, reason:"validated for simulation only; no transaction is signed or broadcast".into() }
}
