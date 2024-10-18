use crate::errors::Result;

pub trait IOManager: Sync + Send {
    fn read(&self, buf: &[u8], offset: u64) -> Result<usize>;
    fn write(&self, buf: &mut [u8], offet: u64) -> Result<usize>;
    fn sync(&self) -> Result<()>;
}
