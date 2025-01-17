#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogRecordPos {
    pub(crate) file_id: u32,
    pub(crate) offset: u64,
}
