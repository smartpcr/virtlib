// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_LSM_UserInputDelayperSession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_LSM_UserInputDelayperSession {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "MaxInputDelay")]
    pub max_input_delay: Option<u64>,

/// 
    #[serde(rename = "MaxInputDelay_Base")]
    pub max_input_delay__base: Option<u32>,
}

impl Win32_PerfRawData_LSM_UserInputDelayperSession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            max_input_delay: None,
            max_input_delay__base: None,
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

    /// Sets the value of MaxInputDelay_Base
    pub fn set_max_input_delay__base(&mut self, value: u32) {
        self.max_input_delay__base = Some(value);
    }

    /// Gets the value of MaxInputDelay_Base
    pub fn get_max_input_delay__base(&self) -> Option<&u32> {
        self.max_input_delay__base.as_ref()
    }
}

