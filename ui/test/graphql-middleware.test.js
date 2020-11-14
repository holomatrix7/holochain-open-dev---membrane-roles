import { gql } from '@apollo/client/core';
import { expect } from '@open-wc/testing';

import { setupApolloClient } from './mocks/setupApolloClient';
import { ASSIGN_ROLE } from '../dist';
import { CREATE_PROFILE } from '@holochain-open-dev/profiles';

describe('Apollo middleware', () => {
  it('assign a role and retrieve it', async () => {
    const client = await setupApolloClient();

    const profile = await client.mutate({
      mutation: CREATE_PROFILE,
      variables: {
        profile: { username: 'alice' },
      },
    });

    await client.mutate({
      mutation: ASSIGN_ROLE,
      variables: {
        roleName: 'editor',
        agentId: profile.data.createProfile.id,
      },
    });

    const result = await client.query({
      query: gql`
        {
          allRoles {
            name
            assignees {
              id
            }
          }
        }
      `,
    });

    expect(result.data.allRoles.length).to.equal(1);
    expect(result.data.allRoles[0].name).to.equal('editor');
    expect(result.data.allRoles[0].assignees.length).to.equal(1);
  });
});
