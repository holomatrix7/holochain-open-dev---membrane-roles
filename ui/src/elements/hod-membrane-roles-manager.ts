import { membraneContext } from '@holochain-open-dev/membrane-context';
import {
  Constructor,
  html,
  LitElement,
  property,
  PropertyValues,
} from 'lit-element';
import { ScopedElementsMixin as Scoped } from '@open-wc/scoped-elements';
import { Hashed } from '@holochain-open-dev/common';
import { CircularProgress } from 'scoped-material-components/mwc-circular-progress';
import { List } from 'scoped-material-components/mwc-list';
import { ListItem } from 'scoped-material-components/mwc-list-item';

import { sharedStyles } from '../sharedStyles';
import { MembraneRole } from '../types';
import { MembraneRolesService } from '../membrane-roles.service';
import { AppWebsocket, CellId } from '@holochain/conductor-api';

export class HodMembraneRolesManager extends membraneContext(
  Scoped(LitElement) as Constructor<LitElement>
) {
  /** Public attributes */

  /** Private properties */
  @property({ type: Array })
  _allMembraneRoles: Array<Hashed<MembraneRole>> | undefined = undefined;

  updated(changedValues: PropertyValues) {
    super.updated(changedValues);
    if (
      changedValues.has('membraneContext') &&
      this.membraneContext.appWebsocket
    ) {
      this.loadRoles();
    }
  }

  async loadRoles() {
    const service = new MembraneRolesService(
      this.membraneContext.appWebsocket as AppWebsocket,
      this.membraneContext.cellId as CellId
    );
    this._allMembraneRoles = await service.getAllRoles();
  }

  static styles() {
    return sharedStyles;
  }

  static get scopedElements() {
    return {
      'mwc-circular-progress': CircularProgress,
      'mwc-list': List,
      'mwc-list-item': ListItem,
    };
  }

  render() {
    if (!this._allMembraneRoles)
      return html`<mwc-circular-progress></mwc-circular-progress>`;
    return html`
      <mwc-list>
        ${this._allMembraneRoles.map(
          role => html` <mwc-list-item>${role.content.name}</mwc-list-item> `
        )}
      </mwc-list>
    `;
  }
}
