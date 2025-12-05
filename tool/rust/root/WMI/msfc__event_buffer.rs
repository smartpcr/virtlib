// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_EventBuffer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_EventBuffer {

/// 
    #[serde(rename = "EventInfo")]
    pub event_info: Vec<u32>,

/// 
    #[serde(rename = "EventType")]
    pub event_type: Option<u32>,
}

impl MSFC_EventBuffer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            event_info: Vec::new(),
            event_type: None,
        }
    }


    /// Sets the value of EventInfo
    pub fn set_event_info(&mut self, value: Vec<u32>) {
        self.event_info = value;
    }

    /// Gets the value of EventInfo
    pub fn get_event_info(&self) -> &Vec<u32> {
        &self.event_info
    }

    /// Sets the value of EventType
    pub fn set_event_type(&mut self, value: u32) {
        self.event_type = Some(value);
    }

    /// Gets the value of EventType
    pub fn get_event_type(&self) -> Option<&u32> {
        self.event_type.as_ref()
    }
}

