// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.vs
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __InstanceOperationEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __InstanceOperationEvent {
    #[serde(flatten)]
    pub base: __Event,

/// 
    #[serde(rename = "TargetInstance")]
    pub target_instance: Option<serde_json::Value>,
}

impl __InstanceOperationEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __Event::new(),
            target_instance: None,
        }
    }


    /// Sets the value of TargetInstance
    pub fn set_target_instance(&mut self, value: serde_json::Value) {
        self.target_instance = Some(value);
    }

    /// Gets the value of TargetInstance
    pub fn get_target_instance(&self) -> Option<&serde_json::Value> {
        self.target_instance.as_ref()
    }
}

