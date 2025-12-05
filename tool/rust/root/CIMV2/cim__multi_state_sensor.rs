// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_MultiStateSensor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_MultiStateSensor {
    #[serde(flatten)]
    pub base: CIM_Sensor,
}

impl CIM_MultiStateSensor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Sensor::new(),
        }
    }

}

