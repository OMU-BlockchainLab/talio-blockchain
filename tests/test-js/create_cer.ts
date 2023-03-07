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
    ///Only system call
    ///api.tx.certificate.createCertificate
    /// params:
    /// @cid: certificate id
    /// + cid: string
    /// @owner: who create this certificate (RoleId)
    /// + owner: string
    /// @org: who issue this certificate (RoleId)
    /// + org: Option<string>
    /// @expired_date: Expiration date 
    /// + expired_date: Option<u64>
//---------------------------------------------//
    interface EventReturn {
      result: string;
      module: string;
      value: string;
    }
    const events = new Promise(async (resolve, _) => {
        await api.tx.certificate.createCertificate('4b787009-d826-4f66-9e67-2ff355cabfd9',null,null)
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
                    const {data,method, section} = event;
                    //console.log(`: ${section}.${method}`, data.toString());
                    if (section == 'certificate'){
                        let res: EventReturn =  {result:"Success", module:"".concat(section,".",method), value: data.toHuman()[0]};
                        //const res = 'Success'.concat(':',section,'.',method,':',data);
                        resolve(res)
                    }

                  });
            }
        });
  })
  console.log(await events);


    
}



main();