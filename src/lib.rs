pub mod api;
mod external;
mod ingestion;
mod processor;

// Define the Query type alias
pub type Query = String;

pub use ingestion::ingest_stream;
pub use processor::spawn_processor;
