use crate::{
    admin::admin_role_hash,
    membrane_roles::{GetRolesOutput, MembraneRole, MembraneRoleOutput},
    progenitor::get_progenitors,
    utils,
};
use hc_utils::{WrappedAgentPubKey, WrappedDnaHash};
use hdk3::prelude::*;

#[hdk_entry(id = "membrane_role_assigment")]
pub struct MembraneRoleAssignment {
    pub role_name: String,
    pub dna_hash: WrappedDnaHash,
    pub agent_pub_key: AgentPubKey,
}

/** Roles **/

#[derive(Serialize, SerializedBytes, Deserialize, Clone)]
pub struct AssignRoleInput {
    role_name: String,
    agent_pub_key: WrappedAgentPubKey,
}
#[hdk_extern]
pub fn assign_membrane_role(input: AssignRoleInput) -> ExternResult<()> {
    let membrane_role = MembraneRole::new(input.role_name.clone())?;

    let membrane_role_hash = hash_entry(&membrane_role)?;

    let role_assignment = MembraneRoleAssignment {
        role_name: input.role_name,
        dna_hash: WrappedDnaHash(zome_info()?.dna_hash),
        agent_pub_key: input.agent_pub_key.0.clone(),
    };

    create_entry(&role_assignment)?;

    let assignment_hash = hash_entry(&role_assignment)?;

    create_link(
        membrane_role_hash,
        assignment_hash.clone(),
        utils::link_tag("assignee")?,
    )?;
    create_link(
        utils::pub_key_to_entry_hash(input.agent_pub_key.0),
        assignment_hash,
        utils::link_tag("has_role")?,
    )?;

    Ok(())
}

#[derive(Serialize, SerializedBytes, Deserialize, Clone)]
pub struct GetAssigneesOutput(Vec<WrappedAgentPubKey>);
#[hdk_extern]
pub fn get_membrane_role_assignees(
    membrane_role_hash: EntryHash,
) -> ExternResult<GetAssigneesOutput> {
    let links = get_links(
        membrane_role_hash.clone(),
        Some(utils::link_tag("assignee")?),
    )?;

    let mut assigned_agents = links
        .into_inner()
        .into_iter()
        .map(|link| {
            let assignment: MembraneRoleAssignment =
                utils::try_get_and_convert(link.target.clone())?;

            Ok(WrappedAgentPubKey(assignment.agent_pub_key))
        })
        .collect::<ExternResult<Vec<WrappedAgentPubKey>>>()?;

    // Add progenitors if the queried role is the admin role
    if membrane_role_hash == admin_role_hash()? {
        assigned_agents.extend(get_progenitors()?);
    }

    Ok(GetAssigneesOutput(assigned_agents))
}

#[hdk_extern]
pub fn get_agent_membrane_roles(agent_pub_key: WrappedAgentPubKey) -> ExternResult<GetRolesOutput> {
    let agent_address = utils::pub_key_to_entry_hash(agent_pub_key.0);

    let links = get_links(agent_address, Some(utils::link_tag("has_role")?))?;

    let agent_roles = links_to_membrane_role_output(links)?;
    Ok(GetRolesOutput(agent_roles))
}

fn links_to_membrane_role_output(links: Links) -> ExternResult<Vec<MembraneRoleOutput>> {
    links
        .into_inner()
        .into_iter()
        .map(|link| {
            let assignment: MembraneRoleAssignment =
                utils::try_get_and_convert(link.target.clone())?;

            let membrane_role = MembraneRole {
                role_name: assignment.role_name,
                dna_hash: assignment.dna_hash,
            };

            Ok(MembraneRoleOutput {
                entry_hash: link.target,
                entry: membrane_role,
            })
        })
        .collect::<ExternResult<Vec<MembraneRoleOutput>>>()
}
