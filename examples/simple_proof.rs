extern crate web3;
extern crate tokio_core;
extern crate env_logger;

use web3::futures::Future;
use web3::types::H256;
use std::str::FromStr;

fn main() {
    env_logger::init();

    let mut event_loop = tokio_core::reactor::Core::new().unwrap();
    let web3 = web3::Web3::new(
        web3::transports::Http::with_event_loop(
            "https://ropsten.infura.io/v3/ff75b47afea04fa49e3d63129e832006",
            &event_loop.handle(),
            64,
        ).unwrap(),
    );
    let hash = H256::from_str("b04fcb9822eb21b5ffdbf89df076de58469af66d23c86abe30266e5d3c5e0db2").unwrap();
    let proof = event_loop.run(web3.bool().get_receipt_proof(hash)).unwrap();
//    let proof = event_loop.run(web3.eth().transaction_receipt(hash)).unwrap();
    println!("Proof: {:?}", proof);
}
