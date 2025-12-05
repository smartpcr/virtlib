// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_CurrentProbe struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_CurrentProbe {
    #[serde(flatten)]
    pub base: CIM_CurrentSensor,
}

impl Win32_CurrentProbe {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_CurrentSensor::new(),
        }
    }

}

