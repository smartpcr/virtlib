// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __ClassCreationEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __ClassCreationEvent {
    #[serde(flatten)]
    pub base: __ClassOperationEvent,
}

impl __ClassCreationEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ClassOperationEvent::new(),
        }
    }

}

