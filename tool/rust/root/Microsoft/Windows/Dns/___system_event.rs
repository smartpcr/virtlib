// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Dns
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __SystemEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __SystemEvent {
    #[serde(flatten)]
    pub base: __ExtrinsicEvent,
}

impl __SystemEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtrinsicEvent::new(),
        }
    }

}

