// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.DEFAULT
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RegistryEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegistryEvent {
    #[serde(flatten)]
    pub base: __ExtrinsicEvent,
}

impl RegistryEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtrinsicEvent::new(),
        }
    }

}

