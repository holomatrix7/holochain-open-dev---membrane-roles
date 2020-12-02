import {
  Orchestrator,
  Config,
  InstallAgentsHapps,
  TransportConfigType,
  Player,
} from "@holochain/tryorama";
import path from "path";

const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const rolesDna = path.join(__dirname, "../../membrane_roles.dna.gz");

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [[[rolesDna]], [[rolesDna]]];

const sleep = (ms) => new Promise((resolve) => setTimeout(() => resolve(), ms));

const orchestrator = new Orchestrator();

orchestrator.registerScenario(
  "create a role and assign an agent",
  async (s, t) => {
    const [player]: Player[] = await s.players([conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_happ], [bob_happ]] = await player.installAgentsHapps(
      installation
    );

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
    t.equal(roles.length, 1);
    t.equal(roles[0].entry.role_name, "editor");

    let agents = await bob_roles.call(
      "membrane_roles",
      "get_membrane_role_assignees",
      roles[0].entry_hash
    );
    t.equal(agents.length, 1);
    t.equal(agents[0], aliceAddress);

    roles = await bob_roles.call(
      "membrane_roles",
      "get_agent_membrane_roles",
      aliceAddress
    );
    t.equal(roles.length, 1);
    t.equal(roles[0].entry.role_name, "editor");
  }
);

orchestrator.run();
