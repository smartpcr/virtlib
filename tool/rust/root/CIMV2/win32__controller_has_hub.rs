// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ControllerHasHub struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ControllerHasHub {
    #[serde(flatten)]
    pub base: CIM_USBControllerHasHub,
}

impl Win32_ControllerHasHub {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_USBControllerHasHub::new(),
        }
    }

}

