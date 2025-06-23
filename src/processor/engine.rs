use crate::api::ApiCaller;
use crate::Query;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;

// Generic processor that accepts any type implementing ApiCaller
pub fn spawn_processor<T>(api: Arc<T>) -> mpsc::Sender<Query>
where
    T: ApiCaller + 'static,
{
    let (tx, rx) = mpsc::channel(1000);
    tokio::spawn(async move {
        if let Err(e) = process(rx, api).await {
            eprintln!("Processor task failed: {:?}", e);
        }
    });

    tx
}

// Internal processing function
pub(crate) async fn process<T>(mut rx: mpsc::Receiver<Query>, api: Arc<T>) -> Result<()>
where
    T: ApiCaller,
{
    let mut cache: HashMap<Query, String> = HashMap::new();

    while let Some(query) = rx.recv().await {
        if let Some(result) = cache.get(&query) {
            println!("Cache hit for '{}': {}", query, result);
            continue;
        }

        let response = api.call(&query).await;
        println!("API call for '{}': {}", query, response);
        cache.insert(query, response);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::Stream;
    use std::collections::VecDeque;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};

    struct MockApi {
        seen: Arc<Mutex<Vec<String>>>,
    }

    #[async_trait::async_trait]
    impl ApiCaller for MockApi {
        async fn call(&self, query: &str) -> String {
            self.seen.lock().unwrap().push(query.to_string());
            format!("mocked: {}", query)
        }
    }

    struct MockQueryStream {
        items: VecDeque<String>,
    }

    impl MockQueryStream {
        fn new_with_duplicates() -> Self {
            let items = VecDeque::from(vec![
                "a".into(),
                "b".into(),
                "a".into(),
                "c".into(),
                "b".into(),
            ]);
            Self { items }
        }
    }

    impl Stream for MockQueryStream {
        type Item = String;

        fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(self.items.pop_front())
        }
    }

    #[tokio::test]
    async fn test_processor_with_mock_api_and_duplicates() -> Result<()> {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let api = Arc::new(MockApi { seen: seen.clone() });

        let tx = spawn_processor(api);
        let mock_stream = MockQueryStream::new_with_duplicates();
        crate::ingestion::ingest_stream(mock_stream, tx).await?;

        // Give the processor a moment to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let seen = seen.lock().unwrap();
        assert_eq!(seen.len(), 3); // a, b, c
        Ok(())
    }
}
