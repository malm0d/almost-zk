pub mod data_structures;
pub mod field_elements;
pub mod elliptic_curve;

//re-export for clean imports in main.rs
pub use data_structures::{R1CS, Witness};
pub use field_elements::to_field_elements;
pub use elliptic_curve::{
    ec_dot_pdt,
    hadamard_pairing,
    check_equality_discrete_logs
};