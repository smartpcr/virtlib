// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.AccessLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExtendedStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExtendedStatus {
    #[serde(flatten)]
    pub base: MSFT_WmiError,

/// 
    #[serde(rename = "original_error")]
    pub original_error: Option<serde_json::Value>,
}

impl MSFT_ExtendedStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_WmiError::new(),
            original_error: None,
        }
    }


    /// Sets the value of original_error
    pub fn set_original_error(&mut self, value: serde_json::Value) {
        self.original_error = Some(value);
    }

    /// Gets the value of original_error
    pub fn get_original_error(&self) -> Option<&serde_json::Value> {
        self.original_error.as_ref()
    }
}

