// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DebuggerEnabled struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DebuggerEnabled {
    #[serde(flatten)]
    pub base: PerfInfo_V2,
}

impl DebuggerEnabled {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
        }
    }

}

