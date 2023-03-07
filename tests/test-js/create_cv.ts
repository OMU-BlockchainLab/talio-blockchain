const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');



const main = async() => {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider });
    console.log('Connected to blockchain');

    const keyring = new Keyring({ type: 'sr25519' });

    // create alice based on the development seed
    const alice = keyring.addFromUri('//Alice');

    // // create Charlie based on the development seed
    // const charlie = keyring.addFromUri('//Charlie');

    // // create Eve based on the development seed
    // const eve = keyring.addFromUri('//Eve');

    // // create Ferdie based on the development seed
    // const ferdie = keyring.addFromUri('//Ferdie');

    // // create Bob based on the development seed
    // const bob = keyring.addFromUri('//Bob');

    // // create Dave based on the development seed
    // const dave = keyring.addFromUri('//Dave');
    // // Charlie đăng ký account trên hệ thống với vai trò Organization

    // const Role = {
    //     Organization:0,
    //     SysMan:1,
    //     User:2,

    // }
//---------------------------------------------//
    ///api.tx.cv.createItem
    /// params:
    /// @cvId: CV id
    /// + cid: string
    /// @owner: who create this certificate (RoleId)
    /// + owner: string
    /// @certificateId: multiple certificates that user created
    /// + certificatedId: Vec<string>
//---------------------------------------------//

    const events = new Promise(async (resolve, _) => {
        await api.tx.cv.createItem('10','aa11b832-b58f-4c37-a7b3-e6e903ed2293',['3123213'])
        .signAndSend(alice, ({ events, dispatchError }: any) => {

            if (dispatchError) {
                if (dispatchError.isModule) {
                  // for module errors, we have the section indexed, lookup
                  const decoded = api.registry.findMetaError(dispatchError.asModule);
                  const { name, section } = decoded;
                  const res = section.concat('.',name);
                  //console.log(`${section}.${name}: ${docs.join(' ')}`);
                  resolve(res)
                } else {
                  // Other, CannotLookup, BadOrigin, no extra info
                  //console.log(dispatchError.toString());
                  resolve(dispatchError.toString())
                }
              }
            else {
                events.forEach(({event}: any) => {
                    const {method, section} = event;
                    //console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
                    if (section == 'certificate'){
                        const res = 'Success'.concat(':',section,'.',method);
                        resolve(res)
                    }

                  });
            }
        });
  })
  console.log(await events);


    
}



main();