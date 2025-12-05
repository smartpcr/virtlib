// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_VidPerfProvider_HyperVVMVidNumaNode struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_VidPerfProvider_HyperVVMVidNumaNode {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "PageCount")]
    pub page_count: Option<u64>,

/// 
    #[serde(rename = "ProcessorCount")]
    pub processor_count: Option<u64>,
}

impl Win32_PerfFormattedData_VidPerfProvider_HyperVVMVidNumaNode {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            page_count: None,
            processor_count: None,
        }
    }


    /// Sets the value of PageCount
    pub fn set_page_count(&mut self, value: u64) {
        self.page_count = Some(value);
    }

    /// Gets the value of PageCount
    pub fn get_page_count(&self) -> Option<&u64> {
        self.page_count.as_ref()
    }

    /// Sets the value of ProcessorCount
    pub fn set_processor_count(&mut self, value: u64) {
        self.processor_count = Some(value);
    }

    /// Gets the value of ProcessorCount
    pub fn get_processor_count(&self) -> Option<&u64> {
        self.processor_count.as_ref()
    }
}

