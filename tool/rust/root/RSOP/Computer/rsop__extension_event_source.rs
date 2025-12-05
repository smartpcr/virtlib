// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_ExtensionEventSource struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_ExtensionEventSource {

/// 
    #[serde(rename = "eventLogName")]
    pub event_log_name: Option<String>,

/// 
    #[serde(rename = "eventLogSource")]
    pub event_log_source: Option<String>,

/// 
    #[serde(rename = "id")]
    pub id: Option<String>,
}

impl RSOP_ExtensionEventSource {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            event_log_name: None,
            event_log_source: None,
            id: None,
        }
    }


    /// Sets the value of eventLogName
    pub fn set_event_log_name(&mut self, value: String) {
        self.event_log_name = Some(value);
    }

    /// Gets the value of eventLogName
    pub fn get_event_log_name(&self) -> Option<&String> {
        self.event_log_name.as_ref()
    }

    /// Sets the value of eventLogSource
    pub fn set_event_log_source(&mut self, value: String) {
        self.event_log_source = Some(value);
    }

    /// Gets the value of eventLogSource
    pub fn get_event_log_source(&self) -> Option<&String> {
        self.event_log_source.as_ref()
    }

    /// Sets the value of id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }
}

