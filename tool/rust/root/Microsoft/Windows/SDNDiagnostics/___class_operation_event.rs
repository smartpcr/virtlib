// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SDNDiagnostics
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __ClassOperationEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __ClassOperationEvent {
    #[serde(flatten)]
    pub base: __Event,

/// 
    #[serde(rename = "TargetClass")]
    pub target_class: Option<serde_json::Value>,
}

impl __ClassOperationEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __Event::new(),
            target_class: None,
        }
    }


    /// Sets the value of TargetClass
    pub fn set_target_class(&mut self, value: serde_json::Value) {
        self.target_class = Some(value);
    }

    /// Gets the value of TargetClass
    pub fn get_target_class(&self) -> Option<&serde_json::Value> {
        self.target_class.as_ref()
    }
}

