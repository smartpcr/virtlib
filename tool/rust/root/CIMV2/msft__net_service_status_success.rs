// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetServiceStatusSuccess struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetServiceStatusSuccess {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "Control")]
    pub control: Option<String>,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,
}

impl MSFT_NetServiceStatusSuccess {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            control: None,
            service: None,
        }
    }


    /// Sets the value of Control
    pub fn set_control(&mut self, value: String) {
        self.control = Some(value);
    }

    /// Gets the value of Control
    pub fn get_control(&self) -> Option<&String> {
        self.control.as_ref()
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

