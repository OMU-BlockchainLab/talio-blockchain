// Import the API & Provider and some utility functions
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

// import the test keyring (already has dev keys for Alice, Bob, Charlie, Eve & Ferdie)
//const testKeyring = require('@polkadot/keyring/testing');

const fs = require('fs');

async function main () {
  // Initialise the provider to connect to the local node
  const provider = new WsProvider('wss://smartcv.org/smartcv-node');

  // Create the API and wait until ready (optional provider passed through)
  const api = await ApiPromise.create({ provider });

  const keyring = new Keyring({ type: 'sr25519' });
  // create Charlie based on the development seed
  const alice = keyring.addFromUri('//Alice');
  // Retrieve the runtime to upgrade
  const code = fs.readFileSync('../../target/release/wbuild/scv-node/scv_node.compact.wasm').toString('hex');
  const proposal = api.tx.system && api.tx.system.setCode
    ? api.tx.system.setCode(`0x${code}`) // For newer versions of Substrate
    : api.tx.consensus.setCode(`0x${code}`); // For previous versions

  console.log(`Upgrading from ${alice}, ${code.length / 2} bytes`);

  // Perform the actual chain upgrade via the sudo module
  api.tx.sudo
    .sudoUncheckedWeight(proposal,0)
    .signAndSend(alice, ({ events = [], status }) => {
      console.log('Proposal status:', status.type);

      if (status.isInBlock) {
        console.error('You have just upgraded your chain');

        console.log('Included at block hash', status.asInBlock.toHex());
        console.log('Events:');

        console.log(JSON.stringify(events.toHuman(), null, 2));
      } else if (status.isFinalized) {
        console.log('Finalized block hash', status.asFinalized.toHex());

        process.exit(0);
      }
    });
}

main().catch((error) => {
  console.error(error);
  process.exit(-1);
});