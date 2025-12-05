// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Scanner struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Scanner {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,
}

impl CIM_Scanner {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
        }
    }

}

