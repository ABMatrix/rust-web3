extern crate web3;
extern crate tokio_core;
extern crate rustc_hex;
extern crate rlp;

use std::str::FromStr;
use rustc_hex::ToHex;
use web3::{
    api::Namespace,
    futures::Future,
    types::{H256, RawReceipt}
};

fn main() {
    let mut event_loop = tokio_core::reactor::Core::new().unwrap();
    let bl = web3::api::Bool::new(
        web3::transports::Http::with_event_loop(
            "https://ropsten.infura.io/v3/ff75b47afea04fa49e3d63129e832006",
            &event_loop.handle(),
            64,
        ).unwrap(),
    );
    let hash = H256::from_str("b04fcb9822eb21b5ffdbf89df076de58469af66d23c86abe30266e5d3c5e0db2").unwrap();
    let raw_receipt = event_loop.run(bl.raw_transaction_receipt(hash)).unwrap();
    println!("raw receipt: {:?} ", raw_receipt);

    let rlp_receipt = rlp::encode(&raw_receipt);
    println!("raw rlp hex: {}",rlp_receipt.to_hex());

    let de_receipt:RawReceipt = rlp::decode(&rlp_receipt).unwrap();
}
