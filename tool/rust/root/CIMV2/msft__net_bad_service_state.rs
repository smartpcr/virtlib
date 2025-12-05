// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBadServiceState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBadServiceState {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,
}

impl MSFT_NetBadServiceState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            service: None,
            state: None,
        }
    }


    /// Sets the value of Service
    pub fn set_service(&mut self, value: String) {
        self.service = Some(value);
    }

    /// Gets the value of Service
    pub fn get_service(&self) -> Option<&String> {
        self.service.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }
}

