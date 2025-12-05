// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __EventDroppedEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __EventDroppedEvent {
    #[serde(flatten)]
    pub base: __SystemEvent,

/// 
    #[serde(rename = "Event")]
    pub event: Option<__Event>,

/// 
    #[serde(rename = "IntendedConsumer")]
    pub intended_consumer: Option<__EventConsumer>,
}

impl __EventDroppedEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SystemEvent::new(),
            event: None,
            intended_consumer: None,
        }
    }


    /// Sets the value of Event
    pub fn set_event(&mut self, value: __Event) {
        self.event = Some(value);
    }

    /// Gets the value of Event
    pub fn get_event(&self) -> Option<&__Event> {
        self.event.as_ref()
    }

    /// Sets the value of IntendedConsumer
    pub fn set_intended_consumer(&mut self, value: __EventConsumer) {
        self.intended_consumer = Some(value);
    }

    /// Gets the value of IntendedConsumer
    pub fn get_intended_consumer(&self) -> Option<&__EventConsumer> {
        self.intended_consumer.as_ref()
    }
}

