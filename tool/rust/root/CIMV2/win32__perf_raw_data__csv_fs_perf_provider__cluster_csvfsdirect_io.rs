// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_CsvFsPerfProvider_ClusterCSVFSDirectIO struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_CsvFsPerfProvider_ClusterCSVFSDirectIO {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AvgBytesPerRead")]
    pub avg_bytes_per_read: Option<u64>,

/// 
    #[serde(rename = "AvgBytesPerRead_Base")]
    pub avg_bytes_per_read__base: Option<u32>,

/// 
    #[serde(rename = "AvgBytesPerWrite")]
    pub avg_bytes_per_write: Option<u64>,

/// 
    #[serde(rename = "AvgBytesPerWrite_Base")]
    pub avg_bytes_per_write__base: Option<u32>,

/// 
    #[serde(rename = "AvgReadQueueLength")]
    pub avg_read_queue_length: Option<u64>,

/// 
    #[serde(rename = "AvgsecPerRead")]
    pub avgsec_per_read: Option<u32>,

/// 
    #[serde(rename = "AvgsecPerRead_Base")]
    pub avgsec_per_read__base: Option<u32>,

/// 
    #[serde(rename = "AvgsecPerWrite")]
    pub avgsec_per_write: Option<u32>,

/// 
    #[serde(rename = "AvgsecPerWrite_Base")]
    pub avgsec_per_write__base: Option<u32>,

/// 
    #[serde(rename = "AvgWriteQueueLength")]
    pub avg_write_queue_length: Option<u64>,

/// 
    #[serde(rename = "CurrentReadQueueLength")]
    pub current_read_queue_length: Option<u64>,

/// 
    #[serde(rename = "CurrentWriteQueueLength")]
    pub current_write_queue_length: Option<u64>,

/// 
    #[serde(rename = "NonSplitReads")]
    pub non_split_reads: Option<u64>,

/// 
    #[serde(rename = "NonSplitReadsPersec")]
    pub non_split_reads_persec: Option<u64>,

/// 
    #[serde(rename = "NonSplitWrites")]
    pub non_split_writes: Option<u64>,

/// 
    #[serde(rename = "NonSplitWritesPersec")]
    pub non_split_writes_persec: Option<u64>,

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
    #[serde(rename = "SplitReads")]
    pub split_reads: Option<u64>,

/// 
    #[serde(rename = "SplitReadsPersec")]
    pub split_reads_persec: Option<u64>,

/// 
    #[serde(rename = "SplitWrites")]
    pub split_writes: Option<u64>,

/// 
    #[serde(rename = "SplitWritesPersec")]
    pub split_writes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteBytes")]
    pub write_bytes: Option<u64>,

/// 
    #[serde(rename = "WriteBytesPersec")]
    pub write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "Writes")]
    pub writes: Option<u64>,

/// 
    #[serde(rename = "WritesPersec")]
    pub writes_persec: Option<u64>,
}

impl Win32_PerfRawData_CsvFsPerfProvider_ClusterCSVFSDirectIO {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            avg_bytes_per_read: None,
            avg_bytes_per_read__base: None,
            avg_bytes_per_write: None,
            avg_bytes_per_write__base: None,
            avg_read_queue_length: None,
            avgsec_per_read: None,
            avgsec_per_read__base: None,
            avgsec_per_write: None,
            avgsec_per_write__base: None,
            avg_write_queue_length: None,
            current_read_queue_length: None,
            current_write_queue_length: None,
            non_split_reads: None,
            non_split_reads_persec: None,
            non_split_writes: None,
            non_split_writes_persec: None,
            read_bytes: None,
            read_bytes_persec: None,
            reads: None,
            reads_persec: None,
            split_reads: None,
            split_reads_persec: None,
            split_writes: None,
            split_writes_persec: None,
            write_bytes: None,
            write_bytes_persec: None,
            writes: None,
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

    /// Sets the value of AvgBytesPerRead_Base
    pub fn set_avg_bytes_per_read__base(&mut self, value: u32) {
        self.avg_bytes_per_read__base = Some(value);
    }

    /// Gets the value of AvgBytesPerRead_Base
    pub fn get_avg_bytes_per_read__base(&self) -> Option<&u32> {
        self.avg_bytes_per_read__base.as_ref()
    }

    /// Sets the value of AvgBytesPerWrite
    pub fn set_avg_bytes_per_write(&mut self, value: u64) {
        self.avg_bytes_per_write = Some(value);
    }

    /// Gets the value of AvgBytesPerWrite
    pub fn get_avg_bytes_per_write(&self) -> Option<&u64> {
        self.avg_bytes_per_write.as_ref()
    }

    /// Sets the value of AvgBytesPerWrite_Base
    pub fn set_avg_bytes_per_write__base(&mut self, value: u32) {
        self.avg_bytes_per_write__base = Some(value);
    }

    /// Gets the value of AvgBytesPerWrite_Base
    pub fn get_avg_bytes_per_write__base(&self) -> Option<&u32> {
        self.avg_bytes_per_write__base.as_ref()
    }

    /// Sets the value of AvgReadQueueLength
    pub fn set_avg_read_queue_length(&mut self, value: u64) {
        self.avg_read_queue_length = Some(value);
    }

    /// Gets the value of AvgReadQueueLength
    pub fn get_avg_read_queue_length(&self) -> Option<&u64> {
        self.avg_read_queue_length.as_ref()
    }

    /// Sets the value of AvgsecPerRead
    pub fn set_avgsec_per_read(&mut self, value: u32) {
        self.avgsec_per_read = Some(value);
    }

    /// Gets the value of AvgsecPerRead
    pub fn get_avgsec_per_read(&self) -> Option<&u32> {
        self.avgsec_per_read.as_ref()
    }

    /// Sets the value of AvgsecPerRead_Base
    pub fn set_avgsec_per_read__base(&mut self, value: u32) {
        self.avgsec_per_read__base = Some(value);
    }

    /// Gets the value of AvgsecPerRead_Base
    pub fn get_avgsec_per_read__base(&self) -> Option<&u32> {
        self.avgsec_per_read__base.as_ref()
    }

    /// Sets the value of AvgsecPerWrite
    pub fn set_avgsec_per_write(&mut self, value: u32) {
        self.avgsec_per_write = Some(value);
    }

    /// Gets the value of AvgsecPerWrite
    pub fn get_avgsec_per_write(&self) -> Option<&u32> {
        self.avgsec_per_write.as_ref()
    }

    /// Sets the value of AvgsecPerWrite_Base
    pub fn set_avgsec_per_write__base(&mut self, value: u32) {
        self.avgsec_per_write__base = Some(value);
    }

    /// Gets the value of AvgsecPerWrite_Base
    pub fn get_avgsec_per_write__base(&self) -> Option<&u32> {
        self.avgsec_per_write__base.as_ref()
    }

    /// Sets the value of AvgWriteQueueLength
    pub fn set_avg_write_queue_length(&mut self, value: u64) {
        self.avg_write_queue_length = Some(value);
    }

    /// Gets the value of AvgWriteQueueLength
    pub fn get_avg_write_queue_length(&self) -> Option<&u64> {
        self.avg_write_queue_length.as_ref()
    }

    /// Sets the value of CurrentReadQueueLength
    pub fn set_current_read_queue_length(&mut self, value: u64) {
        self.current_read_queue_length = Some(value);
    }

    /// Gets the value of CurrentReadQueueLength
    pub fn get_current_read_queue_length(&self) -> Option<&u64> {
        self.current_read_queue_length.as_ref()
    }

    /// Sets the value of CurrentWriteQueueLength
    pub fn set_current_write_queue_length(&mut self, value: u64) {
        self.current_write_queue_length = Some(value);
    }

    /// Gets the value of CurrentWriteQueueLength
    pub fn get_current_write_queue_length(&self) -> Option<&u64> {
        self.current_write_queue_length.as_ref()
    }

    /// Sets the value of NonSplitReads
    pub fn set_non_split_reads(&mut self, value: u64) {
        self.non_split_reads = Some(value);
    }

    /// Gets the value of NonSplitReads
    pub fn get_non_split_reads(&self) -> Option<&u64> {
        self.non_split_reads.as_ref()
    }

    /// Sets the value of NonSplitReadsPersec
    pub fn set_non_split_reads_persec(&mut self, value: u64) {
        self.non_split_reads_persec = Some(value);
    }

    /// Gets the value of NonSplitReadsPersec
    pub fn get_non_split_reads_persec(&self) -> Option<&u64> {
        self.non_split_reads_persec.as_ref()
    }

    /// Sets the value of NonSplitWrites
    pub fn set_non_split_writes(&mut self, value: u64) {
        self.non_split_writes = Some(value);
    }

    /// Gets the value of NonSplitWrites
    pub fn get_non_split_writes(&self) -> Option<&u64> {
        self.non_split_writes.as_ref()
    }

    /// Sets the value of NonSplitWritesPersec
    pub fn set_non_split_writes_persec(&mut self, value: u64) {
        self.non_split_writes_persec = Some(value);
    }

    /// Gets the value of NonSplitWritesPersec
    pub fn get_non_split_writes_persec(&self) -> Option<&u64> {
        self.non_split_writes_persec.as_ref()
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

    /// Sets the value of SplitReads
    pub fn set_split_reads(&mut self, value: u64) {
        self.split_reads = Some(value);
    }

    /// Gets the value of SplitReads
    pub fn get_split_reads(&self) -> Option<&u64> {
        self.split_reads.as_ref()
    }

    /// Sets the value of SplitReadsPersec
    pub fn set_split_reads_persec(&mut self, value: u64) {
        self.split_reads_persec = Some(value);
    }

    /// Gets the value of SplitReadsPersec
    pub fn get_split_reads_persec(&self) -> Option<&u64> {
        self.split_reads_persec.as_ref()
    }

    /// Sets the value of SplitWrites
    pub fn set_split_writes(&mut self, value: u64) {
        self.split_writes = Some(value);
    }

    /// Gets the value of SplitWrites
    pub fn get_split_writes(&self) -> Option<&u64> {
        self.split_writes.as_ref()
    }

    /// Sets the value of SplitWritesPersec
    pub fn set_split_writes_persec(&mut self, value: u64) {
        self.split_writes_persec = Some(value);
    }

    /// Gets the value of SplitWritesPersec
    pub fn get_split_writes_persec(&self) -> Option<&u64> {
        self.split_writes_persec.as_ref()
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

    /// Sets the value of Writes
    pub fn set_writes(&mut self, value: u64) {
        self.writes = Some(value);
    }

    /// Gets the value of Writes
    pub fn get_writes(&self) -> Option<&u64> {
        self.writes.as_ref()
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

