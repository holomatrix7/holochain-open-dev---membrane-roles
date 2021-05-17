use holo_hash::AgentPubKeyB64;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, SerializedBytes)]
pub struct ProgenitorsProperties {
    progenitors: Vec<AgentPubKeyB64>,
}

pub fn get_progenitors() -> ExternResult<Vec<AgentPubKeyB64>> {
    let progenitors: ProgenitorsProperties = zome_info()?.properties.try_into()?;

    Ok(progenitors.progenitors)
}

pub fn am_i_progenitor() -> ExternResult<bool> {
    let my_pub_key = AgentPubKeyB64::from(agent_info()?.agent_initial_pubkey);
    is_progenitor(my_pub_key)
}

pub fn is_progenitor(agent_pub_key: AgentPubKeyB64) -> ExternResult<bool> {
    let progenitors = get_progenitors()?;
    Ok(progenitors.contains(&agent_pub_key))
}
