const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');



const main = async() => {
    const provider = new WsProvider('ws://localhost:9944');
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


//---------------------------------------------//
    ///api.query.hierarchy.node
    /// get all sysman in our system
//---------------------------------------------//
    // list current sysman node 
    const nodes = await api.query.hierarchy.node();

    console.log(`Nodes: ${nodes}`);

//---------------------------------------------//
    ///api.query.hierarchy.tree
    /// param: 
    /// + accountId: address
    /// map parent => vec<child>
//---------------------------------------------//


//---------------------------------------------//
    ///api.query.hierarchy.parents
    /// param:
    /// + accountId: address
    /// map child => parent
//---------------------------------------------//



}



main();