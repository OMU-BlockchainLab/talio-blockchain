const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');



const main = async() => {
    const provider = new WsProvider('ws://13.208.115.196:9947');
    const api = await ApiPromise.create({ provider });
    console.log('Connected to blockchain');

    const keyring = new Keyring({ type: 'sr25519' });

    // create alice based on the development seed
    const alice = keyring.addFromUri('//Alice');

    // create Charlie based on the development seed
    const charlie = keyring.addFromUri('//Charlie');

    // create Eve based on the development seed
    const eve = keyring.addFromUri('//Eve');

    // create Ferdie based on the development seed
    const ferdie = keyring.addFromUri('//Ferdie');

    // create Bob based on the development seed
    const bob = keyring.addFromUri('//Bob');

    // create Dave based on the development seed
    const dave = keyring.addFromUri('//Dave');
    // Charlie đăng ký account trên hệ thống với vai trò Organization

    const Role = {
        Organization:0,
        SysMan:1,
        User:2,

    }

    // console.log(api);
    // const nameTokenString = JSON.stringify(await api.rpc.system.properties());
    // //const nameToken = JSON.parse(nameTokenString).tokenSymbol
    // console.log(nameTokenString)

    // const { data: { free } } = await api.query.system.account(alice.address);

    // console.log('Alice has a current balance of', free.toHuman());
//---------------------------------------------//
    ///api.tx.account.registerAccount
    /// params:
    /// + id: string
    /// + accountId: enum Role {Organization, SysMan, User}
//---------------------------------------------//


    // const unsub = await api.tx.account
    // .registerAccount("123",Role.User)
    // .signAndSend(charlie, (result) => {
    //     console.log(`Current status is ${result.status}`);

    // if (result.status.isFinalized) {
    //     console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
    //     console.log("Create account successfully");
    //     unsub();
    //     }
    // });


    // const unsub = await api.tx.account
    // .registerAccount("1234",Role.Organization)
    // .signAndSend(eve, (result) => {
    //     console.log(`Current status is ${result.status}`);

    // if (result.status.isFinalized) {
    //     console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
    //     console.log("Create account successfully");
    //     unsub();
    //     }
    // });

    // const unsub = await api.tx.account
    // .registerAccount("12345678",Role.Organization)
    // .signAndSend(ferdie, (result) => {
    //     console.log(`Current status is ${result.status}`);

    // if (result.status.isFinalized) {
    //     console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
    //     console.log("Create account successfully");
    //     unsub();
    //     }
    // });

    // try {
    //     await api.tx.account
    //     .registerAccount("12345678910",Role.SysMan,dave.address )
    //     .signAndSend(alice, (result) => {
    //         console.log(`Current status is ${result.status}`);
    
    //     if (result.status.isFinalized) {
    //         console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
    //         console.log("Create account successfully");
        
    //         }
    //     });
    // }
    // catch(e) {
    //     console.log(e);
    // }


    // const unsub = await api.tx.sudo.sudo(
    //     api.tx.account
    //       .registerAccount(
    //         '12345678910',
    //         Role.Organization,
    //       ))
    //       .signAndSend(alice, (result) => {
    //         console.log(`Current status is ${result.status}`)
  
    //         if (result.status.isFinalized) {
    //           console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`)
    //           console.log('Create account successfully')
    //         }
    //       });
//     const unsub = await api.tx.sudo.sudo(
//     api.tx.account
//         .testAccount(
//         1,
//         )
//         .signAndSend(alice, (result) => {
//         console.log(`Current status is ${result.status}`)

//         if (result.status.isFinalized) {
//             console.log(
//             `Transaction finalized at blockHash ${result.status.asFinalized}`,
//             )
//             console.log('Create account successfully')
//         }
//         }),
// )

//   const unsub4 = await api.tx.sudo
//   .sudo(
//     api.tx.account.registerAccount('1',Role.Organization, dave.address )
//   )
//   .signAndSend(alice, (result) => {
//     if (result.status.isFinalized) {
//       console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
//       console.log("Sudo key reward this campaign");
//       unsub4();
//     }

//   });

//---------------------------------------------//
    ///api.tx.account.updateAccount
    /// params:
    /// + role: enum Role {Organization, SysMan, User}
    /// + who: accountId
//---------------------------------------------//


    // const unsub = await api.tx.account
    // .updateAccount(Role.SysMan, charlie)
    // .signAndSend(charlie, (result) => {
    //     console.log(`Current status is ${result.status}`);

    // if (result.status.isFinalized) {
    //     console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
    //     console.log("Update account actor successfully");
    //     unsub();
    //     }
    // });
  

    
  

}



main();