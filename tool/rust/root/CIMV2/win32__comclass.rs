// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_COMClass struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_COMClass {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,
}

impl Win32_COMClass {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
        }
    }

}

