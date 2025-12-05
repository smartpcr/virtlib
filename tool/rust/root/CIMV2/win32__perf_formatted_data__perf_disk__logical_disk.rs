// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_PerfDisk_LogicalDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_PerfDisk_LogicalDisk {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AvgDiskBytesPerRead")]
    pub avg_disk_bytes_per_read: Option<u64>,

/// 
    #[serde(rename = "AvgDiskBytesPerTransfer")]
    pub avg_disk_bytes_per_transfer: Option<u64>,

/// 
    #[serde(rename = "AvgDiskBytesPerWrite")]
    pub avg_disk_bytes_per_write: Option<u64>,

/// 
    #[serde(rename = "AvgDiskQueueLength")]
    pub avg_disk_queue_length: Option<u64>,

/// 
    #[serde(rename = "AvgDiskReadQueueLength")]
    pub avg_disk_read_queue_length: Option<u64>,

/// 
    #[serde(rename = "AvgDisksecPerRead")]
    pub avg_disksec_per_read: Option<u32>,

/// 
    #[serde(rename = "AvgDisksecPerTransfer")]
    pub avg_disksec_per_transfer: Option<u32>,

/// 
    #[serde(rename = "AvgDisksecPerWrite")]
    pub avg_disksec_per_write: Option<u32>,

/// 
    #[serde(rename = "AvgDiskWriteQueueLength")]
    pub avg_disk_write_queue_length: Option<u64>,

/// 
    #[serde(rename = "CurrentDiskQueueLength")]
    pub current_disk_queue_length: Option<u32>,

/// 
    #[serde(rename = "DiskBytesPersec")]
    pub disk_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "DiskReadBytesPersec")]
    pub disk_read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "DiskReadsPersec")]
    pub disk_reads_persec: Option<u32>,

/// 
    #[serde(rename = "DiskTransfersPersec")]
    pub disk_transfers_persec: Option<u32>,

/// 
    #[serde(rename = "DiskWriteBytesPersec")]
    pub disk_write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "DiskWritesPersec")]
    pub disk_writes_persec: Option<u32>,

/// 
    #[serde(rename = "FreeMegabytes")]
    pub free_megabytes: Option<u32>,

/// 
    #[serde(rename = "PercentDiskReadTime")]
    pub percent_disk_read_time: Option<u64>,

/// 
    #[serde(rename = "PercentDiskTime")]
    pub percent_disk_time: Option<u64>,

/// 
    #[serde(rename = "PercentDiskWriteTime")]
    pub percent_disk_write_time: Option<u64>,

/// 
    #[serde(rename = "PercentFreeSpace")]
    pub percent_free_space: Option<u32>,

/// 
    #[serde(rename = "PercentIdleTime")]
    pub percent_idle_time: Option<u64>,

/// 
    #[serde(rename = "SplitIOPerSec")]
    pub split_ioper_sec: Option<u32>,
}

impl Win32_PerfFormattedData_PerfDisk_LogicalDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            avg_disk_bytes_per_read: None,
            avg_disk_bytes_per_transfer: None,
            avg_disk_bytes_per_write: None,
            avg_disk_queue_length: None,
            avg_disk_read_queue_length: None,
            avg_disksec_per_read: None,
            avg_disksec_per_transfer: None,
            avg_disksec_per_write: None,
            avg_disk_write_queue_length: None,
            current_disk_queue_length: None,
            disk_bytes_persec: None,
            disk_read_bytes_persec: None,
            disk_reads_persec: None,
            disk_transfers_persec: None,
            disk_write_bytes_persec: None,
            disk_writes_persec: None,
            free_megabytes: None,
            percent_disk_read_time: None,
            percent_disk_time: None,
            percent_disk_write_time: None,
            percent_free_space: None,
            percent_idle_time: None,
            split_ioper_sec: None,
        }
    }


    /// Sets the value of AvgDiskBytesPerRead
    pub fn set_avg_disk_bytes_per_read(&mut self, value: u64) {
        self.avg_disk_bytes_per_read = Some(value);
    }

    /// Gets the value of AvgDiskBytesPerRead
    pub fn get_avg_disk_bytes_per_read(&self) -> Option<&u64> {
        self.avg_disk_bytes_per_read.as_ref()
    }

    /// Sets the value of AvgDiskBytesPerTransfer
    pub fn set_avg_disk_bytes_per_transfer(&mut self, value: u64) {
        self.avg_disk_bytes_per_transfer = Some(value);
    }

    /// Gets the value of AvgDiskBytesPerTransfer
    pub fn get_avg_disk_bytes_per_transfer(&self) -> Option<&u64> {
        self.avg_disk_bytes_per_transfer.as_ref()
    }

    /// Sets the value of AvgDiskBytesPerWrite
    pub fn set_avg_disk_bytes_per_write(&mut self, value: u64) {
        self.avg_disk_bytes_per_write = Some(value);
    }

    /// Gets the value of AvgDiskBytesPerWrite
    pub fn get_avg_disk_bytes_per_write(&self) -> Option<&u64> {
        self.avg_disk_bytes_per_write.as_ref()
    }

    /// Sets the value of AvgDiskQueueLength
    pub fn set_avg_disk_queue_length(&mut self, value: u64) {
        self.avg_disk_queue_length = Some(value);
    }

    /// Gets the value of AvgDiskQueueLength
    pub fn get_avg_disk_queue_length(&self) -> Option<&u64> {
        self.avg_disk_queue_length.as_ref()
    }

    /// Sets the value of AvgDiskReadQueueLength
    pub fn set_avg_disk_read_queue_length(&mut self, value: u64) {
        self.avg_disk_read_queue_length = Some(value);
    }

    /// Gets the value of AvgDiskReadQueueLength
    pub fn get_avg_disk_read_queue_length(&self) -> Option<&u64> {
        self.avg_disk_read_queue_length.as_ref()
    }

    /// Sets the value of AvgDisksecPerRead
    pub fn set_avg_disksec_per_read(&mut self, value: u32) {
        self.avg_disksec_per_read = Some(value);
    }

    /// Gets the value of AvgDisksecPerRead
    pub fn get_avg_disksec_per_read(&self) -> Option<&u32> {
        self.avg_disksec_per_read.as_ref()
    }

    /// Sets the value of AvgDisksecPerTransfer
    pub fn set_avg_disksec_per_transfer(&mut self, value: u32) {
        self.avg_disksec_per_transfer = Some(value);
    }

    /// Gets the value of AvgDisksecPerTransfer
    pub fn get_avg_disksec_per_transfer(&self) -> Option<&u32> {
        self.avg_disksec_per_transfer.as_ref()
    }

    /// Sets the value of AvgDisksecPerWrite
    pub fn set_avg_disksec_per_write(&mut self, value: u32) {
        self.avg_disksec_per_write = Some(value);
    }

    /// Gets the value of AvgDisksecPerWrite
    pub fn get_avg_disksec_per_write(&self) -> Option<&u32> {
        self.avg_disksec_per_write.as_ref()
    }

    /// Sets the value of AvgDiskWriteQueueLength
    pub fn set_avg_disk_write_queue_length(&mut self, value: u64) {
        self.avg_disk_write_queue_length = Some(value);
    }

    /// Gets the value of AvgDiskWriteQueueLength
    pub fn get_avg_disk_write_queue_length(&self) -> Option<&u64> {
        self.avg_disk_write_queue_length.as_ref()
    }

    /// Sets the value of CurrentDiskQueueLength
    pub fn set_current_disk_queue_length(&mut self, value: u32) {
        self.current_disk_queue_length = Some(value);
    }

    /// Gets the value of CurrentDiskQueueLength
    pub fn get_current_disk_queue_length(&self) -> Option<&u32> {
        self.current_disk_queue_length.as_ref()
    }

    /// Sets the value of DiskBytesPersec
    pub fn set_disk_bytes_persec(&mut self, value: u64) {
        self.disk_bytes_persec = Some(value);
    }

    /// Gets the value of DiskBytesPersec
    pub fn get_disk_bytes_persec(&self) -> Option<&u64> {
        self.disk_bytes_persec.as_ref()
    }

    /// Sets the value of DiskReadBytesPersec
    pub fn set_disk_read_bytes_persec(&mut self, value: u64) {
        self.disk_read_bytes_persec = Some(value);
    }

    /// Gets the value of DiskReadBytesPersec
    pub fn get_disk_read_bytes_persec(&self) -> Option<&u64> {
        self.disk_read_bytes_persec.as_ref()
    }

    /// Sets the value of DiskReadsPersec
    pub fn set_disk_reads_persec(&mut self, value: u32) {
        self.disk_reads_persec = Some(value);
    }

    /// Gets the value of DiskReadsPersec
    pub fn get_disk_reads_persec(&self) -> Option<&u32> {
        self.disk_reads_persec.as_ref()
    }

    /// Sets the value of DiskTransfersPersec
    pub fn set_disk_transfers_persec(&mut self, value: u32) {
        self.disk_transfers_persec = Some(value);
    }

    /// Gets the value of DiskTransfersPersec
    pub fn get_disk_transfers_persec(&self) -> Option<&u32> {
        self.disk_transfers_persec.as_ref()
    }

    /// Sets the value of DiskWriteBytesPersec
    pub fn set_disk_write_bytes_persec(&mut self, value: u64) {
        self.disk_write_bytes_persec = Some(value);
    }

    /// Gets the value of DiskWriteBytesPersec
    pub fn get_disk_write_bytes_persec(&self) -> Option<&u64> {
        self.disk_write_bytes_persec.as_ref()
    }

    /// Sets the value of DiskWritesPersec
    pub fn set_disk_writes_persec(&mut self, value: u32) {
        self.disk_writes_persec = Some(value);
    }

    /// Gets the value of DiskWritesPersec
    pub fn get_disk_writes_persec(&self) -> Option<&u32> {
        self.disk_writes_persec.as_ref()
    }

    /// Sets the value of FreeMegabytes
    pub fn set_free_megabytes(&mut self, value: u32) {
        self.free_megabytes = Some(value);
    }

    /// Gets the value of FreeMegabytes
    pub fn get_free_megabytes(&self) -> Option<&u32> {
        self.free_megabytes.as_ref()
    }

    /// Sets the value of PercentDiskReadTime
    pub fn set_percent_disk_read_time(&mut self, value: u64) {
        self.percent_disk_read_time = Some(value);
    }

    /// Gets the value of PercentDiskReadTime
    pub fn get_percent_disk_read_time(&self) -> Option<&u64> {
        self.percent_disk_read_time.as_ref()
    }

    /// Sets the value of PercentDiskTime
    pub fn set_percent_disk_time(&mut self, value: u64) {
        self.percent_disk_time = Some(value);
    }

    /// Gets the value of PercentDiskTime
    pub fn get_percent_disk_time(&self) -> Option<&u64> {
        self.percent_disk_time.as_ref()
    }

    /// Sets the value of PercentDiskWriteTime
    pub fn set_percent_disk_write_time(&mut self, value: u64) {
        self.percent_disk_write_time = Some(value);
    }

    /// Gets the value of PercentDiskWriteTime
    pub fn get_percent_disk_write_time(&self) -> Option<&u64> {
        self.percent_disk_write_time.as_ref()
    }

    /// Sets the value of PercentFreeSpace
    pub fn set_percent_free_space(&mut self, value: u32) {
        self.percent_free_space = Some(value);
    }

    /// Gets the value of PercentFreeSpace
    pub fn get_percent_free_space(&self) -> Option<&u32> {
        self.percent_free_space.as_ref()
    }

    /// Sets the value of PercentIdleTime
    pub fn set_percent_idle_time(&mut self, value: u64) {
        self.percent_idle_time = Some(value);
    }

    /// Gets the value of PercentIdleTime
    pub fn get_percent_idle_time(&self) -> Option<&u64> {
        self.percent_idle_time.as_ref()
    }

    /// Sets the value of SplitIOPerSec
    pub fn set_split_ioper_sec(&mut self, value: u32) {
        self.split_ioper_sec = Some(value);
    }

    /// Gets the value of SplitIOPerSec
    pub fn get_split_ioper_sec(&self) -> Option<&u32> {
        self.split_ioper_sec.as_ref()
    }
}

