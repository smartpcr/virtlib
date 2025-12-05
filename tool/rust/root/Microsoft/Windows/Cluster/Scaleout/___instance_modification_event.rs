// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Scaleout
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __InstanceModificationEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __InstanceModificationEvent {
    #[serde(flatten)]
    pub base: __InstanceOperationEvent,

/// 
    #[serde(rename = "PreviousInstance")]
    pub previous_instance: Option<serde_json::Value>,
}

impl __InstanceModificationEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __InstanceOperationEvent::new(),
            previous_instance: None,
        }
    }


    /// Sets the value of PreviousInstance
    pub fn set_previous_instance(&mut self, value: serde_json::Value) {
        self.previous_instance = Some(value);
    }

    /// Gets the value of PreviousInstance
    pub fn get_previous_instance(&self) -> Option<&serde_json::Value> {
        self.previous_instance.as_ref()
    }
}

