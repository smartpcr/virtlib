// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PowerManagementEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PowerManagementEvent {
    #[serde(flatten)]
    pub base: __ExtrinsicEvent,

/// 
    #[serde(rename = "EventType")]
    pub event_type: Option<u16>,

/// 
    #[serde(rename = "OEMEventCode")]
    pub oemevent_code: Option<u16>,
}

impl Win32_PowerManagementEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtrinsicEvent::new(),
            event_type: None,
            oemevent_code: None,
        }
    }


    /// Sets the value of EventType
    pub fn set_event_type(&mut self, value: u16) {
        self.event_type = Some(value);
    }

    /// Gets the value of EventType
    pub fn get_event_type(&self) -> Option<&u16> {
        self.event_type.as_ref()
    }

    /// Sets the value of OEMEventCode
    pub fn set_oemevent_code(&mut self, value: u16) {
        self.oemevent_code = Some(value);
    }

    /// Gets the value of OEMEventCode
    pub fn get_oemevent_code(&self) -> Option<&u16> {
        self.oemevent_code.as_ref()
    }
}

