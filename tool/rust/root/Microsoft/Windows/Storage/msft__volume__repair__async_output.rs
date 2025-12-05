// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Volume_Repair_AsyncOutput struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Volume_Repair_AsyncOutput {
    #[serde(flatten)]
    pub base: MSFT_StorageJobOutParams,

/// 
    #[serde(rename = "Output")]
    pub output: Option<u32>,
}

impl MSFT_Volume_Repair_AsyncOutput {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageJobOutParams::new(),
            output: None,
        }
    }


    /// Sets the value of Output
    pub fn set_output(&mut self, value: u32) {
        self.output = Some(value);
    }

    /// Gets the value of Output
    pub fn get_output(&self) -> Option<&u32> {
        self.output.as_ref()
    }
}

