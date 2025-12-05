// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_MemoryArrayLocation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_MemoryArrayLocation {
    #[serde(flatten)]
    pub base: CIM_Realizes,
}

impl Win32_MemoryArrayLocation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Realizes::new(),
        }
    }

}

