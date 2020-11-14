import { randomEntryHash } from 'holochain-ui-test-utils';

// TODO: change the functions of this class to match the functions that your zome has
export class MembraneRolesMock {
  constructor() {
    this.calendarEvents = [];
  }

  create_calendar_event(calendarInput) {
    const newId = randomEntryHash();
    this.calendarEvents.push([
      newId,
      {
        ...calendarInput,
        created_by: randomEntryHash(),
      },
    ]);

    return newId;
  }

  get_all_calendar_events() {
    return this.calendarEvents;
  }
}
