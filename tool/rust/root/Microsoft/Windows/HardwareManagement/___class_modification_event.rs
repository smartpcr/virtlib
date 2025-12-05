// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.HardwareManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __ClassModificationEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __ClassModificationEvent {
    #[serde(flatten)]
    pub base: __ClassOperationEvent,

/// 
    #[serde(rename = "PreviousClass")]
    pub previous_class: Option<serde_json::Value>,
}

impl __ClassModificationEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ClassOperationEvent::new(),
            previous_class: None,
        }
    }


    /// Sets the value of PreviousClass
    pub fn set_previous_class(&mut self, value: serde_json::Value) {
        self.previous_class = Some(value);
    }

    /// Gets the value of PreviousClass
    pub fn get_previous_class(&self) -> Option<&serde_json::Value> {
        self.previous_class.as_ref()
    }
}

