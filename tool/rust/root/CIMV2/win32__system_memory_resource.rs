// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SystemMemoryResource struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SystemMemoryResource {
    #[serde(flatten)]
    pub base: CIM_MemoryMappedIO,
}

impl Win32_SystemMemoryResource {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_MemoryMappedIO::new(),
        }
    }

}

