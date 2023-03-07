const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');



const main = async() => {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider });
    console.log('Connected to blockchain');

    const keyring = new Keyring({ type: 'sr25519' });
    // create Charlie based on the development seed
    const charlie = keyring.addFromUri('//Charlie');

    const alice = keyring.addFromUri('//Alice');

    // create Eve based on the development seed
    const eve = keyring.addFromUri('//Eve');

    // create Ferdie based on the development seed
    const ferdie = keyring.addFromUri('//Ferdie');

    // create Bob based on the development seed
    const bob = keyring.addFromUri('//Bob');

    // create Dave based on the development seed
    const dave = keyring.addFromUri('//Dave');
    // Charlie đăng ký account trên hệ thống với vai trò Organization

    const PHRASE_TEST_ACCOUNT = 'speak sentence monster because comfort feature puppy area team piece plug field';
    // TEST ACCOUNT
    const TEST_ACCOUNT = keyring.addFromUri(PHRASE_TEST_ACCOUNT);
    const Role = {
        Organization:0,
        SysMan:1,
        User:2,

    }
    
    //Lấy toàn bộ thông tin mà Organization và SysMan chưa được validate

    // const accountKeys = await api.query.account.accountStorage.keys();
    // console.log(`account: ${accountKeys}`);
    // console.log(`type of account: ${typeof accountKeys}`);
    // console.log(` accountID: ${accountKeys[0].toHuman()}`);

    // convert to accountId
    // const accountIds = new Array();
    // for (const key in accountKeys) {

    //   // console.log(`${key}: ${accountKeys[key].toHuman()}`);
    //   // console.log(`type of: ${typeof accountKeys[key].toHuman()}`);
    //   accountIds.push(accountKeys[key].toHuman().toString());
    // }
    // console.log(` accountIDs:${accountIds}`);



    //list all accounts storage


    // const infoAccounts = new Promise(async (resolve, reject) => {
    //   try {
    //     await api.query.account.accountStorage.multi(accountIds, (accountStorage) => {  
    //         resolve(accountStorage);
    //     });

    //   } catch (error) {
    //       reject(error);
    //   }
    // })
    // console.log(`List all account storage:${await infoAccounts}`);

    // console.log(`accounts array:${Object.values(await infoAccounts)}`);
    // let infos = new Array();
    // Object.values(await infoAccounts).map(info => {
    //   infos.push(info.toHuman());
    // })
    // const accounts = {};
    // accountIds.forEach((value,index) =>{
    //     accounts[value] = infos[index];
    // } ) ;

    // console.log(accounts);
    // const listUnValidatedSys = Object.entries(accounts).filter(([key,value]) => value.status == "Pending" && value.role == 'SysMan');

    // console.log(listUnValidatedSys);


//---------------------------------------------//
    ///api.tx.hierarchy.approveOrg
    /// only sysman can execute this transaction
    /// params:
    /// + accountId: address
//---------------------------------------------//

    //Validate pending sysman by hierarchy sysman
    //Initial Sysman: Bob , Charlie
    //let accountId = listUnValidatedSys[0][0];

    const events = new Promise(async (resolve, reject) => {
            await api.tx.hierarchy
            .approveSysman('1', '5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc')
            .signAndSend(bob, ({ status, events, dispatchError }) => {

                if (dispatchError) {
                    if (dispatchError.isModule) {
                      // for module errors, we have the section indexed, lookup
                      const decoded = api.registry.findMetaError(dispatchError.asModule);
                      const { docs, name, section } = decoded;
                      const res = 'Error'.concat(':',section,'.',name);
                      //console.log(`${section}.${name}: ${docs.join(' ')}`);
                      resolve(res)
                    } else {
                      // Other, CannotLookup, BadOrigin, no extra info
                      //console.log(dispatchError.toString());
                      resolve(dispatchError.toString())
                    }
                  }
                else {
                    events.forEach(({event, phase}) => {
                        const {data, method, section} = event;
                        //console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
                        if (section == 'hierarchy'){
                            const res = 'Success'.concat(':',section,'.',method);
                            resolve(res)
                        }

                      });
                }
            });
      })
      console.log(await events);


    // const unsub= await api.tx.hierarchy
    //     .approveSysman('1', '5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc')
    //     .signAndSend(bob, ({ status, events, dispatchError }) => {
    //         if (dispatchError) {
    //             if (dispatchError.isModule) {
    //               // for module errors, we have the section indexed, lookup
    //               const decoded = api.registry.findMetaError(dispatchError.asModule);
    //               const { docs, name, section } = decoded;
          
    //               console.log(`${section}.${name}: ${docs.join(' ')}`);
    //             } else {
    //               // Other, CannotLookup, BadOrigin, no extra info
    //               console.log(dispatchError.toString());
    //             }
    //           }
    //     });

    // try {
    //     await api.tx.balances
    //     .transfer(alice.address, 123)
    //     .signAndSend(bob, (result) => {
    //         console.log(`Current status is ${result.status}`);
    
    //     if (result.status.isFinalized) {
    //         console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
    //         console.log("approve sys  successfully");
    //         }
    //     });
    // }

    // catch (e){
    //     console.log(e);
    // }

    // const unsub2 = await api.query.system.events(events =>{
    //     events.forEach(record => {
    //         const { event, phase } = record
    //         const evHuman = event.toHuman()
    //         console.log(evHuman);
    //         if (api.events.system.ExtrinsicFailed.is(event)){
    //             console.log("Ok");
    //         }
    //     })

    // })



    
  

}



// main().catch(console.error).finally(() => process.exit());

main();