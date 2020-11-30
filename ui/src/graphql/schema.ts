import { gql } from '@apollo/client/core';

export const membraneRolesTypeDefs = gql`
  type MembraneRole {
    id: ID!

    name: String!
    description: String!

    membrane: RolesMembrane!

    assignees: [Agent!]!
  }

  interface RolesMembrane implements Membrane {
    id: ID!

    allRoles: [MembraneRole!]!
  }

  extend type Agent {
    roles(membraneId: ID!): [Role!]!
  }

  input RoleParams {
    name: String!
    description: String!
  }

  extend type Mutation {
    createRole(membraneId: ID!, role: RoleParams!): Role!
    assignRole(membraneId: ID!, roleId: ID!, agentId: ID!): Role!
  }
`;
