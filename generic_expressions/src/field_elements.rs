pub struct FieldElement<const BITS: usize>
where
    // this bound is required to force compiler to check if array of size (BITS + 63) / 64 actually exists
    // i.e. (BITS + 63) / 64 does not evaluate to negative number
    // it is called well-formedness bound
    [(); (BITS + 63) / 64]:,
{
    limbs: [u64; (BITS + 63) / 64],
}

impl<const BITS: usize> FieldElement<BITS>
where
    [(); (BITS + 63) / 64]:,
{
    pub const LIMBS: usize = (BITS + 63) / 64;

    pub fn zero() -> Self {
        Self {
            limbs: [0u64; (BITS + 63) / 64],
        }
    }

    pub fn from_limbs(limbs: [u64; (BITS + 63) / 64]) -> Self {
        Self { limbs }
    }

    pub fn limbs(&self) -> &[u64; (BITS + 63) / 64] {
        &self.limbs
    }
}

// Type aliases for common field sizes in cryptography
pub type Fp256 = FieldElement<256>; // 4 limbs. i.e. BN254, secp256k1
pub type Fp381 = FieldElement<381>; // 6 limbs. i.e. BLS12-381
pub type Fp255 = FieldElement<255>; // 4 limbs. i.e. Curve25519

#[test]
fn test_field_elements() {
    // Limbs automatically computed!
    let _fp256 = Fp256::zero();
    assert_eq!(Fp256::LIMBS, 4);

    let _fp381 = Fp381::zero();
    assert_eq!(Fp381::LIMBS, 6);

    let _fp255 = Fp255::zero();
    assert_eq!(Fp255::LIMBS, 4);

    Fp255::from_limbs([0, 0, 0, 0]);
    // this will not compile
    // Fp255::from_limbs([0, 0, 0, 0, 0, 0]);
    // but this will
    Fp381::from_limbs([0, 0, 0, 0, 0, 0]);
}
