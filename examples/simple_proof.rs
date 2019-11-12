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
    let hash = H256::from_str("ce62c3d1d2a43cfcc39707b98de53e61a7ef7b7f8853e943d85e511b3451aa7e").unwrap();
    let proof = event_loop.run(bl.receipt_proof(hash)).unwrap();
    println!("Proof: {:?}", proof);
}
