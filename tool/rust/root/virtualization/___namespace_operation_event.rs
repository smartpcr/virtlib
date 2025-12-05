// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __NamespaceOperationEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __NamespaceOperationEvent {
    #[serde(flatten)]
    pub base: __Event,

/// 
    #[serde(rename = "TargetNamespace")]
    pub target_namespace: Option<__Namespace>,
}

impl __NamespaceOperationEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __Event::new(),
            target_namespace: None,
        }
    }


    /// Sets the value of TargetNamespace
    pub fn set_target_namespace(&mut self, value: __Namespace) {
        self.target_namespace = Some(value);
    }

    /// Gets the value of TargetNamespace
    pub fn get_target_namespace(&self) -> Option<&__Namespace> {
        self.target_namespace.as_ref()
    }
}

