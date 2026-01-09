use std::time::Duration;

/// simple retry logic to show passing async closure
pub async fn retry_operation<F, T, E>(
    mut operation: F,
    max_retries: u32,
    delay: Duration,
) -> Vec<Result<T, E>>
where
    F: AsyncFnMut() -> Result<T, E>,
    E: std::fmt::Debug,
    T: std::fmt::Debug,
{
    let mut results = vec![];
    for attempt in 0..max_retries {
        match operation().await {
            value @ Ok(_) => {
                results.push(value);
                return results;
            }
            e @ Err(_) => {
                println!("Attempt {} failed: {:?}", attempt + 1, e);
                results.push(e);
                tokio::time::sleep(delay).await;
            }
        }
    }
    results
}

#[tokio::test]
async fn test_async_closure() {
    // closures are executed sequentially so no need for synchronizations / atomic stuff
    let mut for_capture = 1;
    let results = retry_operation(
        async || {
            println!("got for_capture from the scope {for_capture}");
            if for_capture > 3 {
                Ok(42)
            } else {
                for_capture += 1;
                Err("ups, failed")
            }
        },
        4,
        Duration::from_secs(3),
    )
    .await;
    println!("got those results: {:?}", results);
}
