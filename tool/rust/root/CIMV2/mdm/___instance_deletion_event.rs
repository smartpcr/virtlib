// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __InstanceDeletionEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __InstanceDeletionEvent {
    #[serde(flatten)]
    pub base: __InstanceOperationEvent,
}

impl __InstanceDeletionEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __InstanceOperationEvent::new(),
        }
    }

}

