use almost_zk::{
    R1CS,
    Witness,
    ec_dot_pdt,
    hadamard_pairing,
    check_equality_discrete_logs,
    to_field_elements
};
use ark_bn254::{
    Fr as Bn254Fr, 
    G1Projective as G1, 
    G2Projective as G2
};
use ark_ec::PrimeGroup;

/// Constructs the R1CS for `z = 2x^{3} + 4xy^{2} - xy + 5`.
/// The constrainst are:
/// v1 = x * x;
/// v2 = 2 * v1 * x;
/// v3 = y * y;
/// v4 = 4 * x * v3;
/// 5 - z + v2 + v4 = x * y;
fn build_r1cs() -> R1CS {
    let l = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 2, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 4, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 0]
    ];

    let r = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0]
    ];

    let o = vec![
        vec![0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1],
        vec![5, -1, 0, 0, 0, 1, 0, 1]
    ];

    R1CS::new(l, r, o)
}

/// Constructs the witness vector: [1, z, x, y, v1, v2, v3, v4]
fn build_witness(x: i64, y: i64) -> Witness {
    let v1 = x * x;
    let v2 = 2 * v1 * x;
    let v3 = y * y;
    let v4 = 4 * x * v3;
    let z = v2 + v4 - (x * y) + 5;

    Witness::new(vec![1, z, x, y, v1, v2, v3, v4])
}

// Prove knowledge of x and y such that z = 2x^{3} + 4xy^{2} - xy + 5
fn main() {
    let x: i64 = 3;
    let y: i64 = 2;

    let r1cs = build_r1cs();

    let witness = build_witness(x, y);
    let a_as_fr = to_field_elements::<Bn254Fr>(&witness.a);

    // Witness as G1 points
    let a_g1: Vec<G1> = a_as_fr
        .iter()
        .map(|a| G1::generator() * a).collect();

    // Witness as G2 points
    let a_g2: Vec<G2> = a_as_fr
        .iter()
        .map(|a| G2::generator() * a)
        .collect();

    
    // Assert that the discrete logs are the same
    assert!(
        check_equality_discrete_logs(&a_g1, &a_g2), 
        "Discrete logs not equal"
    );

    // LaG1
    let l_a_g1 = ec_dot_pdt(
        &r1cs.l,
        &a_g1
    );

    // RaG2
    let r_a_g2 = ec_dot_pdt(
        &r1cs.r,
        &a_g2
    );

    // OaG1
    let o_a_g1 = ec_dot_pdt(
        &r1cs.o,
        &a_g1
    );

    // G2 only
    let g2_only = vec![G2::generator(); o_a_g1.len()];
    
    // e(LaG1, RaG2)
    let lhs_g12 = hadamard_pairing(&l_a_g1, &r_a_g2);
    
    // e(OaG1, G2)
    let rhs_g12 = hadamard_pairing(&o_a_g1, &g2_only);

    // Compare each pair of G12 field elements
    let is_equal = lhs_g12 == rhs_g12;

    println!("{}", is_equal);
}