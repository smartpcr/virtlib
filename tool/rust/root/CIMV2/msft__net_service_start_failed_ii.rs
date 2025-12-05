// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetServiceStartFailedII struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetServiceStartFailedII {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "DependedOnService")]
    pub depended_on_service: Option<String>,

/// 
    #[serde(rename = "Error")]
    pub error: Option<u32>,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,
}

impl MSFT_NetServiceStartFailedII {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            depended_on_service: None,
            error: None,
            service: None,
        }
    }


    /// Sets the value of DependedOnService
    pub fn set_depended_on_service(&mut self, value: String) {
        self.depended_on_service = Some(value);
    }

    /// Gets the value of DependedOnService
    pub fn get_depended_on_service(&self) -> Option<&String> {
        self.depended_on_service.as_ref()
    }

    /// Sets the value of Error
    pub fn set_error(&mut self, value: u32) {
        self.error = Some(value);
    }

    /// Gets the value of Error
    pub fn get_error(&self) -> Option<&u32> {
        self.error.as_ref()
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

