// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClusResPerfProvider_ClusterKeyValueStore struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClusResPerfProvider_ClusterKeyValueStore {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "BytesRead")]
    pub bytes_read: Option<u64>,

/// 
    #[serde(rename = "BytesReadPersec")]
    pub bytes_read_persec: Option<u64>,

/// 
    #[serde(rename = "BytesWritten")]
    pub bytes_written: Option<u64>,

/// 
    #[serde(rename = "BytesWrittenPersec")]
    pub bytes_written_persec: Option<u64>,

/// 
    #[serde(rename = "IOErrors")]
    pub ioerrors: Option<u64>,

/// 
    #[serde(rename = "IOErrorsPerSec")]
    pub ioerrors_per_sec: Option<u64>,

/// 
    #[serde(rename = "ReadsSent")]
    pub reads_sent: Option<u64>,

/// 
    #[serde(rename = "ReadsSentPersec")]
    pub reads_sent_persec: Option<u64>,

/// 
    #[serde(rename = "WritesSent")]
    pub writes_sent: Option<u64>,

/// 
    #[serde(rename = "WritesSentPersec")]
    pub writes_sent_persec: Option<u64>,
}

impl Win32_PerfRawData_ClusResPerfProvider_ClusterKeyValueStore {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            bytes_read: None,
            bytes_read_persec: None,
            bytes_written: None,
            bytes_written_persec: None,
            ioerrors: None,
            ioerrors_per_sec: None,
            reads_sent: None,
            reads_sent_persec: None,
            writes_sent: None,
            writes_sent_persec: None,
        }
    }


    /// Sets the value of BytesRead
    pub fn set_bytes_read(&mut self, value: u64) {
        self.bytes_read = Some(value);
    }

    /// Gets the value of BytesRead
    pub fn get_bytes_read(&self) -> Option<&u64> {
        self.bytes_read.as_ref()
    }

    /// Sets the value of BytesReadPersec
    pub fn set_bytes_read_persec(&mut self, value: u64) {
        self.bytes_read_persec = Some(value);
    }

    /// Gets the value of BytesReadPersec
    pub fn get_bytes_read_persec(&self) -> Option<&u64> {
        self.bytes_read_persec.as_ref()
    }

    /// Sets the value of BytesWritten
    pub fn set_bytes_written(&mut self, value: u64) {
        self.bytes_written = Some(value);
    }

    /// Gets the value of BytesWritten
    pub fn get_bytes_written(&self) -> Option<&u64> {
        self.bytes_written.as_ref()
    }

    /// Sets the value of BytesWrittenPersec
    pub fn set_bytes_written_persec(&mut self, value: u64) {
        self.bytes_written_persec = Some(value);
    }

    /// Gets the value of BytesWrittenPersec
    pub fn get_bytes_written_persec(&self) -> Option<&u64> {
        self.bytes_written_persec.as_ref()
    }

    /// Sets the value of IOErrors
    pub fn set_ioerrors(&mut self, value: u64) {
        self.ioerrors = Some(value);
    }

    /// Gets the value of IOErrors
    pub fn get_ioerrors(&self) -> Option<&u64> {
        self.ioerrors.as_ref()
    }

    /// Sets the value of IOErrorsPerSec
    pub fn set_ioerrors_per_sec(&mut self, value: u64) {
        self.ioerrors_per_sec = Some(value);
    }

    /// Gets the value of IOErrorsPerSec
    pub fn get_ioerrors_per_sec(&self) -> Option<&u64> {
        self.ioerrors_per_sec.as_ref()
    }

    /// Sets the value of ReadsSent
    pub fn set_reads_sent(&mut self, value: u64) {
        self.reads_sent = Some(value);
    }

    /// Gets the value of ReadsSent
    pub fn get_reads_sent(&self) -> Option<&u64> {
        self.reads_sent.as_ref()
    }

    /// Sets the value of ReadsSentPersec
    pub fn set_reads_sent_persec(&mut self, value: u64) {
        self.reads_sent_persec = Some(value);
    }

    /// Gets the value of ReadsSentPersec
    pub fn get_reads_sent_persec(&self) -> Option<&u64> {
        self.reads_sent_persec.as_ref()
    }

    /// Sets the value of WritesSent
    pub fn set_writes_sent(&mut self, value: u64) {
        self.writes_sent = Some(value);
    }

    /// Gets the value of WritesSent
    pub fn get_writes_sent(&self) -> Option<&u64> {
        self.writes_sent.as_ref()
    }

    /// Sets the value of WritesSentPersec
    pub fn set_writes_sent_persec(&mut self, value: u64) {
        self.writes_sent_persec = Some(value);
    }

    /// Gets the value of WritesSentPersec
    pub fn get_writes_sent_persec(&self) -> Option<&u64> {
        self.writes_sent_persec.as_ref()
    }
}

