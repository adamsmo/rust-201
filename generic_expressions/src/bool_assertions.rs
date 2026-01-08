/// boilerplate for encoding boolean expression
pub struct Assert<const COND: bool>;
pub trait IsTrue {}
/// implement IsTrue only for assertions with COND = true
impl IsTrue for Assert<true> {}

/// checks if number is power of 2
pub const fn is_power_of_two(n: usize) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

pub const fn log2(n: usize) -> usize {
    usize::BITS as usize - 1 - n.leading_zeros() as usize
}
