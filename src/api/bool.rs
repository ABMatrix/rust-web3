//! `proof` namespace

use api::Namespace;
use helpers::{self, CallFuture};
use types::{Address, Block, BlockId, BlockNumber, Bytes, CallRequest, H256, H520, H64, Index, SyncState, Transaction, TransactionId, TransactionReceipt, TransactionRequest, U256, Work, Filter, Log};
use Transport;
use super::eth::Eth;
use error::Error;

/// `Proof` namespace
#[derive(Debug, Clone)]
pub struct Proof<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for Proof<T> {
    fn new(transport: T) -> Self
        where
            Self: Sized,
    {
        Proof { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> Proof<T> {

//    pub fn get_header(&self, block_id:BlockId) -> CallFuture<T> {
//
//    }

    /// Get receipt proof
    pub fn get_receipt_proof(&self, hash: H256) -> ReceiptProof<T> {
        let eth = Eth::new(self.transport().clone());
        ReceiptProof::new(eth.transaction_receipt(hash), eth)
    }

//    pub fn get_transaction_proof(&self, hash: H256) -> TransactionProof<T> {
//
//    }

//    /// Get receipt proof
//    pub fn get_receipt_proof(&self, hash: H256) -> CallFuture<Option<Block<Transaction>>, T::Out> {
//        use futures::Future;
//        // get transaction
//        let hash = helpers::serialize(&hash);
//        let transaction = self.transport
//            .execute("eth_getTransactionReceipt", vec![hash]);
//
//        let future = transaction.and_then(|data| {
//            let data:Option<Receipt> = serde_json::from_value(data).unwrap();
//            let receipt = data.unwrap();
//            let include_txs = helpers::serialize(&false);
//            let block_hash = helpers::serialize(&BlockId::Hash(receipt.block_hash.unwrap()));
//            return self.transport
//                .execute("eth_getBlockByHash", vec![block_hash, include_txs]);
//        }).then(|data| {
//            data
//        });
//
//        CallFuture::new(future)
//
//    }
}

pub struct CommonFuture {
    inner: Box<Future>,
}

impl CommonFuture {
    /// Create a new CallFuture wrapping the inner future.
    pub fn new(inner: F) -> Self {
        CommonFuture {
            inner: inner,
        }
    }
}

pub enum CommonValue {

}

impl Future for CommonFuture
{
    type Item = CommonValue;
    type Error = Error;

    fn poll(&mut self) -> Poll<T, Error> {
        match self.inner.poll() {

        }
    }
}

use futures::{Future, IntoFuture, Poll, Stream};


pub enum ReceiptProofState<T: Transport> {
    Transaction(CallFuture<Option<Transaction>, T::Out>),
    Block(Transaction, CallFuture<Option<Block<H256>>, T::Out>),
    Receipts(Transaction, Block<H256>, CallFuture<Option<TransactionReceipt>, T::Out>),
}

pub struct ReceiptProof<T: Transport> {
    eth: Eth<T>,
    state: FetchState<T>,
}

impl<T: Transport> ReceiptProof<T> {
    pub fn new(state: ReceiptProofState<T>, eth: Eth<T>) -> Self {
        ReceiptProof {
            eth,
            state,
        }
    }
}

impl<T: Transport> Future for ReceiptProof<T> {
    type Item = Option<TransactionReceipt>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next = match self.state {
                ReceiptProofState::Transaction(ref mut future) => {
                    let trans = try_ready!(future.poll());
                    if let Some(t) = trans {
                        let block_id = BlockId::Hash(t.block_hash.unwrap());
                        ReceiptProofState::Block(t, self.eth.block(block_id))
                    } else {
                        return Ok(None.into())
                    }
                },
                ReceiptProofState::Block(transaction, ref mut future) => {
                    let block = try_ready!(future.poll());
                    if let Some(b) = block {
                        ReceiptProofState::Receipts(transaction, b, self.eth.transaction_receipt(transaction.hash.clone()))
                    } else {
                        return Ok(None.into())
                    }
                },
                ReceiptProofState::Receipts(transaction, block, ref mut future) => {
                    let receipt = try_ready!(future.poll());
                    return Ok(receipt.into())
                }
            };

            self.state = next;
        }
    }
}

//enum State {
//    Begin,
//    Receipt(Receipt),
//    Block(Block<H256>),
//}
//
//impl Default for State {
//    fn default() -> State {
//        State::Begin
//    }
//}
//
//struct Sync {
//    state: State,
//}
//
//impl Sync {
//    pub fn new() -> Self {
//        Sync{
//            state: State::default()
//        }
//    }
//}
//
//impl Future for Sync {
//    type Item = rpc::Value;
//    type Error = Error;
//
//    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
////        match self.state {
////            State::Begin => {
////
////            },
////            State::Receipt(r) => {
////
////            },
////            State::Block(b) => {
////
////            }
////        }
//
//    }
//}