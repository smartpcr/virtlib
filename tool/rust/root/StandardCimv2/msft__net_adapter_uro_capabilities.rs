// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterUroCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterUroCapabilities {

/// 
    #[serde(rename = "Supported")]
    pub supported: Option<bool>,
}

impl MSFT_NetAdapterUroCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            supported: None,
        }
    }


    /// Sets the value of Supported
    pub fn set_supported(&mut self, value: bool) {
        self.supported = Some(value);
    }

    /// Gets the value of Supported
    pub fn get_supported(&self) -> Option<&bool> {
        self.supported.as_ref()
    }
}

