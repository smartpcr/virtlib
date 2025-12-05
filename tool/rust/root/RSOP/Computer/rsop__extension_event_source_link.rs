// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_ExtensionEventSourceLink struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_ExtensionEventSourceLink {

/// 
    #[serde(rename = "eventSource")]
    pub event_source: Option<RSOP_ExtensionEventSource>,

/// 
    #[serde(rename = "extensionStatus")]
    pub extension_status: Option<RSOP_ExtensionStatus>,
}

impl RSOP_ExtensionEventSourceLink {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            event_source: None,
            extension_status: None,
        }
    }


    /// Sets the value of eventSource
    pub fn set_event_source(&mut self, value: RSOP_ExtensionEventSource) {
        self.event_source = Some(value);
    }

    /// Gets the value of eventSource
    pub fn get_event_source(&self) -> Option<&RSOP_ExtensionEventSource> {
        self.event_source.as_ref()
    }

    /// Sets the value of extensionStatus
    pub fn set_extension_status(&mut self, value: RSOP_ExtensionStatus) {
        self.extension_status = Some(value);
    }

    /// Gets the value of extensionStatus
    pub fn get_extension_status(&self) -> Option<&RSOP_ExtensionStatus> {
        self.extension_status.as_ref()
    }
}

