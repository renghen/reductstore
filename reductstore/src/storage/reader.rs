// Copyright 2023 ReductStore
// Licensed under the Business Source License 1.1

use crate::storage::proto::{ts_to_us, Block};
use reduct_base::error::ReductError;

use bytes::Bytes;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

pub struct RecordReader {
    file: File,
    written_bytes: u64,
    content_length: u64,
    chunk_size: u64,
    timestamp: u64,
    labels: HashMap<String, String>,
    content_type: String,
}

impl RecordReader {
    /// Create a new RecordReader.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the block of data.
    /// * `block` - The block descriptor that contains the record.
    /// * `record_index` - The index of the record in the block.
    /// * `content_length` - The length of the record.
    /// * `block_manager` - The block manager that manages the block.
    ///
    /// # Errors
    ///
    /// * `HTTPError` - If the file cannot be opened.
    pub fn new(
        path: PathBuf,
        block: &Block,
        record_index: usize,
        chunk_size: u64,
    ) -> Result<RecordReader, ReductError> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let record = &block.records[record_index];
        let offset = record.begin;
        file.seek(SeekFrom::Start(offset))?;

        let mut labels = HashMap::new();
        for label in &record.labels {
            labels.insert(label.name.clone(), label.value.clone());
        }
        Ok(Self {
            file,
            written_bytes: 0,
            content_length: record.end - record.begin,
            chunk_size,
            timestamp: ts_to_us(&record.timestamp.clone().unwrap()),
            labels: labels,
            content_type: record.content_type.clone(),
        })
    }

    /// Read the next chunk of data from the record.
    ///
    /// # Returns
    ///
    /// * `Option<Bytes>` - The next chunk of data. If it is None, the record is finished.
    /// * `HTTPError` - If the data cannot be read.
    pub fn read(&mut self) -> Result<Option<Bytes>, ReductError> {
        if self.is_done() {
            return Ok(None);
        }

        let buffer_size = std::cmp::min(self.chunk_size, self.content_length - self.written_bytes);
        let mut buf = vec![0u8; buffer_size as usize];
        let read = self.file.read(&mut *buf)?;
        self.written_bytes += read as u64;
        Ok(Some(Bytes::from(buf)))
    }

    /// Get the timestamp of the record.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Get the labels of the record.
    pub fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }

    /// Get the content type of the record.
    pub fn content_type(&self) -> &String {
        &self.content_type
    }

    /// Get the content length of the record.
    pub fn content_length(&self) -> u64 {
        self.content_length
    }

    /// Test if the record has been fully read.
    pub fn is_done(&self) -> bool {
        self.written_bytes == self.content_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::proto::{record, Record};
    use prost_wkt_types::Timestamp;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_read() {
        let path = tempdir().unwrap().into_path().join("test");
        let block = Block {
            begin_time: Some(Timestamp {
                seconds: 1,
                nanos: 0,
            }),
            records: vec![Record {
                timestamp: Option::from(Timestamp {
                    seconds: 1,
                    nanos: 0,
                }),
                begin: 0,
                end: 10,
                state: record::State::Started as i32,
                labels: vec![],
                content_type: "".to_string(),
            }],
            ..Block::default()
        };

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .unwrap();
        file.write_all("1234567890".as_bytes()).unwrap();

        {
            let mut reader = RecordReader::new(path.clone(), &block, 0, 5).unwrap();
            let chunk = reader.read().unwrap();
            assert_eq!(chunk.unwrap(), "12345".as_bytes());
            let chunk = reader.read().unwrap();
            assert_eq!(chunk.unwrap(), "67890".as_bytes());

            let chunk = reader.read().unwrap();
            assert!(chunk.is_none());
        }
    }
}
