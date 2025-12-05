// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __EventQueueOverflowEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __EventQueueOverflowEvent {
    #[serde(flatten)]
    pub base: __EventDroppedEvent,

/// 
    #[serde(rename = "CurrentQueueSize")]
    pub current_queue_size: Option<u32>,
}

impl __EventQueueOverflowEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __EventDroppedEvent::new(),
            current_queue_size: None,
        }
    }


    /// Sets the value of CurrentQueueSize
    pub fn set_current_queue_size(&mut self, value: u32) {
        self.current_queue_size = Some(value);
    }

    /// Gets the value of CurrentQueueSize
    pub fn get_current_queue_size(&self) -> Option<&u32> {
        self.current_queue_size.as_ref()
    }
}

