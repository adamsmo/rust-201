#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod array_split;
mod field_elements;

pub use array_split::split;
pub use field_elements::{FieldElement, Fp255, Fp256, Fp381};
