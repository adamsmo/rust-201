/// Polynomial of degree N has exactly N+1 coefficients
pub struct Polynomial<T, const DEGREE: usize>
where
    [(); DEGREE + 1]:,
{
    coefficients: [T; DEGREE + 1],
}

impl<T: Default + Copy, const DEGREE: usize> Polynomial<T, DEGREE>
where
    [(); DEGREE + 1]:,
{
    pub fn zero() -> Self {
        Self {
            coefficients: [T::default(); DEGREE + 1],
        }
    }

    pub fn from_coefficients(coefficients: [T; DEGREE + 1]) -> Self {
        Self { coefficients }
    }

    pub fn degree(&self) -> usize {
        DEGREE
    }

    pub fn num_coefficients(&self) -> usize {
        DEGREE + 1
    }
}

/// Multiply two polynomials - degree of result is sum of degrees
impl<T, const D1: usize> Polynomial<T, D1>
where
    T: Default + Copy + std::ops::Mul<Output = T> + std::ops::AddAssign,
    [(); D1 + 1]:,
{
    pub fn mul<const D2: usize>(&self, other: &Polynomial<T, D2>) -> Polynomial<T, { D1 + D2 }>
    where
        [(); D2 + 1]:,
        [(); D1 + D2 + 1]:,
    {
        let mut result = [T::default(); D1 + D2 + 1];
        for i in 0..=D1 {
            for j in 0..=D2 {
                result[i + j] += self.coefficients[i] * other.coefficients[j];
            }
        }
        // if you do not use turbo fish here, compiler will fail
        Polynomial::<T, { D1 + D2 }>::from_coefficients(result)

        // this will blow up, it is still experimental feature
        // Polynomial::from_coefficients(result)
    }
}

#[test]
fn test_polynomial_multiplication() {
    // (1 + 2x) * (3 + 4x) = 3 + 10x + 8x²
    let p1 = Polynomial::<i32, 1>::from_coefficients([1, 2]);
    let p2 = Polynomial::<i32, 1>::from_coefficients([3, 4]);
    let product = p1.mul(&p2);

    assert_eq!(product.degree(), 2);
    assert_eq!(product.num_coefficients(), 3);
}
