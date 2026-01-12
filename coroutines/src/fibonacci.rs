use std::ops::Coroutine;
#[cfg(test)]
use std::ops::CoroutineState;
#[cfg(test)]
use std::pin::Pin;

pub fn fibonacci_coroutine(count: usize) -> impl Coroutine<Yield = u64, Return = &'static str> {
    #[coroutine]
    move || {
        let mut a: u64 = 0;
        let mut b: u64 = 1;

        for _ in 0..count {
            yield a; // <- this is suspension point that is handled with Yielded enum value
            let next = a + b; // <- executions resumes here after calling resume(())
            a = b;
            b = next;
        }

        "Fibonacci sequence complete" // <- this is return instruction, it is handled with Complete enum value
    }
}

#[test]
fn demo() {
    let mut coro_fib = fibonacci_coroutine(5);
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

    // internal coroutine state at each resume() call + what is returned
    // resume()  resume()  resume()  resume()  resume()  resume()
    //    |         |         |         |         |         |
    // --------  --------  --------  --------  --------  --------
    // | a=0  |  | a=1  |  | a=1  |  | a=2  |  | a=3  |  | END  |
    // | b=1  |  | b=1  |  | b=2  |  | b=3  |  | b=5  |  |      |
    // --------  --------  --------  --------  --------  --------
    //    |         |         |         |         |         |
    // Yield(0)  Yield(1)  Yield(1)  Yield(2)  Yield(3) Complete("Fib...")
}
