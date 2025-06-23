use super::ApiCaller;
use crate::external;
use async_trait::async_trait;

/// Real implementation that calls the external API.
pub struct RealApi;

#[async_trait]
impl ApiCaller for RealApi {
    async fn call(&self, query: &str) -> String {
        external::api_call(query).await
    }
}
