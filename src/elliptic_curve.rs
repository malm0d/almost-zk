use ark_bn254::{Bn254, G1Projective as G1, G2Projective as G2};
use ark_ec::{pairing::{Pairing, PairingOutput}, CurveGroup, PrimeGroup};

use crate::field_elements::to_field_elements;

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
            to_field_elements::<Ecg::ScalarField>(constraint);

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
/// - `ec_a_g1`: Vector of G1 points
/// - `ec_a_g2`: Vector of G2 points
///
/// # Returns
/// A vector of G_12 elements `PairingOutput<Bn254>`
pub fn hadamard_pairing(
    ec_a_g1: &[G1],
    ec_a_g2: &[G2]
) -> Vec<PairingOutput<Bn254>> {
    ec_a_g1.iter()
        .zip(ec_a_g2.iter())
        .map(|(g1_point, g2_point)| 
            Bn254::pairing(g1_point, g2_point)
        )
        .collect()
}

/// Because of bilinearity, we can verify that the two vectors of EC point (one in G1,
/// one in G2) have the same discrete logarithms without knowing the discrete logs themselves.
/// That is, we can check that the vectors were encrypted with the same scalars (witness).
/// 
/// Given vectors `a_g1 = [a1G1, a2G1, ... anG1]` and `a_g2 = [a1G2, a2G2, ... anG2]`,
/// check that for each pair (anG1, anG2): e(anG1, G2) == e(G1, anG2) for all n.
/// 
/// This equality holds because of bilinearity:
/// ```math
/// e(anG1, G2) = e(G1, G2)^{an} = e(G1, anG2)
/// ```
pub fn check_equality_discrete_logs(
    a_g1: &[G1],
    a_g2: &[G2]
) -> bool {
    let g1_only: G1 = G1::generator();
    let g2_only: G2 = G2::generator();

    a_g1.iter()
        .zip(a_g2.iter())
        .all(|(g1_point, g2_point)| {
            let lhs = Bn254::pairing(g1_point, &g2_only);
            let rhs = Bn254::pairing(&g1_only, g2_point);
            lhs == rhs
        })
}