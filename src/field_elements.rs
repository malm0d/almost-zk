use ark_bn254::{Bn254, Fr as Bn254Fr, G1Affine, G2Affine};
use ark_ec::{pairing::Pairing, CurveGroup};
use ark_ff::{PrimeField, Field};

/// Converts integer constraints to BN254 field elements. \
/// Includes sign tracking to indicate necessity for negation with negative integers.
fn constraint_to_field_elements(constraint: &[i64]) -> Vec<(Bn254Fr, bool)> {
    let mut res: Vec<(Bn254Fr, bool)> = Vec::new();
    let p = Bn254Fr::from(Bn254Fr::MODULUS);

    for &entry in constraint {
        if entry < 0 {
            // (-x) mod p = (p - x) mod p
            let absolute_value = Bn254Fr::from(entry.unsigned_abs());
            let neg_equivalent = p - absolute_value;
            res.push((neg_equivalent, true));
        } else {
            res.push((Bn254Fr::from(entry), false));
        }
    }

    res
}