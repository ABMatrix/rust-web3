//! `Abos` namespace
#![allow(dead_code, unused_imports)]
use crate::api::Namespace;
use crate::helpers::{self, CallFuture};
use crate::Transport;
use crate::types::{U256, Bytes, BlockId, H256, Address, U64};
use crate::abos_types::{
    Block, Receipt, Filter, Log, CallRequest, BlockNumber, BlockTransaction, MetaData, TxResponse
};
use std::str::FromStr;
use rustc_hex::FromHex;

/// Abos
#[derive(Debug, Clone)]
pub struct Abos<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for Abos<T> {
    fn new(transport: T) -> Self
    where
        Self: Sized,
    {
        Abos { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> Abos<T> {
    /// Get current block number
    pub fn block_number(&self) -> CallFuture<U64, T::Out> {
        CallFuture::new(self.transport.execute("blockNumber", vec![]))
    }

    /// Get list of peer counts.
    pub fn peer_count(&self) -> CallFuture<U256, T::Out> {
        CallFuture::new(self.transport.execute("peerCount", vec![]))
    }

    /// Sends a signed  raw transaction
    pub fn send_raw_transaction(&self, rlp: Bytes) -> CallFuture<TxResponse, T::Out> {
        let rlp = helpers::serialize(&rlp);
        CallFuture::new(self.transport.execute("sendRawTransaction", vec![rlp]))
    }

    /// Sends a signed  transaction
    pub fn send_transaction(&self, tx: String) -> CallFuture<TxResponse, T::Out> {
        let tx = helpers::serialize(&tx);
        CallFuture::new(self.transport.execute("sendRawTransaction", vec![tx]))
    }

    /// Get block details with transaction hashes.
    pub fn block(&self, block: BlockId) -> CallFuture<Option<Block>, T::Out> {
        let include_txs = helpers::serialize(&false);

        let result = match block {
            BlockId::Hash(hash) => {
                let hash = helpers::serialize(&hash);
                self.transport
                    .execute("getBlockByHash", vec![hash, include_txs])
            }
            BlockId::Number(num) => {
                let num = helpers::serialize(&num);
                self.transport
                    .execute("getBlockByNumber", vec![num, include_txs])
            }
        };

        CallFuture::new(result)
    }

    /// Get transaction receipt
    pub fn transaction_receipt(&self, hash: H256) -> CallFuture<Option<Receipt>, T::Out> {
        let hash = helpers::serialize(&hash);

        CallFuture::new(
            self.transport
                .execute("getTransactionReceipt", vec![hash]),
        )
    }

    /// Get all logs matching a given filter object
    pub fn logs(&self, filter: Filter) -> CallFuture<Vec<Log>, T::Out> {
        let filter = helpers::serialize(&filter);
        CallFuture::new(self.transport.execute("getLogs", vec![filter]))
    }

    /// Call a constant method of contract without changing the state of the blockchain.
    pub fn call(&self, req: CallRequest, block: Option<BlockNumber>) -> CallFuture<Bytes, T::Out> {
        let req = helpers::serialize(&req);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::latest()));

        CallFuture::new(self.transport.execute("call", vec![req, block]))
    }


    /// Get transaction
    pub fn transaction(&self, hash: H256) -> CallFuture<Option<BlockTransaction>, T::Out> {
        let hash = helpers::serialize(&hash);
        let result = self.transport
            .execute("getTransaction", vec![hash]);

        CallFuture::new(result)
    }

    /// Get nonce
    pub fn transaction_count(&self, address: Address, block: Option<BlockNumber>) -> CallFuture<U256, T::Out> {
        let address = helpers::serialize(&address);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::latest()));

        CallFuture::new(
            self.transport
                .execute("getTransactionCount", vec![address, block]),
        )
    }

    /// Get code under given address
    pub fn code(&self, address: Address, block: Option<BlockNumber>) -> CallFuture<Bytes, T::Out> {
        let address = helpers::serialize(&address);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::latest()));

        CallFuture::new(self.transport.execute("getCode", vec![address, block]))
    }

    /// Get abi by address
    pub fn abi(&self, address: Address, block: Option<BlockNumber>) -> CallFuture<Bytes, T::Out> {
        let address = helpers::serialize(&address);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::latest()));

        CallFuture::new(self.transport.execute("getAbi", vec![address, block]))
    }

    /// Get balance of given address
    pub fn balance(&self, address: Address, block: Option<BlockNumber>) -> CallFuture<U256, T::Out> {
        let address = helpers::serialize(&address);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::latest()));

        CallFuture::new(
            self.transport
                .execute("getBalance", vec![address, block]),
        )
    }

    /// create new filter
    pub fn new_filter(&self, filter: Filter) -> CallFuture<U256, T::Out> {
        let filter = helpers::serialize(&filter);
        CallFuture::new(self.transport.execute("newFilter", vec![filter]))
    }

    /// Start new block filter
    pub fn new_block_filter(&self) -> CallFuture<U256, T::Out> {
        CallFuture::new(self.transport.execute("newBlockFilter", vec![]))
    }

    /// uninstall filter
    pub fn uninstall_filter(&self, quantity: U256) -> CallFuture<bool, T::Out> {
        let quantity = helpers::serialize(&quantity);
        CallFuture::new(self.transport.execute("uninstallFilter", vec![quantity]))
    }

    ///  get filter changes
    pub fn filter_changes(&self, quantity: U256) -> CallFuture<Vec<Log>, T::Out> {
        let quantity = helpers::serialize(&quantity);
        CallFuture::new(self.transport.execute("getFilterChanges", vec![quantity]))
    }

    ///  get filter logs
    pub fn filter_logs(&self, quantity: U256) -> CallFuture<Vec<Log>, T::Out> {
        let quantity = helpers::serialize(&quantity);
        CallFuture::new(self.transport.execute("getFilterLogs", vec![quantity]))
    }

    /// get transaction proof
    pub fn transaction_proof(&self, data: H256) -> CallFuture<Option<Bytes>, T::Out> {
        let data = helpers::serialize(&data);
        CallFuture::new(self.transport.execute("getTransactionProof", vec![data]))
    }

    /// get metadata
    pub fn meta_data(&self, block: Option<BlockNumber>) -> CallFuture<MetaData, T::Out> {
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::latest()));
        CallFuture::new(self.transport.execute("getMetaData", vec![block]))
    }

    /// get storage at
    pub fn get_storage_at(&self, address: Address, key: H256, block: Option<BlockNumber>) -> CallFuture<Option<Bytes>, T::Out> {
        let address = helpers::serialize(&address);
        let key = helpers::serialize(&key);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::latest()));
        CallFuture::new(self.transport.execute("getStorageAt", vec![address, key, block]))
    }

    // /// get peers
    // pub fn get_peers(&self) -> CallFuture<Peers, T::Out> {
    //     CallFuture::new(self.transport.execute("getPeers", vec![]))
    // }

    // /// get estimate gas
    // pub fn estimate_gas(&self, req: EstimateRequest, block: Option<BlockNumber>) -> CallFuture<U256, T::Out> {
    //     let req = helpers::serialize(&req);
    //     let block = helpers::serialize(&block.unwrap_or(BlockNumber::latest()));

    //     CallFuture::new(self.transport.execute("estimateGas", vec![req, block]))
    // }

    // /// get gas price
    // pub fn gas_price(&self) -> CallFuture<U256, T::Out> {
    //     CallFuture::new(self.transport.execute("getGasPrice", vec![]))
    // }

    // /// get state of chain
    // pub fn syncing(&self) -> CallFuture<bool, T::Out> {
    //     CallFuture::new(self.transport.execute("syncing", vec![]))
    // }

    // /// Get transaction receipt ex
    // pub fn transaction_receipt_ex(&self, hash: H256) -> CallFuture<Option<ReceiptEx>, T::Out> {
    //     let hash = helpers::serialize(&hash);

    //     CallFuture::new(
    //         self.transport
    //             .execute("getTransactionReceiptEx", vec![hash]),
    //     )
    // }
}

//
//#[cfg(test)]
//mod tests {
//    use futures::Future;
//
//    use api::Namespace;
//    use types::{Block, BlockId, BlockNumber, Bytes, CallRequest, H256, Transaction, TransactionId, TransactionReceipt,
//                TransactionRequest, Work};
//    use rpc::Value;
//
//    use super::Abos;
//
//    // taken from RPC docs.
//    const EXAMPLE_BLOCK: &'static str = r#"{
//    "number": "0x1b4",
//    "hash": "0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
//    "parentHash": "0x9646252be9520f6e71339a8df9c55e4d7619deeb018d2a3f2d21fc165dde5eb5",
//    "sealFields": [
//      "0xe04d296d2460cfb8472af2c5fd05b5a214109c25688d3704aed5484f9a7792f2",
//      "0x0000000000000042"
//    ],
//    "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
//    "logsBloom":  "0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec643\
// 41771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5\
// 145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec6\
// 4341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3\
// d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec\
// 64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
//    "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
//    "receiptsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
//    "stateRoot": "0xd5855eb08b3387c0af375e9cdb6acfc05eb8f519e419b874b6ff2ffda7ed1dff",
//    "miner": "0x4e65fda2159562a496f9f3522f89122a3088497a",
//    "difficulty": "0x27f07",
//    "totalDifficulty": "0x27f07",
//    "extraData": "0x0000000000000000000000000000000000000000000000000000000000000000",
//    "size": "0x27f07",
//    "gasLimit": "0x9f759",
//    "minGasPrice": "0x9f759",
//    "gasUsed": "0x9f759",
//    "timestamp": "0x54e34e8e",
//    "transactions": [],
//    "uncles": []
//  }"#;
//
//    // taken from RPC docs.
//    const EXAMPLE_TX: &'static str = r#"{
//    "hash": "0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d1055b",
//    "nonce": "0x0",
//    "blockHash": "0xbeab0aa2411b7ab17f30a99d3cb9c6ef2fc5426d6ad6fd9e2a26a6aed1d1055b",
//    "blockNumber": "0x15df",
//    "transactionIndex": "0x1",
//    "from": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
//    "to":   "0x85dd43d8a49eeb85d32cf465507dd71d507100c1",
//    "value": "0x7f110",
//    "gas": "0x7f110",
//    "gasPrice": "0x09184e72a000",
//    "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360"
//  }"#;
//
//    // taken from RPC docs.
//    const EXAMPLE_RECEIPT: &'static str = r#"{
//    "hash": "0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238",
//    "index": "0x1",
//    "transactionHash": "0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238",
//    "transactionIndex": "0x1",
//    "blockNumber": "0xb",
//    "blockHash": "0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d1055b",
//    "cumulativeGasUsed": "0x33bc",
//    "gasUsed": "0x4dc",
//    "contractAddress": "0xb60e8dd61c5d32be8058bb8eb970870f07233155",
//    "logs": []
//  }"#;
//
//
//    rpc_test! (
//    Eth:block_number => "eth_blockNumber";
//    Value::String("0x123".into()) => 0x123
//  );
//
//    rpc_test! (
//    Eth:call, CallRequest {
//      from: None, to: 0x123.into(),
//      gas: None, gas_price: None,
//      value: Some(0x1.into()), data: None,
//    }, None
//    =>
//    "eth_call", vec![r#"{"to":"0x0000000000000000000000000000000000000123","value":"0x1"}"#, \
// r#""latest""#];
//    Value::String("0x010203".into()) => Bytes(vec![1, 2, 3])
//  );
//
//
//    rpc_test! (
//    Eth:block:block_by_hash, BlockId::Hash(0x123.into())
//    =>
//    "cita_getBlockByHash", vec![r#""0x0000000000000000000000000000000000000000000000000000000\
// 000000123""#, r#"false"#];
//    ::serde_json::from_str(EXAMPLE_BLOCK).unwrap()
//    => ::serde_json::from_str::<Block<H256>>(EXAMPLE_BLOCK).unwrap()
//  );
//
//    rpc_test! (
//    Eth:block, BlockNumber::Pending
//    =>
//    "cita_getBlockByNumber", vec![r#""pending""#, r#"false"#];
//    ::serde_json::from_str(EXAMPLE_BLOCK).unwrap()
//    => ::serde_json::from_str::<Block<H256>>(EXAMPLE_BLOCK).unwrap()
//  );
//
//
//    rpc_test! (
//    Eth:code, 0x123, Some(BlockNumber::Pending)
//    =>
//    "eth_getCode", vec![r#""0x0000000000000000000000000000000000000123""#, r#""pending""#];
//    Value::String("0x0123".into()) => Bytes(vec![0x1, 0x23])
//  );
//
//    rpc_test! (
//    Eth:transaction_count, 0x123, None
//    =>
//    "eth_getTransactionCount", vec![r#""0x0000000000000000000000000000000000000123""#, r#""latest""#];
//    Value::String("0x123".into()) => 0x123
//  );
//
//    rpc_test! (
//    Eth:transaction:tx_by_hash, TransactionId::Hash(0x123.into())
//    =>
//    "cita_getTransaction", vec![r#""0x0000000000000000000000000000000000000000000000000000000000000123""#];
//    ::serde_json::from_str(EXAMPLE_TX).unwrap()
//    => Some(::serde_json::from_str::<Transaction>(EXAMPLE_TX).unwrap())
//  );
//
//
//    rpc_test! (
//    Eth:transaction_receipt, 0x123
//    =>
//    "eth_getTransactionReceipt", vec![r#""0x0000000000000000000000000000000000000000000000000000000000000123""#];
//    ::serde_json::from_str(EXAMPLE_RECEIPT).unwrap()
//    => Some(::serde_json::from_str::<TransactionReceipt>(EXAMPLE_RECEIPT).unwrap())
//  );
//
//    rpc_test! (
//    Eth:new_block_filter => "eth_newBlockFilter";
//    Value::String("0x123".into()) => 0x123
//  );
//    rpc_test! (
//    Eth:new_pending_transaction_filter => "eth_newPendingTransactionFilter";
//    Value::String("0x123".into()) => 0x123
//  );
//
//    rpc_test! (
//    Eth:send_raw_transaction, Bytes(vec![1, 2, 3, 4])
//    =>
//    "eth_sendRawTransaction", vec![r#""0x01020304""#];
//    Value::String("0x0000000000000000000000000000000000000000000000000000000000000123".into()) => 0x123
//  );
//
//    rpc_test! (
//    Eth:send_transaction, TransactionRequest {
//      from: 0x123.into(), to: Some(0x123.into()),
//      gas: None, gas_price: Some(0x1.into()),
//      value: Some(0x1.into()), data: None,
//      nonce: None, condition: None,
//    }
//    =>
//    "cita_sendTransaction", vec![r#"{"from":"0x0000000000000000000000000000000000000123",\
// "gasPrice":"0x1","to":"0x0000000000000000000000000000000000000123","value":"0x1"}"#];
//    Value::String("0x0000000000000000000000000000000000000000000000000000000000000123".into()) => 0x123
//  );
//}
