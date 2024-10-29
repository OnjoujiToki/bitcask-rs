use std::{
    fs::{File, OpenOptions},
    os::unix::prelude::FileExt,
    path::PathBuf,
    sync::Arc,
};

use crate::errors::{Errors, Result};

use log::error;

use std::{io::Read, io::Write};

use parking_lot::RwLock;

use super::IOManager;

// File IO module
// This module contains the FileIO struct and its implementation.
pub struct FileIO {
    fd: Arc<RwLock<File>>, // file descriptor
}

impl FileIO {
    pub fn new(filename: PathBuf) -> Result<Self> {
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(filename)
        {
            Ok(file) => {
                return Ok(FileIO {
                    fd: Arc::new(RwLock::new(file)),
                });
            }
            Err(e) => {
                error!("Failed to open file: {}", e);
                return Err(Errors::FailedToSyncDataFile);
            }
        }
    }
}

impl IOManager for FileIO {
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        let rw_lock_read_guard = self.fd.read();
        if let Ok(n_bytes) = rw_lock_read_guard.read_at(buf, offset) {
            Ok(n_bytes)
        } else {
            Err(Errors::FailedToReadFromDataFile)
        }
    }

    fn write(&self, buf: &[u8]) -> Result<usize> {
        let mut write_guard = self.fd.write();
        if let Err(e) = write_guard.write(buf) {
            error!("Failed to write to file: {}", e);
            return Err(Errors::FailedToWriteToDataFile);
        }
        Ok(buf.len())
    }

    fn sync(&self) -> Result<()> {
        let read_guard = self.fd.read();
        if let Err(e) = read_guard.sync_all() {
            error!("Failed to sync file: {}", e);
            return Err(Errors::FailedToSyncDataFile);
        }
        Ok(())
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_file_io_read() {
        let path = PathBuf::from("test_file_io_write.txt");
        let fio_res = FileIO::new(path.clone());
        assert!(fio_res.is_ok());
        let fio = fio_res.ok().unwrap();
        let res1 = fio.write("key-a".as_bytes());
        assert!(res1.is_ok());
        assert_eq!(res1.ok().unwrap(), 5);
        // The write method is expected to return the number of bytes written.
        let res2 = fio.write("key-b".as_bytes());
        assert!(res2.is_ok());
        assert_eq!(res2.ok().unwrap(), 5);
        let mut buf = [0u8; 5];
        let res3 = fio.read(&mut buf, 5);
        assert!(res3.is_ok());
        assert_eq!(res3.ok().unwrap(), 5);
    }
}
