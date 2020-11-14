import { Orchestrator, Config } from "@holochain/tryorama";

const sleep = (ms) => new Promise((resolve) => setTimeout(() => resolve(), ms));

const orchestrator = new Orchestrator();

export const simpleConfig = {
  alice: Config.dna("../membrane_roles.dna.gz", null),
  bobbo: Config.dna("../membrane_roles.dna.gz", null),
};

orchestrator.registerScenario(
  "create and get a calendar event",
  async (s, t) => {
    const { conductor } = await s.players({
      conductor: Config.gen(simpleConfig),
    });
    await conductor.spawn();

    let aliceAddress = await conductor.call(
      "alice",
      "membrane_roles",
      "who_am_i",
      null
    );

    await conductor.call("bobbo", "membrane_roles", "assign_role", {
      role: "editor",
      agent_pub_key: aliceAddress,
    });

    await sleep(10);

    let roles = await conductor.call(
      "bobbo",
      "membrane_roles",
      "get_all_roles",
      null
    );
    t.equal(roles.length, 1);
    t.equal(roles[0], "editor");

    let agents = await conductor.call(
      "bobbo",
      "membrane_roles",
      "get_assigned_agents_for_role",
      "editor"
    );
    t.equal(agents.length, 1);
    t.equal(agents[0], aliceAddress);

    roles = await conductor.call(
      "bobbo",
      "membrane_roles",
      "get_agent_roles",
      aliceAddress
    );
    t.equal(roles.length, 1);
    t.equal(roles[0], "editor");
  }
);

orchestrator.run();
