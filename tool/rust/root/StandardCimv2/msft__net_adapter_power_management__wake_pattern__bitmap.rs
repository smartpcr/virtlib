// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterPowerManagement_WakePattern_Bitmap struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterPowerManagement_WakePattern_Bitmap {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterPowerManagement_WakePattern,

/// 
    #[serde(rename = "Mask")]
    pub mask: Vec<u8>,

/// 
    #[serde(rename = "Pattern")]
    pub pattern: Vec<u8>,
}

impl MSFT_NetAdapterPowerManagement_WakePattern_Bitmap {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterPowerManagement_WakePattern::new(),
            mask: Vec::new(),
            pattern: Vec::new(),
        }
    }


    /// Sets the value of Mask
    pub fn set_mask(&mut self, value: Vec<u8>) {
        self.mask = value;
    }

    /// Gets the value of Mask
    pub fn get_mask(&self) -> &Vec<u8> {
        &self.mask
    }

    /// Sets the value of Pattern
    pub fn set_pattern(&mut self, value: Vec<u8>) {
        self.pattern = value;
    }

    /// Gets the value of Pattern
    pub fn get_pattern(&self) -> &Vec<u8> {
        &self.pattern
    }
}

