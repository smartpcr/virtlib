// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PnPDevicePropertyStringArray struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PnPDevicePropertyStringArray {
    #[serde(flatten)]
    pub base: Win32_PnPDeviceProperty,

/// 
    #[serde(rename = "Data")]
    pub data: Vec<String>,
}

impl Win32_PnPDevicePropertyStringArray {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PnPDeviceProperty::new(),
            data: Vec::new(),
        }
    }


    /// Sets the value of Data
    pub fn set_data(&mut self, value: Vec<String>) {
        self.data = value;
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> &Vec<String> {
        &self.data
    }
}

