import { Orchestrator, Config, Player } from "@holochain/tryorama";
import path from "path";
import * as msgpack from "@msgpack/msgpack";
import { Base64 } from "js-base64";

export function serializeHash(hash) {
  return `u${Base64.fromUint8Array(hash, true)}`;
}

const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const rolesDna = path.join(__dirname, "../../membrane_roles.dna.gz");

const sleep = (ms) => new Promise((resolve) => setTimeout(() => resolve(null), ms));

const orchestrator = new Orchestrator();

orchestrator.registerScenario(
  "create a role and assign an agent",
  async (s, t) => {
    const [player]: Player[] = await s.players([conductorConfig]);

    const aliceKey = await player.adminWs().generateAgentPubKey();

    const dnas = [
      {
        path: rolesDna,
        nick: `my_cell_nick`,
        properties: { progenitors: [serializeHash(aliceKey)] },
        membrane_proof: undefined,
      },
    ];

    const alice_happ = await player._installHapp({
      installed_app_id: `my_app:12345`, // my_app with some unique installed id value
      agent_key: aliceKey,
      dnas,
    });
    const bob_happ = await player._installHapp({
      installed_app_id: `my_app:1234`, // my_app with some unique installed id value
      agent_key: await player.adminWs().generateAgentPubKey(),
      dnas,
    });

    const alice_roles = alice_happ.cells[0];
    const bob_roles = bob_happ.cells[0];

    let aliceAddress = await alice_roles.call(
      "membrane_roles",
      "who_am_i",
      null
    );

    await bob_roles.call("membrane_roles", "create_membrane_role", {
      role_name: "editor",
    });

    await bob_roles.call("membrane_roles", "assign_membrane_role", {
      role_name: "editor",
      agent_pub_key: aliceAddress,
    });

    await sleep(10);

    let roles = await bob_roles.call(
      "membrane_roles",
      "get_all_membrane_roles",
      null
    );
    t.equal(roles.length, 2);
    t.equal(roles[0].entry.role_name, "editor");
    t.equal(roles[1].entry.role_name, "administrator");

    let agents = await bob_roles.call(
      "membrane_roles",
      "get_membrane_role_assignees",
      roles[0].entry_hash
    );
    t.equal(agents.length, 1);
    t.equal(agents[0], aliceAddress);

    roles = await bob_roles.call(
      "membrane_roles",
      "get_membrane_roles_for_agent",
      aliceAddress
    );
    t.equal(roles.length, 1);
    t.equal(roles[0].entry.role_name, "editor");
  }
);

orchestrator.run();
