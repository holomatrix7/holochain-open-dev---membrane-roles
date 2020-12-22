import { AppWebsocket, CellId } from '@holochain/conductor-api';
import { Hashed } from '@holochain-open-dev/common';
import { MembraneRole } from './types';

export class MembraneRolesService {
  constructor(
    public appWebsocket: AppWebsocket,
    public cellId: CellId,
    public zomeName = 'membrane_roles'
  ) {}
  private callZome(fnName: string, payload: any) {
    return this.appWebsocket.callZome({
      cap: null as any,
      cell_id: this.cellId,
      zome_name: this.zomeName,
      fn_name: fnName,
      payload: payload,
      provenance: this.cellId[1],
    });
  }

  public async getRolesForAgent(
    agentPubKey: string
  ): Promise<Array<Hashed<MembraneRole>>> {
    const roles = await this.callZome(
      'get_membrane_roles_for_agent',
      agentPubKey
    );
    return roles.map((r: any) => ({ hash: r.entry_hash, content: r.entry }));
  }

  public getMembraneRoleAssignees(
    membraneRoleHash: string
  ): Promise<string> {
    return this.callZome('get_membrane_role_assignees', membraneRoleHash);
  }

  public async getAllRoles(): Promise<Array<Hashed<MembraneRole>>> {
    const roles = await this.callZome('get_all_membrane_roles', null);
    return roles.map((r: any) => ({ hash: r.entry_hash, content: r.entry }));
  }

  public createRole(roleName: string): Promise<string> {
    return this.callZome('create_membrane_role', { role_name: roleName });
  }
}
