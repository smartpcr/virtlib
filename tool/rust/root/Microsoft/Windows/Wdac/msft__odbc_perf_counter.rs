// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Wdac
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_OdbcPerfCounter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_OdbcPerfCounter {

/// 
    #[serde(rename = "IsEnabled")]
    pub is_enabled: Option<bool>,

/// 
    #[serde(rename = "Platform")]
    pub platform: Option<String>,
}

impl MSFT_OdbcPerfCounter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            is_enabled: None,
            platform: None,
        }
    }


    /// Sets the value of IsEnabled
    pub fn set_is_enabled(&mut self, value: bool) {
        self.is_enabled = Some(value);
    }

    /// Gets the value of IsEnabled
    pub fn get_is_enabled(&self) -> Option<&bool> {
        self.is_enabled.as_ref()
    }

    /// Sets the value of Platform
    pub fn set_platform(&mut self, value: String) {
        self.platform = Some(value);
    }

    /// Gets the value of Platform
    pub fn get_platform(&self) -> Option<&String> {
        self.platform.as_ref()
    }
}

