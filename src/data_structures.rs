use ark_bn254::Fr as Bn254Fr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R1CS {
    pub l: Vec<Vec<i64>>,
    pub r: Vec<Vec<i64>>,
    pub o: Vec<Vec<i64>>,
}

#[derive(Debug, Clone)]
pub struct Witness {
    /// A vector of BN254 field elements
    pub a: Vec<Bn254Fr>,
}

impl R1CS {
    /// Creates a new R1CS instance
    pub fn new(l: Vec<Vec<i64>>, r: Vec<Vec<i64>>, o: Vec<Vec<i64>>) -> Self {
        Self { l, r, o }
    }
}

impl Witness {
    /// Creates a new Witness instance
    pub fn new(a: Vec<Bn254Fr>) -> Self {
        Self { a }
    }
}

