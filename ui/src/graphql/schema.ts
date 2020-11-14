import { gql } from '@apollo/client/core';

export const membraneRolesTypeDefs = gql`
  type Role {
    name: String!

    assignees: [Agent!]!
  }

  extend type Agent {
    roles: [Role!]!
  }

  extend type Query {
    allRoles: [Role!]!
  }

  extend type Mutation {
    assignRole(roleName: String!, agentId: ID!): Role!
  }
`;
