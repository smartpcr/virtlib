// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Validation
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __InstanceCreationEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __InstanceCreationEvent {
    #[serde(flatten)]
    pub base: __InstanceOperationEvent,
}

impl __InstanceCreationEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __InstanceOperationEvent::new(),
        }
    }

}

