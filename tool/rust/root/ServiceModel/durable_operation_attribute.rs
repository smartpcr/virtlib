// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DurableOperationAttribute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DurableOperationAttribute {
    #[serde(flatten)]
    pub base: Behavior,

/// Specifies whether an Activation message can be processed by the Operation.
    #[serde(rename = "CanCreateInstance")]
    pub can_create_instance: Option<bool>,

/// Specifies whether the runtime will complete the instance after the Operation.
    #[serde(rename = "CompletesInstance")]
    pub completes_instance: Option<bool>,
}

impl DurableOperationAttribute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            can_create_instance: None,
            completes_instance: None,
        }
    }


    /// Sets the value of CanCreateInstance
    pub fn set_can_create_instance(&mut self, value: bool) {
        self.can_create_instance = Some(value);
    }

    /// Gets the value of CanCreateInstance
    pub fn get_can_create_instance(&self) -> Option<&bool> {
        self.can_create_instance.as_ref()
    }

    /// Sets the value of CompletesInstance
    pub fn set_completes_instance(&mut self, value: bool) {
        self.completes_instance = Some(value);
    }

    /// Gets the value of CompletesInstance
    pub fn get_completes_instance(&self) -> Option<&bool> {
        self.completes_instance.as_ref()
    }
}

