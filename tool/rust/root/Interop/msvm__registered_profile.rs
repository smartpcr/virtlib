// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Interop
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_RegisteredProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_RegisteredProfile {
    #[serde(flatten)]
    pub base: CIM_RegisteredProfile,
}

impl Msvm_RegisteredProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_RegisteredProfile::new(),
        }
    }

}

impl Msvm_RegisteredProfile {
    /// Gets the related Msvm_RegisteredProfile object(s)
    pub fn get_related__registered_profile(&self) -> Result<Msvm_RegisteredProfile, WmiError> {
        self.get_related("Msvm_RegisteredProfile")
    }

}

