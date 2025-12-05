// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ComputerSystemDMA struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ComputerSystemDMA {
    #[serde(flatten)]
    pub base: CIM_ComputerSystemResource,
}

impl CIM_ComputerSystemDMA {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ComputerSystemResource::new(),
        }
    }

}

