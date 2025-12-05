// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_USBHub struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_USBHub {
    #[serde(flatten)]
    pub base: CIM_USBHub,
}

impl Win32_USBHub {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_USBHub::new(),
        }
    }

}

