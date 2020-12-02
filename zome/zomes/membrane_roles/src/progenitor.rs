use hc_utils::WrappedAgentPubKey;
use hdk3::prelude::*;

#[derive(Serialize, SerializedBytes, Deserialize, Clone, Debug)]
pub struct ProgenitorsProperties {
    progenitors: Vec<WrappedAgentPubKey>,
}

pub fn get_progenitors() -> ExternResult<Vec<WrappedAgentPubKey>> {
    let progenitors: ProgenitorsProperties = zome_info()?.properties.try_into()?;

    Ok(progenitors.progenitors)
}

pub fn am_i_progenitor() -> ExternResult<bool> {
    let my_pub_key = WrappedAgentPubKey(agent_info()?.agent_initial_pubkey);
    is_progenitor(my_pub_key)
}

pub fn is_progenitor(agent_pub_key: WrappedAgentPubKey) -> ExternResult<bool> {
    let progenitors = get_progenitors()?;
    Ok(progenitors.contains(&agent_pub_key))
}
