// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WorkflowOperationBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowOperationBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// Specifies whether an Activation message can be processed by the Operation.
    #[serde(rename = "CanCreateInstance")]
    pub can_create_instance: Option<bool>,

/// Specifies whether the Operation is invoked Synchronously or not.
    #[serde(rename = "SynchronousDispatch")]
    pub synchronous_dispatch: Option<bool>,
}

impl WorkflowOperationBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            can_create_instance: None,
            synchronous_dispatch: None,
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

    /// Sets the value of SynchronousDispatch
    pub fn set_synchronous_dispatch(&mut self, value: bool) {
        self.synchronous_dispatch = Some(value);
    }

    /// Gets the value of SynchronousDispatch
    pub fn get_synchronous_dispatch(&self) -> Option<&bool> {
        self.synchronous_dispatch.as_ref()
    }
}

