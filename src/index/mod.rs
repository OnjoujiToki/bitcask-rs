use crate::data::log_record::LogRecordPos;
pub trait Indexer: Sync + Send {
    // Sync means that the trait can be shared between multiple threads.
    // Send means that the trait can be sent between threads.
    // The put method inserts the key and the position of the log record into the index.
    fn put(&self, key: Vec<u8>, pos: LogRecordPos) -> bool;
    // The get method retrieves the position of the log record by the key.
    fn get(&self, key: Vec<u8>) -> Option<LogRecordPos>;
    // The delete method deletes the key and the position of the log record from the index.
    fn delete(&self, key: Vec<u8>) -> bool;
}

pub mod btree;
