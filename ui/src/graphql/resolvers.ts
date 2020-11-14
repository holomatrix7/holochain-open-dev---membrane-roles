import { Resolvers } from '@apollo/client/core';
import { AppWebsocket, CellId } from '@holochain/conductor-api';

function secondsToTimestamp(secs: number) {
  return [secs, 0];
}

export function membraneRolesResolvers(
  appWebsocket: AppWebsocket,
  cellId: CellId,
  zomeName = 'membrane_roles'
): Resolvers {
  function callZome(fnName: string, payload: any) {
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
    Agent: {
      async roles(agent) {
        const roles = await callZome('get_agent_roles', agent.id);

        return roles.map((role: string) => ({ name: role }));
      },
    },
    Role: {
      async assignees(role) {
        const agents = await callZome(
          'get_assigned_agents_for_role',
          role.name
        );

        return agents.map((agent: string) => ({ id: agent }));
      },
    },
    Query: {
      async allRoles() {
        const roles = await callZome('get_all_roles', null);

        return roles.map((role: string) => ({
          name: role,
        }));
      },
    },
    Mutation: {
      async assignRole(_, { roleName, agentId }) {
        await callZome('assign_role', {
          role: roleName,
          agent_pub_key: agentId,
        });

        return {
          name: roleName,
        };
      },
    },
  };
}
