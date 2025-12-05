// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTRegistryDword struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTRegistryDword {
    #[serde(flatten)]
    pub base: MSFT_MTRegistryValue,

/// 
    #[serde(rename = "Data")]
    pub data: Option<u32>,
}

impl MSFT_MTRegistryDword {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_MTRegistryValue::new(),
            data: None,
        }
    }


    /// Sets the value of Data
    pub fn set_data(&mut self, value: u32) {
        self.data = Some(value);
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> Option<&u32> {
        self.data.as_ref()
    }
}

