use std::ops::Coroutine;
#[cfg(test)]
use std::ops::CoroutineState;
#[cfg(test)]
use std::pin::Pin;

/// those coroutines never return, as they have infinite loops in them,
/// it is fine as at each step of the loop there is yield

pub fn outlier_detector_coroutine(
    threshold: f64,
) -> impl Coroutine<(f64, f64), Yield = Option<f64>, Return = ()> {
    #[coroutine]
    // this will be argument passed to resume before any yield was handled
    move |mut input: (f64, f64)| {
        loop {
            let (value, average) = input;
            let deviation = (value - average).abs();

            let result = if deviation > threshold {
                Some(value)
            } else {
                None
            };
            // this actually updates input to be the argument passed to subsequent resume() call
            // so it is yield the result and assigning arguments from the next call to resume to input
            input = yield result;
        }
    }
}

pub fn moving_average_coroutine(
    window_size: usize,
) -> impl Coroutine<f64, Yield = f64, Return = ()> {
    #[coroutine]
    // this will be argument passed to resume before any yield was handled
    move |mut value: f64| {
        let mut window: Vec<f64> = Vec::with_capacity(window_size);
        let mut sum: f64 = 0.0;

        loop {
            sum += value;
            window.push(value);

            if window.len() > window_size {
                sum -= window.remove(0);
            }

            let average = sum / window.len() as f64;
            // this actually updates input to be the argument passed to subsequent resume() call
            // so it is yield the average and assigning arguments from the next call to resume to value
            value = yield average;
        }
    }
}

#[test]
fn test_outlier_detector() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 10.0, 5.0, 6.0, 20.0, 7.0, 8.0];
    let window_size = 3;
    let outlier_threshold = 5.0;

    let mut mov_avg = moving_average_coroutine(window_size);
    let mut outlier_detector = outlier_detector_coroutine(outlier_threshold);

    for &value in &data {
        let avg = match Pin::new(&mut mov_avg).resume(value) {
            CoroutineState::Yielded(a) => a,
            CoroutineState::Complete(()) => break,
        };

        let outlier = match Pin::new(&mut outlier_detector).resume((value, avg)) {
            CoroutineState::Yielded(o) => o,
            CoroutineState::Complete(()) => break,
        };

        println!(
            "Value: {:5.1}, Moving Avg: {:5.2}, is outlier: {:?}",
            value, avg, outlier
        );
    }
}
