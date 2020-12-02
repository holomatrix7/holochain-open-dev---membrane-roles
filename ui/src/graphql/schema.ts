import { gql } from '@apollo/client/core';

export const membraneRolesTypeDefs = gql`
  type MembraneRole {
    id: ID!

    name: String!
    membrane: RolesMembrane!

    assignees: [Agent!]!
  }

  interface RolesMembrane implements Membrane {
    id: ID!

    allMembraneRoles: [MembraneRole!]!
    membraneRole(membraneRoleId: ID!): MembraneRole!
  }

  extend type HolochainAgent {
    membraneRoles(membraneId: ID!): [MembraneRole!]!
  }

  input MembraneRoleParams {
    name: String!
  }

  extend type Mutation {
    createMembraneRole(membraneId: ID!, membraneRole: MembraneRoleParams!): MembraneRole!
    assignMembraneRole(membraneId: ID!, roleName: String!, agentId: ID!): Boolean
  }
`;
