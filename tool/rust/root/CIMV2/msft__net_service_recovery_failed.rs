// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetServiceRecoveryFailed struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetServiceRecoveryFailed {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "Action")]
    pub action: Option<String>,

/// 
    #[serde(rename = "ActionType")]
    pub action_type: Option<u32>,

/// 
    #[serde(rename = "Error")]
    pub error: Option<u32>,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,
}

impl MSFT_NetServiceRecoveryFailed {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            action: None,
            action_type: None,
            error: None,
            service: None,
        }
    }


    /// Sets the value of Action
    pub fn set_action(&mut self, value: String) {
        self.action = Some(value);
    }

    /// Gets the value of Action
    pub fn get_action(&self) -> Option<&String> {
        self.action.as_ref()
    }

    /// Sets the value of ActionType
    pub fn set_action_type(&mut self, value: u32) {
        self.action_type = Some(value);
    }

    /// Gets the value of ActionType
    pub fn get_action_type(&self) -> Option<&u32> {
        self.action_type.as_ref()
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

