extern crate tokio_core;
extern crate web3;
extern crate serde_json;

use web3::futures::Future;
use web3::types::{BlockId, BlockNumber, H256, Address, U256};
use std::str::FromStr;
use web3::abos_types::{Filter, CallRequest, EstimateRequest};

const MAX_PARALLEL_REQUESTS: usize = 64;

fn main() {

    let mut event_loop = tokio_core::reactor::Core::new().unwrap();
    let web3 = web3::Web3::new(
        web3::transports::Http::with_event_loop(
            "http://47.99.236.158:1339",
            &event_loop.handle(),
            MAX_PARALLEL_REQUESTS,
        ).unwrap(),
    );

    //get height
    let block_number = web3.abos().block_number().map(|height| {
        println!("height: {:?}", height);
    });
    event_loop.run(block_number).unwrap();

    // get peer count
    let peercount = web3.abos().peer_count().map(|count| {
        println!("peercount: {:?}", count);
    });
    event_loop.run(peercount).unwrap();


    //
    let hash:H256 = H256::from_str("ad5002c6da450e8bfe391b717d14c0008bbf4927ac3c14f126a0ff9f01873f6f").unwrap();
    // get block by hash
    let block_hash = web3.abos().block(BlockId::Hash(hash)).map( |block| {
        println!("block info: {:?}", block);
    });
    event_loop.run(block_hash).unwrap();

    //
    let num = BlockNumber::Latest;
    // get block by hash
    let block_hash = web3.abos().block(BlockId::Number(num)).map( |block| {
        println!("block info: {:?}", block);
    });
    event_loop.run(block_hash).unwrap();

    // get transaction receipt
    let hash = H256::from_str("2b691bcb20ab6765e3ed1c7c2f88086121d6bf31db72c07a4427e54b6691ae11").unwrap();
    let transaction_receipt = web3.abos().transaction_receipt(hash).map(|receipt| {
        println!("transaction receipt: {:?}", receipt);
    });
    event_loop.run(transaction_receipt).unwrap();


    //
    let s =
        "{\"topics\":[\"0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3\",\
             \"0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3\",\
             \"0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3\"]}";
    let filter: Filter = serde_json::from_str(s).unwrap();
    let filter_call = web3.abos().logs(filter).map(|logs| {
        println!("logs: {:?}", logs);
    });
    event_loop.run(filter_call).unwrap();


    // call
    let json = r#"
{
"from": "0x0000000000000000000000000000000000000001",
"to": "0x0000000000000000000000000000000000000002",
"data": "0xabcdef"
}
    "#;
    let call_request:CallRequest = serde_json::from_str(json).unwrap();

    let call_call = web3.abos().call(call_request, None).map(|resp| {
        println!("call: {:?}", resp);
    });
    event_loop.run(call_call).unwrap();

    // get transaction by hash
    let hash = H256::from_str("2b691bcb20ab6765e3ed1c7c2f88086121d6bf31db72c07a4427e54b6691ae11").unwrap();
    let get_transaction = web3.abos().transaction(hash).map(|transaction| {
        println!("transaction: {:?}", transaction);
    });
    event_loop.run(get_transaction).unwrap();

    // get transaction count
    let addr = Address::from_str("3ae5628d6731fbaba7e2340dc925716beef1d4d6").unwrap();
    let get_transaction_count = web3.abos().transaction_count(addr, None).map(|count| {
        println!("transaction count: {:?}", count);
    });
    event_loop.run(get_transaction_count).unwrap();


    // get code
    let addr = Address::from_str("adac34c45e07e2c64910332bbc29a829201e0698").unwrap();
    let get_code = web3.abos().code(addr, None).map(|code| {
        println!("code: {:?}", code);
    });
    event_loop.run(get_code).unwrap();


    // get abi
    let addr = Address::from_str("adac34c45e07e2c64910332bbc29a829201e0698").unwrap();
    let get_abi = web3.abos().abi(addr, None).map(|abi| {
        println!("abi: {:?}", abi);
    });
    event_loop.run(get_abi).unwrap();


    // get balance
    let addr = Address::from_str("47bb23ad36d7ba8c86f261b71392bead660d51d3").unwrap();
    let get_balance = web3.abos().balance(addr, None).map(|balance| {
        println!("balance: {:?}", balance);
    });
    event_loop.run(get_balance).unwrap();



    let s =
        "{\"topics\":[\"0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3\",\
             \"0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3\",\
             \"0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3\"]}";
    let filter: Filter = serde_json::from_str(s).unwrap();
    let new_filter_call = web3.abos().new_filter(filter).map(|quantity| {
        println!("Quantity: {:?}", quantity);
    });
    event_loop.run(new_filter_call).unwrap();


    // new block filter
    let new_block_filter = web3.abos().new_block_filter().map( |quantity| {
        println!("Quantity: {:?}", quantity);
    });
    event_loop.run(new_block_filter).unwrap();


    // uninstall filter
    let quantity = U256::from(0);
    println!("{}", quantity);
    let uninstall_filter = web3.abos().uninstall_filter(quantity).map(|bl| {
        println!("Boolean: {:?}", bl);
    });
    event_loop.run(uninstall_filter).unwrap();

    // get filter change
    let quantity = U256::from(1);
    println!("{}", quantity);
    let filter_changes = web3.abos().filter_changes(quantity).map(|cg| {
        println!("changes: {:?}", cg);
    });
    event_loop.run(filter_changes).unwrap();

    // get filter logs
    let quantity = U256::from(0);
    println!("{}", quantity);
    let filter_logs = web3.abos().filter_logs(quantity).map(|cg| {
        println!("filter logs: {:?}", cg);
    });
    event_loop.run(filter_logs).unwrap();


    // get transaction proof
    let hash = H256::from_str("781c7ceea81befc75ec8f5db90483155627c776ae60a969b2e333a1ecf9186d8").unwrap();
    let get_transaction_proof = web3.abos().transaction_proof(hash).map(|transaction| {
        println!("transaction: {:?}", transaction);
    });
    event_loop.run(get_transaction_proof).unwrap();

    // get meta data
    let meta_data = web3.abos().meta_data(None).map(|data| {
        println!("meta data: {:?}", data);
        data
    });
    event_loop.run(meta_data).unwrap();

    // // get storage of address
    // let address = Address::from_str("ffffffffffffffffffffffffffffffffff020000").unwrap();
    // let key = H256::from_str("0000000000000000000000000000000000000000000000000000000000000007").unwrap();
    // let f_storage_at = web3.abos().get_storage_at(address, key, None);
    // let value = event_loop.run(f_storage_at).unwrap();
    // println!("storage at: {:?}", value);

//     // get peers
//     let f_peers = web3.abos().get_peers();
//     let peers = event_loop.run(f_peers).unwrap();
//     println!("peers: {:?}", peers);

//     // get syncing
//     let f_sync = web3.abos().syncing();
//     let sync = event_loop.run(f_sync).unwrap();
//     println!("syncing: {:?}", sync);

//     // get gas price
//     let f_gas_price = web3.abos().gas_price();
//     let gas_price = event_loop.run(f_gas_price).unwrap();
//     println!("gas price: {:?}", gas_price);

//     // estimate gas
//     let json = r#"
//         {
//         "to": "0x0000000000000000000000000000000000000002",
//         "data": "0xabcdef"
//         }
//     "#;
//     let estimate_request:EstimateRequest = serde_json::from_str(json).unwrap();
//     let f_estimate_gas = web3.abos().estimate_gas(estimate_request, None);
//     let gas = event_loop.run(f_estimate_gas).unwrap();
//     println!("estimate gas: {:?}", gas);

//     // get transaction receiptex
//     let hash = H256::from_str("781c7ceea81befc75ec8f5db90483155627c776ae60a969b2e333a1ecf9186d8").unwrap();
//     let f_receiptex = web3.abos().transaction_receipt_ex(hash);
//     let receiptex = event_loop.run(f_receiptex).unwrap();
//     println!("receipt ex: {:?}", receiptex);
}
