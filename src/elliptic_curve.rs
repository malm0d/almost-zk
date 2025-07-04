use ark_bn254::{Bn254, Fr as Bn254Fr, G1Projective as G1, G2Projective as G2};
use ark_ec::{pairing::Pairing, CurveGroup};
use ark_ff::{PrimeField, Field};
use ark_std::{One, Zero};

use crate::field_elements::constraint_to_field_elements;

/// Computes the dot product between the constraint matrix and the witness vector,
/// which is equivalent to elliptic curve point multiplications and additions. \
/// I.e., computes La, Ra, or Oa (depending on the constraint matrix), where `a` is
/// the encrypted witness vector (in G1 or G2).
/// 
/// # Arguments
/// - `constraint_matrix`: A matrix of `i64` coefficients (e.g. `Vec<Vec<i64>`) defining 
/// the constraints in the R1CS.
/// - `encrypted_witness`: A slice of EC points (`Ecg`) representing the witness.
/// 
/// # Returns
/// A vector of EC points `Vec<Ecg>` representing the result of the dot product.
fn ec_dot_pdt<Ecg: CurveGroup>(
    constraint_matrix: &[Vec<i64>],
    encrypted_witness: &[Ecg],
) -> Vec<Ecg> {
    let mut res: Vec<Ecg> = Vec::new();

    for constraint in constraint_matrix {
        let mut ec_point_aggregator = Ecg::zero();

        // Convert each coefficient of the constraint into the scalar field of the curve.
        // This ensures all scalar multiplications use the correct field modulus.
        // For BN254, this will use Bn254Fr when `Ecg` is G1 or G2.
        let coeff_as_fe: Vec<Ecg::ScalarField> =
            constraint_to_field_elements::<Ecg::ScalarField>(constraint);

        for (scalar, witness_ec_point) in coeff_as_fe.into_iter().zip(encrypted_witness) {
            ec_point_aggregator += witness_ec_point.mul(scalar);
        }

        res.push(ec_point_aggregator);
    }
    res
}