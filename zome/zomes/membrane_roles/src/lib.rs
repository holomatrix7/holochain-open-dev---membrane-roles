use hc_utils::WrappedAgentPubKey;
use hdk3::hash_path::path::Component;
use hdk3::prelude::*;

mod utils;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

entry_defs![Path::entry_def()];

#[hdk_extern]
pub fn who_am_i(_: ()) -> ExternResult<WrappedAgentPubKey> {
    let agent_info = agent_info!()?;

    Ok(WrappedAgentPubKey(agent_info.agent_initial_pubkey))
}

/** Roles **/

#[derive(Serialize, SerializedBytes, Deserialize, Clone)]
pub struct Role(String);

#[derive(Serialize, SerializedBytes, Deserialize, Clone)]
pub struct AssignRoleInput {
    role: Role,
    agent_pub_key: WrappedAgentPubKey,
}
#[hdk_extern]
pub fn assign_role(input: AssignRoleInput) -> ExternResult<()> {
    let role_path = get_role_path(input.role.0);

    role_path.ensure()?;

    create_link!(
        role_path.hash()?,
        pub_key_to_entry_hash(input.agent_pub_key.0.clone()),
        utils::link_tag("assignee")?
    )?;
    create_link!(
        pub_key_to_entry_hash(input.agent_pub_key.0),
        role_path.hash()?,
        utils::link_tag("has_role")?
    )?;

    Ok(())
}

#[derive(Serialize, SerializedBytes, Deserialize, Clone)]
pub struct GetAssignedAgentsOutput(Vec<WrappedAgentPubKey>);
#[hdk_extern]
pub fn get_assigned_agents_for_role(role: Role) -> ExternResult<GetAssignedAgentsOutput> {
    let path = get_role_path(role.0);

    let links = get_links!(path.hash()?, utils::link_tag("assignee")?)?;

    let assigned_agents = links
        .into_inner()
        .into_iter()
        .map(|link| WrappedAgentPubKey(entry_hash_to_pub_key(link.target)))
        .collect();

    Ok(GetAssignedAgentsOutput(assigned_agents))
}

#[derive(Serialize, SerializedBytes, Deserialize, Clone)]
pub struct GetRolesOutput(Vec<Role>);
#[hdk_extern]
pub fn get_all_roles(_: ()) -> ExternResult<GetRolesOutput> {
    let all_roles_path = Path::from("all_roles");

    let children_links = all_roles_path.children()?;

    let all_roles: Vec<Role> = children_links
        .into_inner()
        .into_iter()
        .map(|child_link| {
            let path: Path = utils::try_get_and_convert(child_link.target)?;

            let components: Vec<Component> = path.into();

            let role: String = components.last().unwrap().try_into()?;

            Ok(Role(role))
        })
        .collect::<ExternResult<Vec<Role>>>()?;

    Ok(GetRolesOutput(all_roles))
}

#[hdk_extern]
pub fn get_agent_roles(agent_pub_key: WrappedAgentPubKey) -> ExternResult<GetRolesOutput> {
    let agent_address = pub_key_to_entry_hash(agent_pub_key.0);

    let links = get_links!(agent_address, utils::link_tag("has_role")?)?;

    let agent_roles = links
        .into_inner()
        .into_iter()
        .map(|link| {
            let path: Path = utils::try_get_and_convert(link.target)?;

            let components: Vec<Component> = path.into();

            let role: String = components.last().unwrap().try_into()?;

            Ok(Role(role))
        })
        .collect::<ExternResult<Vec<Role>>>()?;

    Ok(GetRolesOutput(agent_roles))
}

/*

unassign_role(role, agent_pub_key)


 */
/** Helper functions */

fn get_role_path(role_name: String) -> Path {
    Path::from(format!("all_roles.{}", role_name))
}

fn entry_hash_to_pub_key(entry_hash: EntryHash) -> AgentPubKey {
    let bytes = entry_hash.into_inner();
    AgentPubKey::from_raw_bytes(bytes)
}

fn pub_key_to_entry_hash(agent_pub_key: AgentPubKey) -> EntryHash {
    let agent_address: AnyDhtHash = agent_pub_key.into();
    agent_address.into()
}
