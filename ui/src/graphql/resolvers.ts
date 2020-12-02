import { Resolvers } from '@apollo/client/core';
import { AppWebsocket, InstalledAppId } from '@holochain/conductor-api';
import { getCellIdForDnaHash } from '@holochain-open-dev/common';

export function membraneRolesResolvers(
  appWebsocket: AppWebsocket,
  installedAppId: InstalledAppId,
  zomeName = 'membrane_roles'
): Resolvers {
  async function callZome(membraneId: string, fnName: string, payload: any) {
    const cellId = await getCellIdForDnaHash(
      appWebsocket,
      installedAppId,
      membraneId
    );
    return appWebsocket.callZome({
      cap: null as any,
      cell_id: cellId,
      zome_name: zomeName,
      fn_name: fnName,
      payload: payload,
      provenance: cellId[1],
    });
  }
  return {
    HolochainAgent: {
      async membraneRoles(agent, { membraneId }) {
        const roles = await callZome(membraneId, 'get_agent_roles', agent.id);

        return roles.map((role: string) => ({ name: role }));
      },
    },
    MembraneRole: {
      async assignees(membraneRole) {
        const agents = await callZome(
          membraneRole.membrane.id,
          'get_assigned_agents_for_role',
          membraneRole.id
        );

        return agents.map((agent: string) => ({ id: agent }));
      },
      membrane(membraneRole) {
        return {
          id: membraneRole.dna_hash,
        };
      },
    },
    RolesMembrane: {
      async allMembraneRoles(membrane) {
        const roles = await callZome(membrane.id, 'get_all_roles', null);

        return roles.map((role: string) => ({
          name: role,
        }));
      },
    },
    Mutation: {
      async assignMembraneRole(_, { membraneId, membraneRoleId, agentId }) {
        await callZome(membraneId, 'assign_role', {
          role: membraneRoleId,
          agent_pub_key: agentId,
        });

        return true;
      },
      async createMembraneRole(_, { membraneId, role }) {
        const membraneRole = await callZome(membraneId, 'create_role', role);

        return {
          id: membraneRole.entry_hash,
          ...membraneRole.entry,
        };
      },
    },
  };
}
