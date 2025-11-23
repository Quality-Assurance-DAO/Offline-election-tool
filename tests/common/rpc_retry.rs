//! RPC retry logic with exponential backoff

use std::time::Duration;
use tokio::time::sleep;

/// Retry a function with exponential backoff
/// 
/// # Arguments
/// * `func` - Async function to retry (must be FnMut to allow multiple calls)
/// * `max_attempts` - Maximum number of retry attempts (default: 3)
/// 
/// # Returns
/// Result from the function if successful, or error after all retries exhausted
pub async fn retry_with_backoff<F, Fut, T, E>(
    mut func: F,
    max_attempts: usize,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut last_error: Option<E> = None;
    
    for attempt in 1..=max_attempts {
        match func().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_attempts {
                    // Exponential backoff: 1s, 2s, 4s
                    let delay_seconds = 2_u64.pow((attempt - 1) as u32);
                    eprintln!("RPC call failed, retrying in {}s (attempt {}/{})", delay_seconds, attempt, max_attempts);
                    sleep(Duration::from_secs(delay_seconds)).await;
                }
            }
        }
    }
    
    // All retries exhausted - return last error
    Err(last_error.expect("Should have at least one error after retries"))
}

/// Retry a function with exponential backoff (default 3 attempts)
pub async fn retry_with_backoff_default<F, Fut, T, E>(func: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    retry_with_backoff(func, 3).await
}

