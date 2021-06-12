const nearAPI = require("near-api-js");
const sha256 = require("js-sha256");
const BN = require("bn.js");


const networkId = "testnet";

//rpc
const provider = new nearAPI.providers.JsonRpcProvider(
    `https://rpc.${networkId}.near.org`
);

//get key for pri
const privateKey = "qcsNgUS65nYsipiN9cwsEk27uw2EG1nivair4NcEcpVrMU2JrGm7vs9AccvcVU5ZUzsJjy6gPmWpxRr59bskyG7";
const keyPair = nearAPI.utils.key_pair.KeyPairEd25519.fromString(privateKey);
const publicKey = keyPair.getPublicKey();



async function a(){

    //config
    const sender = "zombie6.testnet";
    const receiver = "zombie6.testnet";


    const accessKey = await provider.query(
        `access_key/${sender}/${publicKey.toString()}`, ''
    );

    //none
    const nonce = ++accessKey.nonce;


    //action
    // const actions = [nearAPI.transactions.addKey(
    //     nearAPI.utils.PublicKey.from("5TjYQ5TrXDd65o18dZPBgiscARo3w6Djdf7EjMwWMgdH"), //ms1
    //     nearAPI.transactions.functionCallAccessKey("zombie4.testnet",["claim"],new BN("300000000000000"))
    // )]

    const actions = [nearAPI.transactions.functionCall("claim",{account_id:"zombie5.testnet"} ,new BN("300000000000000"),0)]

    //区块哈希查询
    const recentBlockHash = nearAPI.utils.serialize.base_decode(
        accessKey.block_hash
    );

    //定义交易结构
    const transaction = nearAPI.transactions.createTransaction(
        sender,
        publicKey,
        receiver,
        nonce,
        actions,
        recentBlockHash
    );

    //序列化对象
    const serializedTx = nearAPI.utils.serialize.serialize(
        nearAPI.transactions.SCHEMA,
        transaction
    );

    //数组转换
    const serializedTxHash = new Uint8Array(sha256.sha256.array(serializedTx));

    //签名
    const signature = keyPair.sign(serializedTxHash);

    //签署交易
    const signedTransaction = new nearAPI.transactions.SignedTransaction({
        transaction,
        signature: new nearAPI.transactions.Signature({
            keyType: transaction.publicKey.keyType,
            data: signature.signature,
        }),
    });

    //编码交易
    const signedSerializedTx = signedTransaction.encode();

    //发送交易
    // sends transaction to NEAR blockchain via JSON RPC call and records the result
    const result = await provider.sendJsonRpc("broadcast_tx_commit", [
        Buffer.from(signedSerializedTx).toString("base64"),
    ]);
    console.log(result)
    console.log(result.Failure)
}

a()