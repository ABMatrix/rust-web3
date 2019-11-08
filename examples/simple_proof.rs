extern crate web3;
extern crate tokio_core;
extern crate env_logger;

use std::str::FromStr;
use web3::{
    api::Namespace,
    futures::Future,
    types::H256
};

fn main() {
    env_logger::init();

    let mut event_loop = tokio_core::reactor::Core::new().unwrap();
    let bl = web3::api::Bool::new(
        web3::transports::Http::with_event_loop(
            "https://ropsten.infura.io/v3/ff75b47afea04fa49e3d63129e832006",
            &event_loop.handle(),
            64,
        ).unwrap(),
    );
    let hash = H256::from_str("b04fcb9822eb21b5ffdbf89df076de58469af66d23c86abe30266e5d3c5e0db2").unwrap();
    let proof = event_loop.run(bl.receipt_proof(hash)).unwrap();
    println!("Proof: {:?}", proof);
}
