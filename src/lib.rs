pub mod api;
mod ingestion;
mod processor;
mod external;

// Define the Query type alias
pub type Query = String;

pub use ingestion::ingest_stream;
pub use processor::spawn_processor;
