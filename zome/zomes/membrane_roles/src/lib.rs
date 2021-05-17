use admin::{admin_role, ADMIN_ROLE_NAME};
use holo_hash::{AgentPubKeyB64};
use hdk::prelude::*;
use membrane_roles::{create_membrane_role, CreateMembraneRoleInput};
use progenitor::{am_i_progenitor};

mod admin;
mod assignment;
mod membrane_roles;
mod progenitor;
mod utils;

pub fn err(reason: &str) -> WasmError {
    WasmError::Guest(String::from(reason))
}

entry_defs![
    Path::entry_def(),
    assignment::MembraneRoleAssignment::entry_def(),
    membrane_roles::MembraneRole::entry_def()
];

#[hdk_extern]
pub fn who_am_i(_: ()) -> ExternResult<AgentPubKeyB64> {
    let agent_info = agent_info()?;

    Ok(AgentPubKeyB64::from(agent_info.agent_initial_pubkey))
}

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    if am_i_progenitor()? {
        let membrane_role = admin_role()?;
        let hash = hash_entry(&membrane_role)?;

        if let None = get(hash.clone(), GetOptions::default())? {
            create_membrane_role(CreateMembraneRoleInput {
                role_name: ADMIN_ROLE_NAME.into(),
            })?;
        }
    }

    Ok(InitCallbackResult::Pass)
}
/* 
#[hdk_extern]
fn validate(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    // Only progenitors or admins can modify state in this zome
    let author = data.element.header().author();

    validate_can_modify_roles(author.clone())
}

fn validate_can_modify_roles(agent_pub_key: AgentPubKey) -> ExternResult<ValidateCallbackResult> {
    if is_progenitor(WrappedAgentPubKey(agent_pub_key.clone()))? {
        return Ok(ValidateCallbackResult::Valid);
    }

    let admin_assignment = MembraneRoleAssignment {
        role_name: ADMIN_ROLE_NAME.into(),
        agent_pub_key: agent_pub_key.clone(),
        dna_hash: WrappedDnaHash(zome_info()?.dna_hash),
    };

    let hash = hash_entry(&admin_assignment)?;

    match get(hash.clone(), GetOptions)? {
        None => Ok(ValidateCallbackResult::UnresolvedDependencies(vec![
            hash.into()
        ])),
        Some(_) => Ok(ValidateCallbackResult::Valid),
    }
}
 */