// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_XHCICommonBuffer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_XHCICommonBuffer {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AllocationCount")]
    pub allocation_count: Option<u32>,

/// 
    #[serde(rename = "FreeCount")]
    pub free_count: Option<u32>,

/// 
    #[serde(rename = "PagesInUse")]
    pub pages_in_use: Option<u32>,

/// 
    #[serde(rename = "PagesTotal")]
    pub pages_total: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_XHCICommonBuffer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            allocation_count: None,
            free_count: None,
            pages_in_use: None,
            pages_total: None,
        }
    }


    /// Sets the value of AllocationCount
    pub fn set_allocation_count(&mut self, value: u32) {
        self.allocation_count = Some(value);
    }

    /// Gets the value of AllocationCount
    pub fn get_allocation_count(&self) -> Option<&u32> {
        self.allocation_count.as_ref()
    }

    /// Sets the value of FreeCount
    pub fn set_free_count(&mut self, value: u32) {
        self.free_count = Some(value);
    }

    /// Gets the value of FreeCount
    pub fn get_free_count(&self) -> Option<&u32> {
        self.free_count.as_ref()
    }

    /// Sets the value of PagesInUse
    pub fn set_pages_in_use(&mut self, value: u32) {
        self.pages_in_use = Some(value);
    }

    /// Gets the value of PagesInUse
    pub fn get_pages_in_use(&self) -> Option<&u32> {
        self.pages_in_use.as_ref()
    }

    /// Sets the value of PagesTotal
    pub fn set_pages_total(&mut self, value: u32) {
        self.pages_total = Some(value);
    }

    /// Gets the value of PagesTotal
    pub fn get_pages_total(&self) -> Option<&u32> {
        self.pages_total.as_ref()
    }
}

