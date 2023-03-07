const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');



const main = async() => {
    const provider = new WsProvider('wss://smartcv.org');
    const api = await ApiPromise.create({ provider });
    console.log('Connected to blockchain');

    const keyring = new Keyring({ type: 'sr25519' });
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
    
    //Lấy toàn bộ thông tin mà Organization và SysMan chưa được validate

    const accountKeys = await api.query.account.accountStorage.keys();
    console.log(`account: ${accountKeys}`);
    console.log(`type of account: ${typeof accountKeys}`);
    console.log(` accountID: ${accountKeys[0].toHuman()}`);

    // convert to accountId
    const accountIds = new Array();
    for (const key in accountKeys) {

      // console.log(`${key}: ${accountKeys[key].toHuman()}`);
      // console.log(`type of: ${typeof accountKeys[key].toHuman()}`);
      accountIds.push(accountKeys[key].toHuman().toString());
    }
    console.log(` accountIDs:${accountIds}`);



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
    // const listUnValidatedOrg = Object.entries(accounts).filter(([key,value]) => value.status == "Pending" && value.role == 'Organization');

    // console.log(listUnValidatedOrg);


//---------------------------------------------//
    ///api.tx.hierarchy.approveOrg
    /// only sysman can execute this transaction
    /// params:
    /// + accountId: address
//---------------------------------------------//


    //Validate organization by hierarchy sysman
    //Initial Sysman: Bob , Charlie
    //lấy accountid bằng cách lấy key của listUnValidatedAccounts
    // let accountId = listUnValidatedOrg[0][0];
    // const unsub = await api.tx.hierarchy
    // .approveOrg(accountId)
    // .signAndSend(charlie, (result) => {
    //     console.log(`Current status is ${result.status}`);

    // if (result.status.isFinalized) {
    //     console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
    //     console.log("approve org  successfully");
    //     unsub();
    //     }
    // });

  //   const events = new Promise(async (resolve, reject) => {
  //       await api.tx.hierarchy
  //       .approveOrg('8915a2ac-bbce-480f-8aa9-31c778a8ef98', '5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc')
  //       .signAndSend(bob, ({ status, events, dispatchError }) => {

  //           if (dispatchError) {
  //               if (dispatchError.isModule) {
  //                 // for module errors, we have the section indexed, lookup
  //                 const decoded = api.registry.findMetaError(dispatchError.asModule);
  //                 const { docs, name, section } = decoded;
  //                 const res = section.concat('.',name);
  //                 //console.log(`${section}.${name}: ${docs.join(' ')}`);
  //                 resolve(res)
  //               } else {
  //                 // Other, CannotLookup, BadOrigin, no extra info
  //                 //console.log(dispatchError.toString());
  //                 resolve(dispatchError.toString())
  //               }
  //             }
  //       });
  // })
  // console.log(await events);


    
  

}



main();