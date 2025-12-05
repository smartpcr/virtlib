// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SDNDiagnostics.Server
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __NamespaceDeletionEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __NamespaceDeletionEvent {
    #[serde(flatten)]
    pub base: __NamespaceOperationEvent,
}

impl __NamespaceDeletionEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __NamespaceOperationEvent::new(),
        }
    }

}

