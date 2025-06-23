use crate::Query;
use anyhow::Result;
use futures::Stream;
use futures::StreamExt;
use tokio::sync::mpsc;

/// Consumes a stream of queries and forwards them into the processor channel.
pub async fn ingest_stream<S>(mut stream: S, sender: mpsc::Sender<Query>) -> Result<()>
where
    S: Stream<Item = Query> + Unpin,
{
    while let Some(query) = stream.next().await {
        sender.send(query).await?;
    }
    Ok(())
}
