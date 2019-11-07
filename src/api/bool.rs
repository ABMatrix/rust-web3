//! `proof` namespace

use api::Namespace;
use helpers::{self, CallFuture, BatchCallFuture};
use types::{Address, Block, BlockId, BlockNumber, Bytes, CallRequest, H256, H520, H64, Index, SyncState, Transaction, TransactionId, TransactionReceipt, TransactionRequest, U256, Work, Filter, Log};
use Transport;
use BatchTransport;
use super::eth::Eth;
use error::Error;

/// `Proof` namespace
#[derive(Debug, Clone)]
pub struct Proof<T> {
    transport: T,
}

impl<T: BatchTransport> Namespace<T> for Proof<T> {
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

impl<T: BatchTransport> Proof<T> {

//    pub fn get_header(&self, block_id:BlockId) -> CallFuture<T> {
//
//    }

//    pub fn receipts(&self, hashs:Vec<H256>) -> BatchCallFuture<Option<TransactionReceipt>, T::Batch> {
//
//        let (id, request) = self.transport.prepare(method, params);
//        self.send(id, request)
//    }

    /// Get receipt proof
    pub fn get_receipt_proof(&self, hash: H256) -> ReceiptProof<T> {
        let hash = TransactionId::Hash(hash);
        let eth = Eth::new(self.transport().clone());
        ReceiptProof::new(ReceiptProofState::Transaction(eth.transaction(hash)), eth)
    }

//    pub fn get_transaction_proof(&self, hash: H256) -> TransactionProof<T> {
//
//    }
}

use futures::{Future, IntoFuture, Poll, Stream};


pub enum ReceiptProofState<T: Transport> {
    Transaction(CallFuture<Option<Transaction>, T::Out>),
    Block(Transaction, CallFuture<Option<Block<H256>>, T::Out>),
    Receipts(Transaction, Block<H256>, CallFuture<Option<TransactionReceipt>, T::Out>),
}

pub struct ReceiptProof<T: Transport> {
    eth: Eth<T>,
    state: ReceiptProofState<T>,
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
                ReceiptProofState::Block(ref transaction, ref mut future) => {
                    let block = try_ready!(future.poll());
                    if let Some(b) = block {
                        ReceiptProofState::Receipts(transaction.clone(), b, self.eth.transaction_receipt(transaction.hash.clone()))
                    } else {
                        return Ok(None.into())
                    }
                },
                ReceiptProofState::Receipts(ref transaction, ref block, ref mut future) => {
                    let receipt = try_ready!(future.poll());
                    return Ok(receipt.into())
                }
            };

            self.state = next;
        }
    }
}
