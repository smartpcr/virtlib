// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServiceToMonitor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServiceToMonitor {

/// 
    #[serde(rename = "DefaultMonitoring")]
    pub default_monitoring: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl MSFT_ServiceToMonitor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            default_monitoring: None,
            name: None,
        }
    }


    /// Sets the value of DefaultMonitoring
    pub fn set_default_monitoring(&mut self, value: bool) {
        self.default_monitoring = Some(value);
    }

    /// Gets the value of DefaultMonitoring
    pub fn get_default_monitoring(&self) -> Option<&bool> {
        self.default_monitoring.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

