// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSV1_0Trace struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSV1_0Trace {
    #[serde(flatten)]
    pub base: EventTrace,
}

impl MSV1_0Trace {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTrace::new(),
        }
    }

}

