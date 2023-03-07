const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');



const main = async() => {
    const provider = new WsProvider('wss://smartcv.org/smartcv-node');
    const api = await ApiPromise.create({ provider });
    console.log('Connected to blockchain');
    
    //Lấy toàn bộ thông tin mà Organization và SysMan chưa được validate

    const accountKeys = await api.query.account.accountStorage.keys();
    //console.log(`account: ${accountKeys}`);
    //console.log(`type of account: ${typeof accountKeys}`);
    //console.log(` accountID: ${accountKeys[0].toHuman()}`);

    // convert to accountId
    const accountIds = new Array();
    for (const key in accountKeys) {

      // console.log(`${key}: ${accountKeys[key].toHuman()}`);
      // console.log(`type of: ${typeof accountKeys[key].toHuman()}`);
      accountIds.push(accountKeys[key].toHuman().toString());
    }
    //console.log(` accountIDs:${accountIds}`);



    //list all accounts storage


    const infoAccounts = new Promise(async (resolve, reject) => {
      try {
        await api.query.account.accountStorage.multi(accountIds, (accountStorage) => {  
            resolve(accountStorage);
        });

      } catch (error) {
          reject(error);
      }
    })
    //console.log(`List all account storage:${await infoAccounts}`);

    //console.log(`accounts array:${Object.values(await infoAccounts)}`);
    let infos = new Array();
    Object.values(await infoAccounts).map(info => {
      infos.push(info.toHuman());
    })
    const accounts = {};
    accountIds.forEach((value,index) =>{
        accounts[value] = infos[index];
    } ) ;

    //console.log(accounts);
    // List sysman
    const listSysMan = Object.entries(accounts).filter(([key,value]) => value.role == 'SysMan');

    console.log(listSysMan);

    // List Organization
    const listOrganization = Object.entries(accounts).filter(([key,value]) => value.role == 'Organization');

    console.log(listOrganization);


}



main();