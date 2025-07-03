use ark_bn254::Fr as Bn254Fr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R1CS {
    pub o: Vec<Vec<i64>>,
    pub l: Vec<Vec<i64>>,
    pub r: Vec<Vec<i64>>,
}

#[derive(Debug, Clone)]
pub struct Witness {
    pub a: Vec<Bn254Fr>, // A vector of BN254 field elements
}

impl R1CS {

}

impl Witness {

}

