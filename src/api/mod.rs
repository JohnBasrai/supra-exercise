use async_trait::async_trait;

/// Trait for making async API calls.
#[async_trait]
pub trait ApiCaller: Send + Sync {
    async fn call(&self, query: &str) -> String;
}

mod client;
// Only need one export - make it public so tests can use it
pub use client::RealApi;
