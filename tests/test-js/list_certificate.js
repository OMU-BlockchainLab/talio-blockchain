const { ApiPromise, WsProvider } = require('@polkadot/api');


const main = async() => {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider });
    console.log('Connected to blockchain');

    const certs = new Map();
    const certificates = await api.query.certificate.certificateById.entries();

    certificates.forEach(function(cert) {
        certs.set(cert[0].toHuman(),cert[1].toString());
    });

    console.log(certs);
    //Kết quả
    // Map(2) {
    //     [ '1234' ] => '{"owner":"0x34623738373030392d643832362d346636362d396536372d326666333535636162666436","org":null,"score":null,"isPublic":true,"createdDate":1662515064002,"expiredDate":null,"status":"Unvalidated"}',
    //     [ '123' ] => '{"owner":"0x34623738373030392d643832362d346636362d396536372d326666333535636162666436","org":null,"score":null,"isPublic":true,"createdDate":1662513960003,"expiredDate":null,"status":"Unvalidated"}'
    //   }
    // Cần chuyển "0x34623738373030392d643832362d346636362d396536372d326666333535636162666436" ra dạng array [u8;36] nữa
}



main();