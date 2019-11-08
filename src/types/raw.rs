use hash::{keccak, KECCAK_EMPTY_LIST_RLP, KECCAK_NULL_RLP};
use ethereum_types::{Address, Bloom, H256, U256, BloomInput};
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use serde::{Serialize, Serializer};

pub type Bytes = Vec<u8>;
pub type BlockNumber = u64;

/// Semantic boolean for when a seal/signature is included.
#[derive(Debug, Clone, Copy)]
enum Seal {
    /// The seal/signature is included.
    With,
    /// The seal/signature is not included.
    Without,
}

/// The header of ethereum
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// Parent hash.
    pub parent_hash: H256,
    /// Block timestamp.
    pub timestamp: u64,
    /// Block number.
    pub number: BlockNumber,
    /// Block author.
    pub author: Address,

    /// Transactions root.
    pub transactions_root: H256,
    /// Block uncles hash.
    pub uncles_hash: H256,
    /// Block extra data.
    pub extra_data: Bytes,

    /// State root.
    pub state_root: H256,
    /// Block receipts root.
    pub receipts_root: H256,
    /// Block bloom.
    pub log_bloom: Bloom,
    /// Gas used for contracts execution.
    pub gas_used: U256,
    /// Block gas limit.
    pub gas_limit: U256,

    /// Block difficulty.
    pub difficulty: U256,
    /// Vector of post-RLP-encoded fields.
    pub seal: Vec<Bytes>,

    /// Memoized hash of that header and the seal.
    pub hash: Option<H256>,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            parent_hash: H256::zero(),
            timestamp: 0,
            number: 0,
            author: Address::zero(),

            transactions_root: KECCAK_NULL_RLP,
            uncles_hash: KECCAK_EMPTY_LIST_RLP,
            extra_data: Vec::new(),

            state_root: KECCAK_NULL_RLP,
            receipts_root: KECCAK_NULL_RLP,
            log_bloom: Bloom::default(),
            gas_used: U256::default(),
            gas_limit: U256::default(),

            difficulty: U256::default(),
            seal: Vec::new(),
            hash: None,
        }
    }
}

impl Header {
    /// Get the hash of this header (keccak of the RLP with seal).
    pub fn hash(&self) -> H256 {
        self.hash.unwrap_or_else(|| keccak(self.rlp(Seal::With)))
    }

    /// Get the hash of the header excluding the seal
    pub fn bare_hash(&self) -> H256 {
        keccak(self.rlp(Seal::Without))
    }

    /// Get the RLP representation of this Header.
    fn rlp(&self, with_seal: Seal) -> Bytes {
        let mut s = RlpStream::new();
        self.stream_rlp(&mut s, with_seal);
        s.out()
    }

    /// Place this header into an RLP stream `s`, optionally `with_seal`.
    fn stream_rlp(&self, s: &mut RlpStream, with_seal: Seal) {
        if let Seal::With = with_seal {
            s.begin_list(13 + self.seal.len());
        } else {
            s.begin_list(13);
        }

        s.append(&self.parent_hash);
        s.append(&self.uncles_hash);
        s.append(&self.author);
        s.append(&self.state_root);
        s.append(&self.transactions_root);
        s.append(&self.receipts_root);
        s.append(&self.log_bloom);
        s.append(&self.difficulty);
        s.append(&self.number);
        s.append(&self.gas_limit);
        s.append(&self.gas_used);
        s.append(&self.timestamp);
        s.append(&self.extra_data);

        if let Seal::With = with_seal {
            for b in &self.seal {
                s.append_raw(b, 1);
            }
        }
    }
}

impl Decodable for Header {
    fn decode(r: &Rlp) -> Result<Self, DecoderError> {
        let mut blockheader = Header {
            parent_hash: r.val_at(0)?,
            uncles_hash: r.val_at(1)?,
            author: r.val_at(2)?,
            state_root: r.val_at(3)?,
            transactions_root: r.val_at(4)?,
            receipts_root: r.val_at(5)?,
            log_bloom: r.val_at(6)?,
            difficulty: r.val_at(7)?,
            number: r.val_at(8)?,
            gas_limit: r.val_at(9)?,
            gas_used: r.val_at(10)?,
            timestamp: r.val_at(11)?,
            extra_data: r.val_at(12)?,
            seal: Vec::new(),
            hash: keccak(r.as_raw()).into(),
        };

        for i in 13..r.item_count()? {
            blockheader.seal.push(r.at(i)?.as_raw().to_vec())
        }

        Ok(blockheader)
    }
}

impl Encodable for Header {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.stream_rlp(s, Seal::With);
    }
}

/// Transaction outcome store in the receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionOutcome {
    /// Status and state root are unknown under EIP-98 rules.
    Unknown,
    /// State root is known. Pre EIP-98 and EIP-658 rules.
    StateRoot(H256),
    /// Status code is known. EIP-658 rules.
    StatusCode(u8),
}

/// Information describing execution of a transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    /// The total gas used in the block following execution of the transaction.
    pub gas_used: U256,
    /// The OR-wide combination of all logs' blooms for this transaction.
    pub log_bloom: Bloom,
    /// The logs stemming from this transaction.
    pub logs: Vec<LogEntry>,
    /// Transaction outcome.
    pub outcome: TransactionOutcome,
}

impl Encodable for Receipt {
    fn rlp_append(&self, s: &mut RlpStream) {
        match self.outcome {
            TransactionOutcome::Unknown => {
                s.begin_list(3);
            }
            TransactionOutcome::StateRoot(ref root) => {
                s.begin_list(4);
                s.append(root);
            }
            TransactionOutcome::StatusCode(ref status_code) => {
                s.begin_list(4);
                s.append(status_code);
            }
        }
        s.append(&self.gas_used);
        s.append(&self.log_bloom);
        s.append_list(&self.logs);
    }
}

impl Receipt {
    /// Create a new receipt.
    pub fn new(outcome: TransactionOutcome, gas_used: U256, logs: Vec<LogEntry>) -> Self {
        Self {
            gas_used,
            log_bloom: logs.iter().fold(Bloom::default(), |mut b, l| {
                b.accrue_bloom(&l.bloom());
                b
            }),
            logs,
            outcome,
        }
    }
}

impl Decodable for Receipt {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if rlp.item_count()? == 3 {
            Ok(Receipt {
                outcome: TransactionOutcome::Unknown,
                gas_used: rlp.val_at(0)?,
                log_bloom: rlp.val_at(1)?,
                logs: rlp.list_at(2)?,
            })
        } else {
            Ok(Receipt {
                gas_used: rlp.val_at(1)?,
                log_bloom: rlp.val_at(2)?,
                logs: rlp.list_at(3)?,
                outcome: {
                    let first = rlp.at(0)?;
                    if first.is_data() && first.data()?.len() <= 1 {
                        TransactionOutcome::StatusCode(first.as_val()?)
                    } else {
                        TransactionOutcome::StateRoot(first.as_val()?)
                    }
                },
            })
        }
    }
}




/// A record of execution for a `LOG` operation.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LogEntry {
    /// The address of the contract executing at the point of the `LOG` operation.
    pub address: Address,
    /// The topics associated with the `LOG` operation.
    pub topics: Vec<H256>,
    /// The data associated with the `LOG` operation.
    pub data: Bytes,
}

impl LogEntry {
    /// Calculates the bloom of this log entry.
    pub fn bloom(&self) -> Bloom {
        self.topics.iter().fold(
            Bloom::from(BloomInput::Raw(self.address.as_bytes())),
            |mut b, t| {
                b.accrue(BloomInput::Raw(t.as_bytes()));
                b
            },
        )
    }
}

impl Encodable for LogEntry {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(3);
        s.append(&self.address);
        s.append_list(&self.topics);
        s.append(&self.data);
    }
}

impl Decodable for LogEntry {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(LogEntry {
            address: rlp.val_at(0)?,
            topics: rlp.list_at(1)?,
            data: rlp.val_at(2)?,
        })
    }
}

use super::{TransactionReceipt as RpcReceipt, Block as RpcBlock, Log as RpcLog};

impl From<RpcBlock<H256>> for Header {
    fn from(block: RpcBlock<H256>) -> Header {
        Header {
            parent_hash: block.parent_hash,
            timestamp: block.timestamp.low_u64(),
            number: block.number.map(|v| v.low_u64()).unwrap_or(0),
            author: block.author,
            transactions_root: block.transactions_root,
            uncles_hash: block.uncles_hash,
            extra_data: block.extra_data.0,
            state_root: block.state_root,
            receipts_root: block.receipts_root,
            log_bloom: block.logs_bloom,
            gas_used: block.gas_used,
            gas_limit: block.gas_limit,
            difficulty: block.difficulty,
            seal: block.seal_fields.into_iter().map(|v|v.0).collect(),
            hash: None,
        }
    }
}

impl From<RpcReceipt> for Receipt {
    fn from(rpc: RpcReceipt) -> Receipt {
        let mut outcome= TransactionOutcome::Unknown;
        if let Some(state) = rpc.status {
            let status: u64 = state.low_u64();
            outcome = TransactionOutcome::StatusCode(status as u8);
        }
        if let Some(root) = rpc.root {
            outcome = TransactionOutcome::StateRoot(root);
        }
        Receipt {
            gas_used: rpc.gas_used.unwrap_or(U256::zero()),
            log_bloom: rpc.log_bloom,
            logs: rpc.logs.into_iter().map(|v| v.into()).collect(),
            outcome: outcome
        }
    }
}

impl From<RpcLog> for LogEntry {
    fn from(rpc: RpcLog) -> LogEntry {
        LogEntry {
            address: rpc.address,
            topics: rpc.topics,
            data: rpc.data.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_state_root() {
        let expected:Vec<u8> = ::rustc_hex::FromHex::from_hex("f9014183040caeb9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000f838f794dcf421d093428b096ca501a7cd1a740855a7976fc0a00000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let r = Receipt::new(
            TransactionOutcome::Unknown,
            0x40cae.into(),
            vec![LogEntry {
                address: Address::from_str("dcf421d093428b096ca501a7cd1a740855a7976f").unwrap(),
                topics: vec![],
                data: vec![0u8; 32],
            }],
        );
        assert_eq!(&::rlp::encode(&r)[..], &expected[..]);
    }

    #[test]
    fn test_basic() {
        let expected:Vec<u8> = ::rustc_hex::FromHex::from_hex("f90162a02f697d671e9ae4ee24a43c4b0d7e15f1cb4ba6de1561120d43b9a4e8c4a8a6ee83040caeb9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000f838f794dcf421d093428b096ca501a7cd1a740855a7976fc0a00000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let r = Receipt::new(
            TransactionOutcome::StateRoot(
                H256::from_str("2f697d671e9ae4ee24a43c4b0d7e15f1cb4ba6de1561120d43b9a4e8c4a8a6ee")
                    .unwrap(),
            ),
            0x40cae.into(),
            vec![LogEntry {
                address: Address::from_str("dcf421d093428b096ca501a7cd1a740855a7976f").unwrap(),
                topics: vec![],
                data: vec![0u8; 32],
            }],
        );
        let encoded = ::rlp::encode(&r);
        assert_eq!(&encoded[..], &expected[..]);
        let decoded: Receipt = ::rlp::decode(&encoded).expect("decoding receipt failed");
        assert_eq!(decoded, r);
    }

    #[test]
    fn test_status_code() {
        let expected:Vec<u8> = ::rustc_hex::FromHex::from_hex("f901428083040caeb9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000f838f794dcf421d093428b096ca501a7cd1a740855a7976fc0a00000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let r = Receipt::new(
            TransactionOutcome::StatusCode(0),
            0x40cae.into(),
            vec![LogEntry {
                address: Address::from_str("dcf421d093428b096ca501a7cd1a740855a7976f").unwrap(),
                topics: vec![],
                data: vec![0u8; 32],
            }],
        );
        let encoded: Vec<u8> = ::rlp::encode(&r);
        assert_eq!(&encoded[..], &expected[..]);
        let decoded: Receipt = ::rlp::decode(&encoded).expect("decoding receipt failed");
        assert_eq!(decoded, r);
    }
}