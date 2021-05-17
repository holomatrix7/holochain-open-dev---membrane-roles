use crate::{
    admin::admin_role_hash,
    membrane_roles::{GetRolesOutput, MembraneRole, MembraneRoleOutput},
    progenitor::get_progenitors,
    utils,
};
use holo_hash::{AgentPubKeyB64, DnaHashB64};
use hdk::prelude::*;

#[hdk_entry(id = "membrane_role_assigment")]
pub struct MembraneRoleAssignment {
    pub role_name: String,
    pub dna_hash: DnaHashB64,
    pub agent_pub_key: AgentPubKey,
}

/** Roles **/

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct AssignRoleInput {
    role_name: String,
    agent_pub_key: AgentPubKeyB64,
}
#[hdk_extern]
pub fn assign_membrane_role(input: AssignRoleInput) -> ExternResult<()> {
    let membrane_role = MembraneRole::new(input.role_name.clone())?;

    let membrane_role_hash = hash_entry(&membrane_role)?;

    let role_assignment = MembraneRoleAssignment {
        role_name: input.role_name,
        dna_hash: DnaHashB64::from(zome_info()?.dna_hash),
        agent_pub_key: AgentPubKey::from(input.agent_pub_key.clone()),
    };

    create_entry(&role_assignment)?;

    let assignment_hash = hash_entry(&role_assignment)?;

    create_link(
        membrane_role_hash,
        assignment_hash.clone(),
        utils::link_tag("assignee")?,
    )?;
    create_link(
        utils::pub_key_to_entry_hash(input.agent_pub_key.into()),
        assignment_hash,
        utils::link_tag("has_role")?,
    )?;

    Ok(())
}

#[hdk_extern]
pub fn get_membrane_role_assignees(
    membrane_role_hash: EntryHash,
) -> ExternResult<Vec<AgentPubKeyB64>> {
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

            Ok(AgentPubKeyB64::from(assignment.agent_pub_key))
        })
        .collect::<ExternResult<Vec<AgentPubKeyB64>>>()?;

    // Add progenitors if the queried role is the admin role
    if membrane_role_hash == admin_role_hash()? {
        assigned_agents.extend(get_progenitors()?);
    }

    Ok(assigned_agents)
}

#[hdk_extern]
pub fn get_membrane_roles_for_agent(agent_pub_key: AgentPubKeyB64) -> ExternResult<GetRolesOutput> {
    let agent_address = utils::pub_key_to_entry_hash(agent_pub_key.into());

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
