// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_PerfOS_NUMANodeMemory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_PerfOS_NUMANodeMemory {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AvailableMBytes")]
    pub available_mbytes: Option<u32>,

/// 
    #[serde(rename = "FreeAndZeroPageListMBytes")]
    pub free_and_zero_page_list_mbytes: Option<u32>,

/// 
    #[serde(rename = "StandbyListMBytes")]
    pub standby_list_mbytes: Option<u32>,

/// 
    #[serde(rename = "TotalMBytes")]
    pub total_mbytes: Option<u32>,
}

impl Win32_PerfFormattedData_PerfOS_NUMANodeMemory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            available_mbytes: None,
            free_and_zero_page_list_mbytes: None,
            standby_list_mbytes: None,
            total_mbytes: None,
        }
    }


    /// Sets the value of AvailableMBytes
    pub fn set_available_mbytes(&mut self, value: u32) {
        self.available_mbytes = Some(value);
    }

    /// Gets the value of AvailableMBytes
    pub fn get_available_mbytes(&self) -> Option<&u32> {
        self.available_mbytes.as_ref()
    }

    /// Sets the value of FreeAndZeroPageListMBytes
    pub fn set_free_and_zero_page_list_mbytes(&mut self, value: u32) {
        self.free_and_zero_page_list_mbytes = Some(value);
    }

    /// Gets the value of FreeAndZeroPageListMBytes
    pub fn get_free_and_zero_page_list_mbytes(&self) -> Option<&u32> {
        self.free_and_zero_page_list_mbytes.as_ref()
    }

    /// Sets the value of StandbyListMBytes
    pub fn set_standby_list_mbytes(&mut self, value: u32) {
        self.standby_list_mbytes = Some(value);
    }

    /// Gets the value of StandbyListMBytes
    pub fn get_standby_list_mbytes(&self) -> Option<&u32> {
        self.standby_list_mbytes.as_ref()
    }

    /// Sets the value of TotalMBytes
    pub fn set_total_mbytes(&mut self, value: u32) {
        self.total_mbytes = Some(value);
    }

    /// Gets the value of TotalMBytes
    pub fn get_total_mbytes(&self) -> Option<&u32> {
        self.total_mbytes.as_ref()
    }
}

