use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

pub fn fibonacci_coroutine(count: usize) -> impl Coroutine<Yield = u64, Return = &'static str> {
    #[coroutine]
    move || {
        let mut a: u64 = 0;
        let mut b: u64 = 1;

        for _ in 0..count {
            yield a;
            let next = a + b;
            a = b;
            b = next;
        }

        "Fibonacci sequence complete"
    }
}

#[test]
fn demo() {
    let mut coro_fib = fibonacci_coroutine(10);
    let mut coro_result = Vec::new();

    loop {
        // using Pin as it required by resume() from Coroutine trait
        match Pin::new(&mut coro_fib).resume(()) {
            CoroutineState::Yielded(value) => coro_result.push(value),
            CoroutineState::Complete(msg) => {
                println!("  {:?}", coro_result);
                println!("  Completion message: {}", msg);
                break;
            }
        }
    }
}
