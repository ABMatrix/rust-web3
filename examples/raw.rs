extern crate web3;
extern crate tokio_core;
extern crate rustc_hex;
extern crate rlp;

use std::str::FromStr;
use rustc_hex::ToHex;
use web3::{
    api::Namespace,
    types::{H256, RawReceipt}
};
use web3::types::BlockId;

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
    if let Some(raw_receipt) = event_loop.run(bl.raw_transaction_receipt(hash)).unwrap() {
        println!("raw receipt: {:?} ", raw_receipt);
        let rlp_receipt = rlp::encode(&raw_receipt);
        println!("raw rlp hex: {}", rlp_receipt.to_hex::<String>());

        let de_receipt: RawReceipt = rlp::decode(&rlp_receipt).unwrap();
    }

    let expected = "f9020ea00e6c4c42797b9ff736222c9172fda32564f4028bd7a0506af58ca87b6a4516eba01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d4934794d7a15baeb7ea05c9660cbe03fb7999c2c2e57625a0abc2db349e5361fee90bf67fb015cf5991b77f2771e4c3cf627a74b119068184a01a5ad14d4162624fd7f47fd40e54b6198c3f7ad8c79fff9ce5481ce8d3f5169ca07fa081e3e33e53c4d09ae691af3853bb73a7e02c856104fe843172abab85df7bb90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000400000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000200000000000000000000000000000000008501b4ee1cc0836683ab837a121d835cdb6e845dc2404a8f41746c616e7469632043727970746fa0b13e3b734345e4b6f56af4b228fe00c9d77a3d1859ac44279a52f534b8e1869a8860f7e4e35f62e3bb";
    let block_hash = H256::from_str("a9eb12b702c62ab317547e9375601992ce1d6d9ec3de6c77c0abeac41eae2706").unwrap();
    if let Some(header) = event_loop.run(bl.raw_header(BlockId::Hash(block_hash.clone()))).unwrap() {
        println!("{:?}", header);
        let hash = header.hash();
        assert_eq!(hash, block_hash);
        let rlp_header = rlp::encode(&header);
        let hex_header = rlp_header.to_hex::<String>();
        assert_eq!(expected, hex_header);
    }
}
