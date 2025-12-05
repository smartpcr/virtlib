// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_MicrosoftWindowsMPTFCounters_MPTFInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_MicrosoftWindowsMPTFCounters_MPTFInformation {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Temperature")]
    pub temperature: Option<u32>,
}

impl Win32_PerfRawData_MicrosoftWindowsMPTFCounters_MPTFInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            temperature: None,
        }
    }


    /// Sets the value of Temperature
    pub fn set_temperature(&mut self, value: u32) {
        self.temperature = Some(value);
    }

    /// Gets the value of Temperature
    pub fn get_temperature(&self) -> Option<&u32> {
        self.temperature.as_ref()
    }
}

