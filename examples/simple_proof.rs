extern crate web3;
extern crate tokio_core;
extern crate env_logger;

use web3::futures::Future;
use web3::types::H256;
use std::str::FromStr;
use web3::api::Namespace;

fn main() {
    env_logger::init();

    let mut event_loop = tokio_core::reactor::Core::new().unwrap();
    let bool = web3::api::Bool::new(
        web3::transports::Http::with_event_loop(
            "https://ropsten.infura.io/v3/ff75b47afea04fa49e3d63129e832006",
            &event_loop.handle(),
            64,
        ).unwrap(),
    );
    let hash = H256::from_str("b04fcb9822eb21b5ffdbf89df076de58469af66d23c86abe30266e5d3c5e0db2").unwrap();
    let hash2 = H256::from_str("a792d598c1f29cb48e0c1301c36f40dcff68541ff928a7a9aa30dbdcb1a0ee6b").unwrap();
//    let proof = event_loop.run(bool.receipts(vec![hash, hash2])).unwrap();
    let proof = event_loop.run(bool.receipt_proof(hash)).unwrap();
    println!("Proof: {:?}", proof);
}
