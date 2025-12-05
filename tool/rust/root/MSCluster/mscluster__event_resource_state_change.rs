// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_EventResourceStateChange struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_EventResourceStateChange {
    #[serde(flatten)]
    pub base: MSCluster_EventStateChange,

/// 
    #[serde(rename = "EventGroup")]
    pub event_group: Option<String>,

/// 
    #[serde(rename = "EventNode")]
    pub event_node: Option<String>,
}

impl MSCluster_EventResourceStateChange {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_EventStateChange::new(),
            event_group: None,
            event_node: None,
        }
    }


    /// Sets the value of EventGroup
    pub fn set_event_group(&mut self, value: String) {
        self.event_group = Some(value);
    }

    /// Gets the value of EventGroup
    pub fn get_event_group(&self) -> Option<&String> {
        self.event_group.as_ref()
    }

    /// Sets the value of EventNode
    pub fn set_event_node(&mut self, value: String) {
        self.event_node = Some(value);
    }

    /// Gets the value of EventNode
    pub fn get_event_node(&self) -> Option<&String> {
        self.event_node.as_ref()
    }
}

