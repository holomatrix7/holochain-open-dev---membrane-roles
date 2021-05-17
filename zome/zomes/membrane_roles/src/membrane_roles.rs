use crate::utils;
use holo_hash::DnaHashB64;
use hdk::prelude::*;

#[hdk_entry(id = "membrane_role")]
#[derive(Clone)]
pub struct MembraneRole {
    pub role_name: String,
    pub dna_hash: DnaHashB64,
}

impl MembraneRole {
    pub fn new(role_name: String) -> ExternResult<Self> {
        let dna_hash = DnaHashB64::from(zome_info()?.dna_hash);
        Ok(MembraneRole {
            dna_hash,
            role_name,
        })
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct MembraneRoleOutput {
    pub entry_hash: EntryHash,
    pub entry: MembraneRole,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreateMembraneRoleInput {
    pub role_name: String,
}
#[hdk_extern]
pub fn create_membrane_role(input: CreateMembraneRoleInput) -> ExternResult<MembraneRoleOutput> {
    let membrane_role = MembraneRole::new(input.role_name.clone())?;

    create_entry(&membrane_role)?;

    let entry_hash = hash_entry(&membrane_role)?;

    let path = all_roles_path();

    path.ensure()?;

    create_link(
        path.hash()?,
        entry_hash.clone(),
        utils::link_tag(input.role_name.as_str())?,
    )?;

    let output = MembraneRoleOutput {
        entry_hash,
        entry: membrane_role,
    };
    Ok(output)
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct GetRolesOutput(pub Vec<MembraneRoleOutput>);
#[hdk_extern]
pub fn get_all_membrane_roles(_: ()) -> ExternResult<GetRolesOutput> {
    let all_roles_path = all_roles_path();
    let dna_hash = DnaHashB64::from(zome_info()?.dna_hash);

    let links = get_links(all_roles_path.hash()?, None)?;

    let all_roles: Vec<MembraneRoleOutput> = links
        .into_inner()
        .into_iter()
        .map(|link| {
            let role = MembraneRole {
                role_name: utils::tag_to_string(link.tag)?,
                dna_hash: dna_hash.clone(),
            };

            Ok(MembraneRoleOutput {
                entry_hash: link.target,
                entry: role,
            })
        })
        .collect::<ExternResult<Vec<MembraneRoleOutput>>>()?;

    Ok(GetRolesOutput(all_roles))
}

/** Helper functions */

fn all_roles_path() -> Path {
    Path::from("all_roles")
}
