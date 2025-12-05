// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetServiceConfigBackoutFailed struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetServiceConfigBackoutFailed {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "ConfigField")]
    pub config_field: Option<String>,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,
}

impl MSFT_NetServiceConfigBackoutFailed {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            config_field: None,
            service: None,
        }
    }


    /// Sets the value of ConfigField
    pub fn set_config_field(&mut self, value: String) {
        self.config_field = Some(value);
    }

    /// Gets the value of ConfigField
    pub fn get_config_field(&self) -> Option<&String> {
        self.config_field.as_ref()
    }

    /// Sets the value of Service
    pub fn set_service(&mut self, value: String) {
        self.service = Some(value);
    }

    /// Gets the value of Service
    pub fn get_service(&self) -> Option<&String> {
        self.service.as_ref()
    }
}

