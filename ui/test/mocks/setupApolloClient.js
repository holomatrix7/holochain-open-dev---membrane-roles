import { gql, ApolloClient, InMemoryCache } from '@apollo/client/core';
import { SchemaLink } from '@apollo/client/link/schema';
import { makeExecutableSchema } from '@graphql-tools/schema';
import ConductorApi from '@holochain/conductor-api';

import { membraneRolesResolvers, membraneRolesTypeDefs } from '../../dist';
import {
  profilesTypeDefs,
  profilesResolvers,
} from '@holochain-open-dev/profiles';
import { AppWebsocketMock, DnaMock } from 'holochain-ui-test-utils';
import { MembraneRolesMock } from './membrane_roles.mock';

const rootTypeDef = gql`
  type Query {
    _: Boolean
  }

  type Mutation {
    _: Boolean
  }
`;

// TODO: add your own typeDefs to rootTypeDef
const allTypeDefs = [rootTypeDef, profilesTypeDefs, membraneRolesTypeDefs];

const dnaMock = new DnaMock({ membrane_roles: new MembraneRolesMock() });
async function getAppWebsocket() {
  if (process.env.CONDUCTOR_URL)
    return ConductorApi.AppWebsocket.connect(process.env.CONDUCTOR_URL);
  else {
    return new AppWebsocketMock([dnaMock]);
  }
}

/**
 * If process.env.CONDUCTOR_URL is undefined, it will mock the backend
 * If process.env.CONDUCTOR_URL is defined, it will try to connect to holochain at that URL
 */
export async function setupApolloClient() {
  const appWebsocket = await getAppWebsocket();

  const appInfo = await appWebsocket.appInfo({ app_id: 'test-app' });

  const cellId = appInfo.cell_data[0][0];

  const executableSchema = makeExecutableSchema({
    typeDefs: allTypeDefs,
    resolvers: [
      profilesResolvers(appWebsocket, cellId),
      membraneRolesResolvers(appWebsocket, cellId),
    ],
  });

  const schemaLink = new SchemaLink({ schema: executableSchema });

  return new ApolloClient({
    typeDefs: allTypeDefs,

    cache: new InMemoryCache(),
    link: schemaLink,
  });
}
