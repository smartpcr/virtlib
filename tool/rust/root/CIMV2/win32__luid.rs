// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_LUID struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_LUID {

/// 
    #[serde(rename = "HighPart")]
    pub high_part: Option<u32>,

/// 
    #[serde(rename = "LowPart")]
    pub low_part: Option<u32>,
}

impl Win32_LUID {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            high_part: None,
            low_part: None,
        }
    }


    /// Sets the value of HighPart
    pub fn set_high_part(&mut self, value: u32) {
        self.high_part = Some(value);
    }

    /// Gets the value of HighPart
    pub fn get_high_part(&self) -> Option<&u32> {
        self.high_part.as_ref()
    }

    /// Sets the value of LowPart
    pub fn set_low_part(&mut self, value: u32) {
        self.low_part = Some(value);
    }

    /// Gets the value of LowPart
    pub fn get_low_part(&self) -> Option<&u32> {
        self.low_part.as_ref()
    }
}

