// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_Event struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_Event {
    #[serde(flatten)]
    pub base: __ExtrinsicEvent,

/// 
    #[serde(rename = "EventObjectName")]
    pub event_object_name: Option<String>,

/// 
    #[serde(rename = "EventObjectPath")]
    pub event_object_path: Option<String>,

/// 
    #[serde(rename = "EventObjectType")]
    pub event_object_type: Option<u32>,

/// 
    #[serde(rename = "EventTypeMajor")]
    pub event_type_major: Option<u32>,

/// 
    #[serde(rename = "EventTypeMinor")]
    pub event_type_minor: Option<u32>,
}

impl MSCluster_Event {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtrinsicEvent::new(),
            event_object_name: None,
            event_object_path: None,
            event_object_type: None,
            event_type_major: None,
            event_type_minor: None,
        }
    }


    /// Sets the value of EventObjectName
    pub fn set_event_object_name(&mut self, value: String) {
        self.event_object_name = Some(value);
    }

    /// Gets the value of EventObjectName
    pub fn get_event_object_name(&self) -> Option<&String> {
        self.event_object_name.as_ref()
    }

    /// Sets the value of EventObjectPath
    pub fn set_event_object_path(&mut self, value: String) {
        self.event_object_path = Some(value);
    }

    /// Gets the value of EventObjectPath
    pub fn get_event_object_path(&self) -> Option<&String> {
        self.event_object_path.as_ref()
    }

    /// Sets the value of EventObjectType
    pub fn set_event_object_type(&mut self, value: u32) {
        self.event_object_type = Some(value);
    }

    /// Gets the value of EventObjectType
    pub fn get_event_object_type(&self) -> Option<&u32> {
        self.event_object_type.as_ref()
    }

    /// Sets the value of EventTypeMajor
    pub fn set_event_type_major(&mut self, value: u32) {
        self.event_type_major = Some(value);
    }

    /// Gets the value of EventTypeMajor
    pub fn get_event_type_major(&self) -> Option<&u32> {
        self.event_type_major.as_ref()
    }

    /// Sets the value of EventTypeMinor
    pub fn set_event_type_minor(&mut self, value: u32) {
        self.event_type_minor = Some(value);
    }

    /// Gets the value of EventTypeMinor
    pub fn get_event_type_minor(&self) -> Option<&u32> {
        self.event_type_minor.as_ref()
    }
}

