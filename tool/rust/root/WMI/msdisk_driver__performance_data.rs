// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSDiskDriver_PerformanceData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSDiskDriver_PerformanceData {
    #[serde(flatten)]
    pub base: MSDiskDriver,

/// 
    #[serde(rename = "BytesRead")]
    pub bytes_read: Option<i64>,

/// 
    #[serde(rename = "BytesWritten")]
    pub bytes_written: Option<i64>,

/// 
    #[serde(rename = "IdleTime")]
    pub idle_time: Option<i64>,

/// 
    #[serde(rename = "QueryTime")]
    pub query_time: Option<i64>,

/// 
    #[serde(rename = "QueueDepth")]
    pub queue_depth: Option<u32>,

/// 
    #[serde(rename = "ReadCount")]
    pub read_count: Option<u32>,

/// 
    #[serde(rename = "ReadTime")]
    pub read_time: Option<i64>,

/// 
    #[serde(rename = "SplitCount")]
    pub split_count: Option<u32>,

/// 
    #[serde(rename = "StorageDeviceNumber")]
    pub storage_device_number: Option<u32>,

/// 
    #[serde(rename = "StorageManagerName")]
    pub storage_manager_name: Vec<u16>,

/// 
    #[serde(rename = "WriteCount")]
    pub write_count: Option<u32>,

/// 
    #[serde(rename = "WriteTime")]
    pub write_time: Option<i64>,
}

impl MSDiskDriver_PerformanceData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSDiskDriver::new(),
            bytes_read: None,
            bytes_written: None,
            idle_time: None,
            query_time: None,
            queue_depth: None,
            read_count: None,
            read_time: None,
            split_count: None,
            storage_device_number: None,
            storage_manager_name: Vec::new(),
            write_count: None,
            write_time: None,
        }
    }


    /// Sets the value of BytesRead
    pub fn set_bytes_read(&mut self, value: i64) {
        self.bytes_read = Some(value);
    }

    /// Gets the value of BytesRead
    pub fn get_bytes_read(&self) -> Option<&i64> {
        self.bytes_read.as_ref()
    }

    /// Sets the value of BytesWritten
    pub fn set_bytes_written(&mut self, value: i64) {
        self.bytes_written = Some(value);
    }

    /// Gets the value of BytesWritten
    pub fn get_bytes_written(&self) -> Option<&i64> {
        self.bytes_written.as_ref()
    }

    /// Sets the value of IdleTime
    pub fn set_idle_time(&mut self, value: i64) {
        self.idle_time = Some(value);
    }

    /// Gets the value of IdleTime
    pub fn get_idle_time(&self) -> Option<&i64> {
        self.idle_time.as_ref()
    }

    /// Sets the value of QueryTime
    pub fn set_query_time(&mut self, value: i64) {
        self.query_time = Some(value);
    }

    /// Gets the value of QueryTime
    pub fn get_query_time(&self) -> Option<&i64> {
        self.query_time.as_ref()
    }

    /// Sets the value of QueueDepth
    pub fn set_queue_depth(&mut self, value: u32) {
        self.queue_depth = Some(value);
    }

    /// Gets the value of QueueDepth
    pub fn get_queue_depth(&self) -> Option<&u32> {
        self.queue_depth.as_ref()
    }

    /// Sets the value of ReadCount
    pub fn set_read_count(&mut self, value: u32) {
        self.read_count = Some(value);
    }

    /// Gets the value of ReadCount
    pub fn get_read_count(&self) -> Option<&u32> {
        self.read_count.as_ref()
    }

    /// Sets the value of ReadTime
    pub fn set_read_time(&mut self, value: i64) {
        self.read_time = Some(value);
    }

    /// Gets the value of ReadTime
    pub fn get_read_time(&self) -> Option<&i64> {
        self.read_time.as_ref()
    }

    /// Sets the value of SplitCount
    pub fn set_split_count(&mut self, value: u32) {
        self.split_count = Some(value);
    }

    /// Gets the value of SplitCount
    pub fn get_split_count(&self) -> Option<&u32> {
        self.split_count.as_ref()
    }

    /// Sets the value of StorageDeviceNumber
    pub fn set_storage_device_number(&mut self, value: u32) {
        self.storage_device_number = Some(value);
    }

    /// Gets the value of StorageDeviceNumber
    pub fn get_storage_device_number(&self) -> Option<&u32> {
        self.storage_device_number.as_ref()
    }

    /// Sets the value of StorageManagerName
    pub fn set_storage_manager_name(&mut self, value: Vec<u16>) {
        self.storage_manager_name = value;
    }

    /// Gets the value of StorageManagerName
    pub fn get_storage_manager_name(&self) -> &Vec<u16> {
        &self.storage_manager_name
    }

    /// Sets the value of WriteCount
    pub fn set_write_count(&mut self, value: u32) {
        self.write_count = Some(value);
    }

    /// Gets the value of WriteCount
    pub fn get_write_count(&self) -> Option<&u32> {
        self.write_count.as_ref()
    }

    /// Sets the value of WriteTime
    pub fn set_write_time(&mut self, value: i64) {
        self.write_time = Some(value);
    }

    /// Gets the value of WriteTime
    pub fn get_write_time(&self) -> Option<&i64> {
        self.write_time.as_ref()
    }
}

