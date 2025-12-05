// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerManagerRequestGuid struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerManagerRequestGuid {

/// 
    #[serde(rename = "HighHalf")]
    pub high_half: Option<u64>,

/// 
    #[serde(rename = "LowHalf")]
    pub low_half: Option<u64>,
}

impl MSFT_ServerManagerRequestGuid {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            high_half: None,
            low_half: None,
        }
    }


    /// Sets the value of HighHalf
    pub fn set_high_half(&mut self, value: u64) {
        self.high_half = Some(value);
    }

    /// Gets the value of HighHalf
    pub fn get_high_half(&self) -> Option<&u64> {
        self.high_half.as_ref()
    }

    /// Sets the value of LowHalf
    pub fn set_low_half(&mut self, value: u64) {
        self.low_half = Some(value);
    }

    /// Gets the value of LowHalf
    pub fn get_low_half(&self) -> Option<&u64> {
        self.low_half.as_ref()
    }
}

