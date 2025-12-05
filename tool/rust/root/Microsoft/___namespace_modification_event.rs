// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __NamespaceModificationEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __NamespaceModificationEvent {
    #[serde(flatten)]
    pub base: __NamespaceOperationEvent,

/// 
    #[serde(rename = "PreviousNamespace")]
    pub previous_namespace: Option<__Namespace>,
}

impl __NamespaceModificationEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __NamespaceOperationEvent::new(),
            previous_namespace: None,
        }
    }


    /// Sets the value of PreviousNamespace
    pub fn set_previous_namespace(&mut self, value: __Namespace) {
        self.previous_namespace = Some(value);
    }

    /// Gets the value of PreviousNamespace
    pub fn get_previous_namespace(&self) -> Option<&__Namespace> {
        self.previous_namespace.as_ref()
    }
}

