// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __ExtrinsicEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __ExtrinsicEvent {
    #[serde(flatten)]
    pub base: __Event,
}

impl __ExtrinsicEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __Event::new(),
        }
    }

}

