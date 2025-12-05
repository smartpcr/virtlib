// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_WFP struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_WFP {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ProviderCount")]
    pub provider_count: Option<u32>,
}

impl Win32_PerfRawData_Counters_WFP {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            provider_count: None,
        }
    }


    /// Sets the value of ProviderCount
    pub fn set_provider_count(&mut self, value: u32) {
        self.provider_count = Some(value);
    }

    /// Gets the value of ProviderCount
    pub fn get_provider_count(&self) -> Option<&u32> {
        self.provider_count.as_ref()
    }
}

