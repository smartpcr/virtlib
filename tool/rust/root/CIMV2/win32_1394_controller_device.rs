// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_1394ControllerDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_1394ControllerDevice {
    #[serde(flatten)]
    pub base: CIM_ControlledBy,
}

impl Win32_1394ControllerDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ControlledBy::new(),
        }
    }

}

