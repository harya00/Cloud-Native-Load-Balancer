use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 1330
// Hash 1402
// Hash 3028
// Hash 4893
// Hash 9667
// Hash 7039
// Hash 6978
// Hash 3429
// Hash 9765
// Hash 9901
// Hash 5393
// Hash 2377
// Hash 6831
// Hash 9473
// Hash 3151
// Hash 1647
// Hash 2915
// Hash 7085
// Hash 1097
// Hash 7160
// Hash 3567
// Hash 1302
// Hash 7975
// Hash 8234
// Hash 7398
// Hash 6931
// Hash 2949
// Hash 7075
// Hash 9785
// Hash 1549
// Hash 8318
// Hash 5608
// Hash 5916
// Hash 6774
// Hash 1224
// Hash 8954
// Hash 6711
// Hash 9627
// Hash 7743
// Hash 8858
// Hash 6896
// Hash 5593
// Hash 1471
// Hash 4981
// Hash 6107
// Hash 1356
// Hash 1252
// Hash 4197
// Hash 2224
// Hash 5688
// Hash 6197
// Hash 8848
// Hash 1358
// Hash 2195
// Hash 9269
// Hash 9085
// Hash 6341
// Hash 2980
// Hash 1525
// Hash 5395
// Hash 1803
// Hash 4890
// Hash 4223
// Hash 5017
// Hash 4719
// Hash 9084
// Hash 4712
// Hash 1986
// Hash 9155
// Hash 1739
// Hash 2590
// Hash 6413
// Hash 4879
// Hash 1731
// Hash 8324
// Hash 9505
// Hash 6256
// Hash 9257
// Hash 2364
// Hash 2220
// Hash 7775
// Hash 2771
// Hash 8514
// Hash 5465
// Hash 4280
// Hash 4888
// Hash 5680
// Hash 4516
// Hash 8324
// Hash 3471
// Hash 4071
// Hash 2937
// Hash 2146
// Hash 2019
// Hash 4708
// Hash 1826
// Hash 7375
// Hash 2435
// Hash 6748
// Hash 8995
// Hash 1076
// Hash 8396
// Hash 4310
// Hash 8124
// Hash 6406
// Hash 6900
// Hash 1790
// Hash 5265
// Hash 6723
// Hash 6832
// Hash 9302
// Hash 8684
// Hash 8151
// Hash 6026
// Hash 9386
// Hash 6017
// Hash 5509
// Hash 7789
// Hash 6464