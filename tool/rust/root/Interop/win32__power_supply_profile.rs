// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Interop
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PowerSupplyProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PowerSupplyProfile {
    #[serde(flatten)]
    pub base: CIM_RegisteredProfile,
}

impl Win32_PowerSupplyProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_RegisteredProfile::new(),
        }
    }

}

