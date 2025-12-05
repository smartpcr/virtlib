// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __NamespaceCreationEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __NamespaceCreationEvent {
    #[serde(flatten)]
    pub base: __NamespaceOperationEvent,
}

impl __NamespaceCreationEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __NamespaceOperationEvent::new(),
        }
    }

}

