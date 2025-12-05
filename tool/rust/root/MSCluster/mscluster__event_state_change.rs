// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_EventStateChange struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_EventStateChange {
    #[serde(flatten)]
    pub base: MSCluster_Event,

/// 
    #[serde(rename = "EventNewState")]
    pub event_new_state: Option<u32>,
}

impl MSCluster_EventStateChange {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_Event::new(),
            event_new_state: None,
        }
    }


    /// Sets the value of EventNewState
    pub fn set_event_new_state(&mut self, value: u32) {
        self.event_new_state = Some(value);
    }

    /// Gets the value of EventNewState
    pub fn get_event_new_state(&self) -> Option<&u32> {
        self.event_new_state.as_ref()
    }
}

