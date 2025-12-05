// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DSCConfigurationOutputWriteError struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DSCConfigurationOutputWriteError {
    #[serde(flatten)]
    pub base: MSFT_DSCConfigurationOutput,

/// 
    #[serde(rename = "Error")]
    pub error: Option<serde_json::Value>,
}

impl MSFT_DSCConfigurationOutputWriteError {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_DSCConfigurationOutput::new(),
            error: None,
        }
    }


    /// Sets the value of Error
    pub fn set_error(&mut self, value: serde_json::Value) {
        self.error = Some(value);
    }

    /// Gets the value of Error
    pub fn get_error(&self) -> Option<&serde_json::Value> {
        self.error.as_ref()
    }
}

