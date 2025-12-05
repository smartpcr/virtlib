// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.power
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PowerMeterEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PowerMeterEvent {
    #[serde(flatten)]
    pub base: __ExtrinsicEvent,

/// 
    #[serde(rename = "EventSource")]
    pub event_source: Option<CIM_PowerMeter>,

/// 
    #[serde(rename = "EventType")]
    pub event_type: Option<u32>,
}

impl Win32_PowerMeterEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtrinsicEvent::new(),
            event_source: None,
            event_type: None,
        }
    }


    /// Sets the value of EventSource
    pub fn set_event_source(&mut self, value: CIM_PowerMeter) {
        self.event_source = Some(value);
    }

    /// Gets the value of EventSource
    pub fn get_event_source(&self) -> Option<&CIM_PowerMeter> {
        self.event_source.as_ref()
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

