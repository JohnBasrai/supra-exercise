use supra_embp::{spawn_processor, ingest_stream};
use futures::stream;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_ingest_and_process() {
    let queries = vec!["one".into(), "two".into(), "one".into()];
    let (tx, rx) = mpsc::channel(10);
    let query_stream = stream::iter(queries.clone());

    // Ingest items into the processor queue
    let ingest_task = tokio::spawn(ingest_stream(query_stream, tx));

    // Run the real processor
    let processor = spawn_processor();
    tokio::spawn(async move {
        while let Some(q) = rx.recv().await {
            processor.send(q).await.unwrap();
        }
    });

    ingest_task.await.unwrap().unwrap();
}
