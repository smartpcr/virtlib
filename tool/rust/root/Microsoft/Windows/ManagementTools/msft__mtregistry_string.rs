// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTRegistryString struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTRegistryString {
    #[serde(flatten)]
    pub base: MSFT_MTRegistryValue,

/// 
    #[serde(rename = "Data")]
    pub data: Option<String>,
}

impl MSFT_MTRegistryString {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_MTRegistryValue::new(),
            data: None,
        }
    }


    /// Sets the value of Data
    pub fn set_data(&mut self, value: String) {
        self.data = Some(value);
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> Option<&String> {
        self.data.as_ref()
    }
}

