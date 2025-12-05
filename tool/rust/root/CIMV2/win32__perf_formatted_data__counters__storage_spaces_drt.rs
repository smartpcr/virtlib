// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_StorageSpacesDrt struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_StorageSpacesDrt {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "CleanBytes")]
    pub clean_bytes: Option<u64>,

/// 
    #[serde(rename = "CleanCount")]
    pub clean_count: Option<u64>,

/// 
    #[serde(rename = "DirtyBytes")]
    pub dirty_bytes: Option<u64>,

/// 
    #[serde(rename = "DirtyCount")]
    pub dirty_count: Option<u64>,

/// 
    #[serde(rename = "FlushedBytes")]
    pub flushed_bytes: Option<u64>,

/// 
    #[serde(rename = "FlushedCount")]
    pub flushed_count: Option<u64>,

/// 
    #[serde(rename = "FlushingBytes")]
    pub flushing_bytes: Option<u64>,

/// 
    #[serde(rename = "FlushingCount")]
    pub flushing_count: Option<u64>,

/// 
    #[serde(rename = "Limit")]
    pub limit: Option<u32>,

/// 
    #[serde(rename = "SynchronizingBytes")]
    pub synchronizing_bytes: Option<u64>,

/// 
    #[serde(rename = "SynchronizingCount")]
    pub synchronizing_count: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_StorageSpacesDrt {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            clean_bytes: None,
            clean_count: None,
            dirty_bytes: None,
            dirty_count: None,
            flushed_bytes: None,
            flushed_count: None,
            flushing_bytes: None,
            flushing_count: None,
            limit: None,
            synchronizing_bytes: None,
            synchronizing_count: None,
        }
    }


    /// Sets the value of CleanBytes
    pub fn set_clean_bytes(&mut self, value: u64) {
        self.clean_bytes = Some(value);
    }

    /// Gets the value of CleanBytes
    pub fn get_clean_bytes(&self) -> Option<&u64> {
        self.clean_bytes.as_ref()
    }

    /// Sets the value of CleanCount
    pub fn set_clean_count(&mut self, value: u64) {
        self.clean_count = Some(value);
    }

    /// Gets the value of CleanCount
    pub fn get_clean_count(&self) -> Option<&u64> {
        self.clean_count.as_ref()
    }

    /// Sets the value of DirtyBytes
    pub fn set_dirty_bytes(&mut self, value: u64) {
        self.dirty_bytes = Some(value);
    }

    /// Gets the value of DirtyBytes
    pub fn get_dirty_bytes(&self) -> Option<&u64> {
        self.dirty_bytes.as_ref()
    }

    /// Sets the value of DirtyCount
    pub fn set_dirty_count(&mut self, value: u64) {
        self.dirty_count = Some(value);
    }

    /// Gets the value of DirtyCount
    pub fn get_dirty_count(&self) -> Option<&u64> {
        self.dirty_count.as_ref()
    }

    /// Sets the value of FlushedBytes
    pub fn set_flushed_bytes(&mut self, value: u64) {
        self.flushed_bytes = Some(value);
    }

    /// Gets the value of FlushedBytes
    pub fn get_flushed_bytes(&self) -> Option<&u64> {
        self.flushed_bytes.as_ref()
    }

    /// Sets the value of FlushedCount
    pub fn set_flushed_count(&mut self, value: u64) {
        self.flushed_count = Some(value);
    }

    /// Gets the value of FlushedCount
    pub fn get_flushed_count(&self) -> Option<&u64> {
        self.flushed_count.as_ref()
    }

    /// Sets the value of FlushingBytes
    pub fn set_flushing_bytes(&mut self, value: u64) {
        self.flushing_bytes = Some(value);
    }

    /// Gets the value of FlushingBytes
    pub fn get_flushing_bytes(&self) -> Option<&u64> {
        self.flushing_bytes.as_ref()
    }

    /// Sets the value of FlushingCount
    pub fn set_flushing_count(&mut self, value: u64) {
        self.flushing_count = Some(value);
    }

    /// Gets the value of FlushingCount
    pub fn get_flushing_count(&self) -> Option<&u64> {
        self.flushing_count.as_ref()
    }

    /// Sets the value of Limit
    pub fn set_limit(&mut self, value: u32) {
        self.limit = Some(value);
    }

    /// Gets the value of Limit
    pub fn get_limit(&self) -> Option<&u32> {
        self.limit.as_ref()
    }

    /// Sets the value of SynchronizingBytes
    pub fn set_synchronizing_bytes(&mut self, value: u64) {
        self.synchronizing_bytes = Some(value);
    }

    /// Gets the value of SynchronizingBytes
    pub fn get_synchronizing_bytes(&self) -> Option<&u64> {
        self.synchronizing_bytes.as_ref()
    }

    /// Sets the value of SynchronizingCount
    pub fn set_synchronizing_count(&mut self, value: u64) {
        self.synchronizing_count = Some(value);
    }

    /// Gets the value of SynchronizingCount
    pub fn get_synchronizing_count(&self) -> Option<&u64> {
        self.synchronizing_count.as_ref()
    }
}

