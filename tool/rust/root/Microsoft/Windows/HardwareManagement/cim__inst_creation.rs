// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.HardwareManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_InstCreation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_InstCreation {
    #[serde(flatten)]
    pub base: CIM_InstIndication,
}

impl CIM_InstCreation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_InstIndication::new(),
        }
    }

}

