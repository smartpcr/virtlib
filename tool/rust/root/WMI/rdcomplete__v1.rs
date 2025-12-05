// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RDComplete_V1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RDComplete_V1 {
    #[serde(flatten)]
    pub base: EventTraceEvent_V1,
}

impl RDComplete_V1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTraceEvent_V1::new(),
        }
    }

}

