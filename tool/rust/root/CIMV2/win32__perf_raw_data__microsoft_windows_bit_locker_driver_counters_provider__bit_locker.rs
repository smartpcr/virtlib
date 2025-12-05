// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_MicrosoftWindowsBitLockerDriverCountersProvider_BitLocker struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_MicrosoftWindowsBitLockerDriverCountersProvider_BitLocker {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "MaxReadSplitSize")]
    pub max_read_split_size: Option<u32>,

/// 
    #[serde(rename = "MaxWriteSplitSize")]
    pub max_write_split_size: Option<u32>,

/// 
    #[serde(rename = "MinReadSplitSize")]
    pub min_read_split_size: Option<u32>,

/// 
    #[serde(rename = "MinWriteSplitSize")]
    pub min_write_split_size: Option<u32>,

/// 
    #[serde(rename = "ReadRequestsPersec")]
    pub read_requests_persec: Option<u64>,

/// 
    #[serde(rename = "ReadSubrequestsPersec")]
    pub read_subrequests_persec: Option<u64>,

/// 
    #[serde(rename = "WriteRequestsPersec")]
    pub write_requests_persec: Option<u64>,

/// 
    #[serde(rename = "WriteSubrequestsPersec")]
    pub write_subrequests_persec: Option<u64>,
}

impl Win32_PerfRawData_MicrosoftWindowsBitLockerDriverCountersProvider_BitLocker {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            max_read_split_size: None,
            max_write_split_size: None,
            min_read_split_size: None,
            min_write_split_size: None,
            read_requests_persec: None,
            read_subrequests_persec: None,
            write_requests_persec: None,
            write_subrequests_persec: None,
        }
    }


    /// Sets the value of MaxReadSplitSize
    pub fn set_max_read_split_size(&mut self, value: u32) {
        self.max_read_split_size = Some(value);
    }

    /// Gets the value of MaxReadSplitSize
    pub fn get_max_read_split_size(&self) -> Option<&u32> {
        self.max_read_split_size.as_ref()
    }

    /// Sets the value of MaxWriteSplitSize
    pub fn set_max_write_split_size(&mut self, value: u32) {
        self.max_write_split_size = Some(value);
    }

    /// Gets the value of MaxWriteSplitSize
    pub fn get_max_write_split_size(&self) -> Option<&u32> {
        self.max_write_split_size.as_ref()
    }

    /// Sets the value of MinReadSplitSize
    pub fn set_min_read_split_size(&mut self, value: u32) {
        self.min_read_split_size = Some(value);
    }

    /// Gets the value of MinReadSplitSize
    pub fn get_min_read_split_size(&self) -> Option<&u32> {
        self.min_read_split_size.as_ref()
    }

    /// Sets the value of MinWriteSplitSize
    pub fn set_min_write_split_size(&mut self, value: u32) {
        self.min_write_split_size = Some(value);
    }

    /// Gets the value of MinWriteSplitSize
    pub fn get_min_write_split_size(&self) -> Option<&u32> {
        self.min_write_split_size.as_ref()
    }

    /// Sets the value of ReadRequestsPersec
    pub fn set_read_requests_persec(&mut self, value: u64) {
        self.read_requests_persec = Some(value);
    }

    /// Gets the value of ReadRequestsPersec
    pub fn get_read_requests_persec(&self) -> Option<&u64> {
        self.read_requests_persec.as_ref()
    }

    /// Sets the value of ReadSubrequestsPersec
    pub fn set_read_subrequests_persec(&mut self, value: u64) {
        self.read_subrequests_persec = Some(value);
    }

    /// Gets the value of ReadSubrequestsPersec
    pub fn get_read_subrequests_persec(&self) -> Option<&u64> {
        self.read_subrequests_persec.as_ref()
    }

    /// Sets the value of WriteRequestsPersec
    pub fn set_write_requests_persec(&mut self, value: u64) {
        self.write_requests_persec = Some(value);
    }

    /// Gets the value of WriteRequestsPersec
    pub fn get_write_requests_persec(&self) -> Option<&u64> {
        self.write_requests_persec.as_ref()
    }

    /// Sets the value of WriteSubrequestsPersec
    pub fn set_write_subrequests_persec(&mut self, value: u64) {
        self.write_subrequests_persec = Some(value);
    }

    /// Gets the value of WriteSubrequestsPersec
    pub fn get_write_subrequests_persec(&self) -> Option<&u64> {
        self.write_subrequests_persec.as_ref()
    }
}

