// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetServiceCrash struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetServiceCrash {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "Action")]
    pub action: Option<String>,

/// 
    #[serde(rename = "ActionDelay")]
    pub action_delay: Option<u32>,

/// 
    #[serde(rename = "ActionType")]
    pub action_type: Option<u32>,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,

/// 
    #[serde(rename = "TimesFailed")]
    pub times_failed: Option<u32>,
}

impl MSFT_NetServiceCrash {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            action: None,
            action_delay: None,
            action_type: None,
            service: None,
            times_failed: None,
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

    /// Sets the value of ActionDelay
    pub fn set_action_delay(&mut self, value: u32) {
        self.action_delay = Some(value);
    }

    /// Gets the value of ActionDelay
    pub fn get_action_delay(&self) -> Option<&u32> {
        self.action_delay.as_ref()
    }

    /// Sets the value of ActionType
    pub fn set_action_type(&mut self, value: u32) {
        self.action_type = Some(value);
    }

    /// Gets the value of ActionType
    pub fn get_action_type(&self) -> Option<&u32> {
        self.action_type.as_ref()
    }

    /// Sets the value of Service
    pub fn set_service(&mut self, value: String) {
        self.service = Some(value);
    }

    /// Gets the value of Service
    pub fn get_service(&self) -> Option<&String> {
        self.service.as_ref()
    }

    /// Sets the value of TimesFailed
    pub fn set_times_failed(&mut self, value: u32) {
        self.times_failed = Some(value);
    }

    /// Gets the value of TimesFailed
    pub fn get_times_failed(&self) -> Option<&u32> {
        self.times_failed.as_ref()
    }
}

