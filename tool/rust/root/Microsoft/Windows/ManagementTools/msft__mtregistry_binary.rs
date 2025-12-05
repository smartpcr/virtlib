// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTRegistryBinary struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTRegistryBinary {
    #[serde(flatten)]
    pub base: MSFT_MTRegistryValue,

/// 
    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

impl MSFT_MTRegistryBinary {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_MTRegistryValue::new(),
            data: Vec::new(),
        }
    }


    /// Sets the value of Data
    pub fn set_data(&mut self, value: Vec<u8>) {
        self.data = value;
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}

