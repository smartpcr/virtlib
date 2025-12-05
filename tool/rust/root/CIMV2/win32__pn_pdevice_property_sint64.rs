// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PnPDevicePropertySint64 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PnPDevicePropertySint64 {
    #[serde(flatten)]
    pub base: Win32_PnPDeviceProperty,

/// 
    #[serde(rename = "Data")]
    pub data: Option<i64>,
}

impl Win32_PnPDevicePropertySint64 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PnPDeviceProperty::new(),
            data: None,
        }
    }


    /// Sets the value of Data
    pub fn set_data(&mut self, value: i64) {
        self.data = Some(value);
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> Option<&i64> {
        self.data.as_ref()
    }
}

