const nearAPI = require("near-api-js");
const { KeyPair} = nearAPI;
let a = KeyPair.fromRandom("ED25519")
let b = a.getPublicKey().toString()
console.log(a)
console.log(b)

