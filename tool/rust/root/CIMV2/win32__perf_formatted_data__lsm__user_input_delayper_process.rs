// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_LSM_UserInputDelayperProcess struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_LSM_UserInputDelayperProcess {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "MaxInputDelay")]
    pub max_input_delay: Option<u64>,
}

impl Win32_PerfFormattedData_LSM_UserInputDelayperProcess {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            max_input_delay: None,
        }
    }


    /// Sets the value of MaxInputDelay
    pub fn set_max_input_delay(&mut self, value: u64) {
        self.max_input_delay = Some(value);
    }

    /// Gets the value of MaxInputDelay
    pub fn get_max_input_delay(&self) -> Option<&u64> {
        self.max_input_delay.as_ref()
    }
}

