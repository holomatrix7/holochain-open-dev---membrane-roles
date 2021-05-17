use hdk::prelude::*;

use crate::membrane_roles::MembraneRole;

pub const ADMIN_ROLE_NAME: &str = "administrator";

pub fn admin_role() -> ExternResult<MembraneRole> {
    MembraneRole::new(ADMIN_ROLE_NAME.into())
}

pub fn admin_role_hash() -> ExternResult<EntryHash> {
    let membrane_role = admin_role()?;

    let hash = hash_entry(&membrane_role)?;
    Ok(hash)
}
