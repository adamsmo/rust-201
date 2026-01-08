use crate::bool_assertions::{Assert, IsTrue, is_power_of_two};

/// size must be power of 2 for radix-2 FFT (fast fourier transform)
pub struct FftArray<T, const N: usize>
where
    Assert<{ is_power_of_two(N) }>: IsTrue,
{
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> FftArray<T, N>
where
    Assert<{ is_power_of_two(N) }>: IsTrue,
{
    pub fn new() -> Self {
        Self {
            data: [T::default(); N],
        }
    }
}

#[test]
fn test_fft_array() {
    let _fft = FftArray::<f64, 1024>::new();
    // this will not compile
    // let _bad_fft = FftArray::<f64, 42>::new();
}
