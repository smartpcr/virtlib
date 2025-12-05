// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Directory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Directory {
    #[serde(flatten)]
    pub base: CIM_LogicalFile,
}

impl CIM_Directory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalFile::new(),
        }
    }

}

