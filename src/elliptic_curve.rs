use ark_bn254::{Bn254, Fr as Bn254Fr, G1Projective as G1, G2Projective as G2};
use ark_ec::{pairing::{Pairing, PairingOutput}, CurveGroup};
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
pub fn ec_dot_pdt<Ecg: CurveGroup>(
    constraint_matrix: &[Vec<i64>],
    encrypted_witness: &[Ecg],
) -> Vec<Ecg> {
    let mut res: Vec<Ecg> = Vec::new();

    for constraint in constraint_matrix {
        let mut ec_point_aggregator = Ecg::zero();

        // Convert each coefficient of the constraint into the scalar field of the curve.
        // This ensures all scalar multiplications use the correct field modulus.
        // For BN254, this will use Bn254Fr when `Ecg` is G1 or G2.
        // `ScalarField` implements the `PrimeField` trait
        let coeff_as_fe: Vec<Ecg::ScalarField> =
            constraint_to_field_elements::<Ecg::ScalarField>(constraint);

        for (scalar, witness_ec_point) in coeff_as_fe.into_iter().zip(encrypted_witness) {
            ec_point_aggregator += witness_ec_point.mul(scalar);
        }

        res.push(ec_point_aggregator);
    }
    res
}
/// Computes the element-wise bilinear pairing between a vector of EC points in G1 and
/// a vector of EC points in G2, implementing the equivalent of a Hadamard product.
/// 
/// The returning vector of bilinear pairings:
/// ```math
/// [ e(L_1·a·G1, R_1·a·G2), e(L_2·a·G1, R_2·a·G2), ... e(L_n·a·G1, R_n·a·G2) ]
/// ```
/// Which, by bilinearity means:
/// ```math
/// [ e(G1, G2)^{(L1·a)(R1·1)}, e(G1, G2)^{(L2·a)(R2·1)}, ... e(G1, G2)^{(Ln·a)(Rn·1)} ]
/// ```
/// 
/// # Arguments
/// - `l_a_G1`: Vector of G1 points representing the left-hand terms of constraints (L·a·G1)
/// - `r_a_G2`: Vector of G2 points representing the right-hand terms of constraints (R·a·G2)
///
/// # Returns
/// A vector of `PairingOutput<Bn254>` elements (in GT/Fq12), where each element is:
/// ```text
/// e(G1, G2)^{(L1·a)(R2·a)}
/// ```
pub fn hadamard_pairing(
    l_a_g1: &[G1],
    r_a_g2: &[G2]
) -> Vec<PairingOutput<Bn254>> {
    l_a_g1
        .iter()
        .zip(r_a_g2.iter())
        .map(|(g1_point, g2_point)| 
            Bn254::pairing(g1_point, g2_point)
        )
        .collect()
}