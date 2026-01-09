use std::ops::{Add, Mul};

#[derive(Clone, Debug)]
pub struct Matrix<T, const R: usize, const C: usize> {
    data: [[T; C]; R],
}

impl<T: Default + Copy, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn zeros() -> Self {
        Self {
            data: [[T::default(); C]; R],
        }
    }

    pub fn from_data(data: [[T; C]; R]) -> Self {
        Self { data }
    }
}

/// Matrix transpose: (R, C) -> (C, R)
impl<T: Default + Copy, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn transpose(&self) -> Matrix<T, C, R> {
        let mut result = Matrix::<T, C, R>::zeros();
        for i in 0..R {
            for j in 0..C {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
}

/// Matrix multiplication: (R, M) * (M, C) -> (R, C)
impl<T, const R: usize, const M: usize> Matrix<T, R, M>
where
    T: Default + Copy + Mul<Output = T> + Add<Output = T>,
{
    pub fn matmul<const C: usize>(&self, other: &Matrix<T, M, C>) -> Matrix<T, R, C> {
        let mut result = Matrix::<T, R, C>::zeros();
        for i in 0..R {
            for j in 0..C {
                let mut sum = T::default();
                for k in 0..M {
                    sum = sum + self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }
}

/// Concatenate matrices horizontally: (R, C1) + (R, C2) -> (R, C1+C2)
impl<T: Default + Copy, const R: usize, const C1: usize> Matrix<T, R, C1> {
    pub fn horizontal_concat<const C2: usize>(
        &self,
        other: &Matrix<T, R, C2>,
    ) -> Matrix<T, R, { C1 + C2 }>
    where
        [(); C1 + C2]:,
    {
        let mut result = Matrix::<T, R, { C1 + C2 }>::zeros();
        for i in 0..R {
            for j in 0..C1 {
                result.data[i][j] = self.data[i][j];
            }
            for j in 0..C2 {
                result.data[i][C1 + j] = other.data[i][j];
            }
        }
        result
    }
}

/// Concatenate matrices vertically: (R1, C) + (R2, C) -> (R1+R2, C)
impl<T: Default + Copy, const R1: usize, const C: usize> Matrix<T, R1, C> {
    pub fn vertical_concat<const R2: usize>(
        &self,
        other: &Matrix<T, R2, C>,
    ) -> Matrix<T, { R1 + R2 }, C>
    where
        [(); R1 + R2]:,
    {
        let mut result = Matrix::<T, { R1 + R2 }, C>::zeros();
        for i in 0..R1 {
            result.data[i] = self.data[i];
        }
        for i in 0..R2 {
            result.data[R1 + i] = other.data[i];
        }
        result
    }
}

#[test]
fn test_matrix_ops() {
    let a: Matrix<i32, 2, 3> = Matrix::from_data([[1, 2, 3], [4, 5, 6]]);
    let b: Matrix<i32, 3, 2> = Matrix::from_data([[1, 2], [3, 4], [5, 6]]);

    // Result is 2x2
    let c: Matrix<i32, 2, 2> = a.matmul(&b);

    // Transpose: 2x3 -> 3x2
    let _at: Matrix<i32, 3, 2> = a.transpose();

    // Horizontal concat: 2x3 + 2x2 -> 2x5
    let _wide: Matrix<i32, 2, 5> = a.horizontal_concat(&c);

    // This does not compile
    // let _bad_mul = a.matmul(&a);
    // let _bad_concat = a.horizontal_concat(&b);
}
