// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DeviceFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DeviceFile {
    #[serde(flatten)]
    pub base: CIM_LogicalFile,
}

impl CIM_DeviceFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalFile::new(),
        }
    }

}

