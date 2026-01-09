// it will never finish, i.e. server listening for requests
pub async fn eternal_listener() -> ! {
    loop {
        std::hint::black_box(());
    }
}

// A task that runs forever OR fails
pub async fn listener_with_errors() -> Result<!, String> {
    let mut count = 0;
    loop {
        count += 1;
        if count > 100 {
            return Err("too many iterations".to_string());
        }
    }
}

// A task that always succeeds but might take a while
// it is more convention, then explicitly comes from the type
pub async fn guaranteed_fetch(url: &str) -> Result<String, !> {
    // Retry forever until success
    loop {
        if let Some(result) = try_fetch(url) {
            return Ok(result);
        }
    }
}

fn try_fetch(_url: &str) -> Option<String> {
    Some("data".to_string())
}

#[tokio::test]
async fn test_async_task() {
    // if it returns it is Ok
    let Ok(data) = guaranteed_fetch("http://example.com").await;
    println!("Got data: {}", data);

    // if it returns, it's an error
    let Err(e) = listener_with_errors().await;
    println!("Listener failed: {}", e);
}
