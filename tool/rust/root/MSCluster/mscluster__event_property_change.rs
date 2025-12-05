// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_EventPropertyChange struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_EventPropertyChange {
    #[serde(flatten)]
    pub base: MSCluster_Event,

/// 
    #[serde(rename = "EventProperty")]
    pub event_property: Option<String>,
}

impl MSCluster_EventPropertyChange {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_Event::new(),
            event_property: None,
        }
    }


    /// Sets the value of EventProperty
    pub fn set_event_property(&mut self, value: String) {
        self.event_property = Some(value);
    }

    /// Gets the value of EventProperty
    pub fn get_event_property(&self) -> Option<&String> {
        self.event_property.as_ref()
    }
}

