// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_CsvFsPerfProvider_ClusterCSVFSRedirectedIO struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_CsvFsPerfProvider_ClusterCSVFSRedirectedIO {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AvgBytesPerRead")]
    pub avg_bytes_per_read: Option<u64>,

/// 
    #[serde(rename = "AvgBytesPerWrite")]
    pub avg_bytes_per_write: Option<u64>,

/// 
    #[serde(rename = "AvgReadsQueueLength")]
    pub avg_reads_queue_length: Option<u64>,

/// 
    #[serde(rename = "AvgsecPerRead")]
    pub avgsec_per_read: Option<u32>,

/// 
    #[serde(rename = "AvgsecPerWrite")]
    pub avgsec_per_write: Option<u32>,

/// 
    #[serde(rename = "AvgTrimQueueLength")]
    pub avg_trim_queue_length: Option<u64>,

/// 
    #[serde(rename = "AvgWritesQueueLength")]
    pub avg_writes_queue_length: Option<u64>,

/// 
    #[serde(rename = "CurrentReadQueueLength")]
    pub current_read_queue_length: Option<u64>,

/// 
    #[serde(rename = "CurrentTrimQueueLength")]
    pub current_trim_queue_length: Option<u64>,

/// 
    #[serde(rename = "CurrentWriteQueueLength")]
    pub current_write_queue_length: Option<u64>,

/// 
    #[serde(rename = "ReadBytes")]
    pub read_bytes: Option<u64>,

/// 
    #[serde(rename = "ReadBytesPersec")]
    pub read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "Reads")]
    pub reads: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec")]
    pub reads_persec: Option<u64>,

/// 
    #[serde(rename = "TrimLatency")]
    pub trim_latency: Option<u32>,

/// 
    #[serde(rename = "TrimPersec")]
    pub trim_persec: Option<u64>,

/// 
    #[serde(rename = "WriteBytes")]
    pub write_bytes: Option<u64>,

/// 
    #[serde(rename = "WriteBytesPersec")]
    pub write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WritesPersec")]
    pub writes_persec: Option<u64>,
}

impl Win32_PerfFormattedData_CsvFsPerfProvider_ClusterCSVFSRedirectedIO {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            avg_bytes_per_read: None,
            avg_bytes_per_write: None,
            avg_reads_queue_length: None,
            avgsec_per_read: None,
            avgsec_per_write: None,
            avg_trim_queue_length: None,
            avg_writes_queue_length: None,
            current_read_queue_length: None,
            current_trim_queue_length: None,
            current_write_queue_length: None,
            read_bytes: None,
            read_bytes_persec: None,
            reads: None,
            reads_persec: None,
            trim_latency: None,
            trim_persec: None,
            write_bytes: None,
            write_bytes_persec: None,
            writes_persec: None,
        }
    }


    /// Sets the value of AvgBytesPerRead
    pub fn set_avg_bytes_per_read(&mut self, value: u64) {
        self.avg_bytes_per_read = Some(value);
    }

    /// Gets the value of AvgBytesPerRead
    pub fn get_avg_bytes_per_read(&self) -> Option<&u64> {
        self.avg_bytes_per_read.as_ref()
    }

    /// Sets the value of AvgBytesPerWrite
    pub fn set_avg_bytes_per_write(&mut self, value: u64) {
        self.avg_bytes_per_write = Some(value);
    }

    /// Gets the value of AvgBytesPerWrite
    pub fn get_avg_bytes_per_write(&self) -> Option<&u64> {
        self.avg_bytes_per_write.as_ref()
    }

    /// Sets the value of AvgReadsQueueLength
    pub fn set_avg_reads_queue_length(&mut self, value: u64) {
        self.avg_reads_queue_length = Some(value);
    }

    /// Gets the value of AvgReadsQueueLength
    pub fn get_avg_reads_queue_length(&self) -> Option<&u64> {
        self.avg_reads_queue_length.as_ref()
    }

    /// Sets the value of AvgsecPerRead
    pub fn set_avgsec_per_read(&mut self, value: u32) {
        self.avgsec_per_read = Some(value);
    }

    /// Gets the value of AvgsecPerRead
    pub fn get_avgsec_per_read(&self) -> Option<&u32> {
        self.avgsec_per_read.as_ref()
    }

    /// Sets the value of AvgsecPerWrite
    pub fn set_avgsec_per_write(&mut self, value: u32) {
        self.avgsec_per_write = Some(value);
    }

    /// Gets the value of AvgsecPerWrite
    pub fn get_avgsec_per_write(&self) -> Option<&u32> {
        self.avgsec_per_write.as_ref()
    }

    /// Sets the value of AvgTrimQueueLength
    pub fn set_avg_trim_queue_length(&mut self, value: u64) {
        self.avg_trim_queue_length = Some(value);
    }

    /// Gets the value of AvgTrimQueueLength
    pub fn get_avg_trim_queue_length(&self) -> Option<&u64> {
        self.avg_trim_queue_length.as_ref()
    }

    /// Sets the value of AvgWritesQueueLength
    pub fn set_avg_writes_queue_length(&mut self, value: u64) {
        self.avg_writes_queue_length = Some(value);
    }

    /// Gets the value of AvgWritesQueueLength
    pub fn get_avg_writes_queue_length(&self) -> Option<&u64> {
        self.avg_writes_queue_length.as_ref()
    }

    /// Sets the value of CurrentReadQueueLength
    pub fn set_current_read_queue_length(&mut self, value: u64) {
        self.current_read_queue_length = Some(value);
    }

    /// Gets the value of CurrentReadQueueLength
    pub fn get_current_read_queue_length(&self) -> Option<&u64> {
        self.current_read_queue_length.as_ref()
    }

    /// Sets the value of CurrentTrimQueueLength
    pub fn set_current_trim_queue_length(&mut self, value: u64) {
        self.current_trim_queue_length = Some(value);
    }

    /// Gets the value of CurrentTrimQueueLength
    pub fn get_current_trim_queue_length(&self) -> Option<&u64> {
        self.current_trim_queue_length.as_ref()
    }

    /// Sets the value of CurrentWriteQueueLength
    pub fn set_current_write_queue_length(&mut self, value: u64) {
        self.current_write_queue_length = Some(value);
    }

    /// Gets the value of CurrentWriteQueueLength
    pub fn get_current_write_queue_length(&self) -> Option<&u64> {
        self.current_write_queue_length.as_ref()
    }

    /// Sets the value of ReadBytes
    pub fn set_read_bytes(&mut self, value: u64) {
        self.read_bytes = Some(value);
    }

    /// Gets the value of ReadBytes
    pub fn get_read_bytes(&self) -> Option<&u64> {
        self.read_bytes.as_ref()
    }

    /// Sets the value of ReadBytesPersec
    pub fn set_read_bytes_persec(&mut self, value: u64) {
        self.read_bytes_persec = Some(value);
    }

    /// Gets the value of ReadBytesPersec
    pub fn get_read_bytes_persec(&self) -> Option<&u64> {
        self.read_bytes_persec.as_ref()
    }

    /// Sets the value of Reads
    pub fn set_reads(&mut self, value: u64) {
        self.reads = Some(value);
    }

    /// Gets the value of Reads
    pub fn get_reads(&self) -> Option<&u64> {
        self.reads.as_ref()
    }

    /// Sets the value of ReadsPersec
    pub fn set_reads_persec(&mut self, value: u64) {
        self.reads_persec = Some(value);
    }

    /// Gets the value of ReadsPersec
    pub fn get_reads_persec(&self) -> Option<&u64> {
        self.reads_persec.as_ref()
    }

    /// Sets the value of TrimLatency
    pub fn set_trim_latency(&mut self, value: u32) {
        self.trim_latency = Some(value);
    }

    /// Gets the value of TrimLatency
    pub fn get_trim_latency(&self) -> Option<&u32> {
        self.trim_latency.as_ref()
    }

    /// Sets the value of TrimPersec
    pub fn set_trim_persec(&mut self, value: u64) {
        self.trim_persec = Some(value);
    }

    /// Gets the value of TrimPersec
    pub fn get_trim_persec(&self) -> Option<&u64> {
        self.trim_persec.as_ref()
    }

    /// Sets the value of WriteBytes
    pub fn set_write_bytes(&mut self, value: u64) {
        self.write_bytes = Some(value);
    }

    /// Gets the value of WriteBytes
    pub fn get_write_bytes(&self) -> Option<&u64> {
        self.write_bytes.as_ref()
    }

    /// Sets the value of WriteBytesPersec
    pub fn set_write_bytes_persec(&mut self, value: u64) {
        self.write_bytes_persec = Some(value);
    }

    /// Gets the value of WriteBytesPersec
    pub fn get_write_bytes_persec(&self) -> Option<&u64> {
        self.write_bytes_persec.as_ref()
    }

    /// Sets the value of WritesPersec
    pub fn set_writes_persec(&mut self, value: u64) {
        self.writes_persec = Some(value);
    }

    /// Gets the value of WritesPersec
    pub fn get_writes_persec(&self) -> Option<&u64> {
        self.writes_persec.as_ref()
    }
}

