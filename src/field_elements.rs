use ark_ff::{PrimeField};

/// Converts an array of integers to field elements.
/// # Returns
/// A vector of generic type implementing the `PrimeField` trait
pub fn to_field_elements<Fp: PrimeField>(
    constraint: &[i64],
) -> Vec<Fp> {
    let mut res: Vec<Fp> = Vec::new();

    for &entry in constraint {
        // convert magnitude as field element
        let abs_as_fe = Fp::from(entry.unsigned_abs() as u128);

        if entry < 0 {
            // negating computes: -abs_as_fe mod p (equivalent to p - abs_as_fe)
            let negated = abs_as_fe.neg();
            res.push(negated);
        } else {
            res.push(abs_as_fe);
        }
    }
    res
}