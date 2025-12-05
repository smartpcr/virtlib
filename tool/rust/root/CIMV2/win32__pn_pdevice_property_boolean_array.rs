// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PnPDevicePropertyBooleanArray struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PnPDevicePropertyBooleanArray {
    #[serde(flatten)]
    pub base: Win32_PnPDeviceProperty,

/// 
    #[serde(rename = "Data")]
    pub data: Vec<bool>,
}

impl Win32_PnPDevicePropertyBooleanArray {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PnPDeviceProperty::new(),
            data: Vec::new(),
        }
    }


    /// Sets the value of Data
    pub fn set_data(&mut self, value: Vec<bool>) {
        self.data = value;
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> &Vec<bool> {
        &self.data
    }
}

