import { gql } from '@apollo/client/core';

export const ASSIGN_ROLE = gql`
  mutation AssignRole($roleName: String!, $agentId: ID!) {
    assignRole(roleName: $roleName, agentId: $agentId) {
      name
    }
  }
`;
