use hc_utils::WrappedAgentPubKey;
use hdk3::prelude::*;

mod assignment;
mod membrane_roles;
mod utils;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

entry_defs![Path::entry_def(), assignment::MembraneRoleAssignment::entry_def(), membrane_roles::MembraneRole::entry_def()];

#[hdk_extern]
pub fn who_am_i(_: ()) -> ExternResult<WrappedAgentPubKey> {
    let agent_info = agent_info()?;

    Ok(WrappedAgentPubKey(agent_info.agent_initial_pubkey))
}
