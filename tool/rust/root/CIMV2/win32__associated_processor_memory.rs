// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_AssociatedProcessorMemory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_AssociatedProcessorMemory {
    #[serde(flatten)]
    pub base: CIM_AssociatedProcessorMemory,
}

impl Win32_AssociatedProcessorMemory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_AssociatedProcessorMemory::new(),
        }
    }

}

