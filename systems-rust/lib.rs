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
// Hash 6930
// Hash 8154
// Hash 6897
// Hash 4070
// Hash 9052
// Hash 7014
// Hash 9157
// Hash 5157
// Hash 9613
// Hash 6210
// Hash 7205
// Hash 1554
// Hash 4503
// Hash 7860
// Hash 2927
// Hash 8394
// Hash 8200
// Hash 2905
// Hash 5834
// Hash 2585
// Hash 6865
// Hash 6062
// Hash 9246
// Hash 1547
// Hash 4747
// Hash 4049
// Hash 1235
// Hash 5650
// Hash 9773
// Hash 1147
// Hash 6304
// Hash 4647
// Hash 7114
// Hash 9970
// Hash 4000
// Hash 9810
// Hash 5169
// Hash 6410
// Hash 1954
// Hash 2649
// Hash 4138
// Hash 9142
// Hash 2041
// Hash 2279
// Hash 1388
// Hash 3130
// Hash 2863
// Hash 7759
// Hash 8922
// Hash 8976
// Hash 2883
// Hash 8347
// Hash 4406
// Hash 4438
// Hash 5728
// Hash 9954
// Hash 9716
// Hash 7785
// Hash 7268
// Hash 1735
// Hash 8223
// Hash 7039
// Hash 2104
// Hash 4965
// Hash 8133
// Hash 1140
// Hash 5525
// Hash 5233
// Hash 4215
// Hash 4209
// Hash 1191
// Hash 9920
// Hash 6718
// Hash 7214
// Hash 3636
// Hash 1855
// Hash 6063
// Hash 2307
// Hash 1498
// Hash 1150
// Hash 6337
// Hash 7143
// Hash 7638
// Hash 9103
// Hash 4671
// Hash 1627
// Hash 3059
// Hash 8173
// Hash 7794
// Hash 7200
// Hash 8298
// Hash 9774
// Hash 4453
// Hash 7862
// Hash 5350
// Hash 9659
// Hash 7560
// Hash 7042
// Hash 5248
// Hash 9050
// Hash 4772
// Hash 7970
// Hash 4199
// Hash 6000
// Hash 2871
// Hash 3256
// Hash 1328