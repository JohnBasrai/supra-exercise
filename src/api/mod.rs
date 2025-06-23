use async_trait::async_trait;

/// Trait for making async API calls.
///
/// Note: Currently `pub(crate)` but can be made `pub` later to allow user-defined logic.
#[async_trait]
pub(crate) trait ApiCaller: Send + Sync {
    async fn call(&self, query: &str) -> String;
}

mod client;
// Only need one export - make it public so tests can use it
pub use client::RealApi;