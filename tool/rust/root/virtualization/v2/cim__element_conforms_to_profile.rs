// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;
use Microsoft.Test.Wmi.root.Interop;


/// CIM_ElementConformsToProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ElementConformsToProfile {

/// The RegisteredProfile to which the ManagedElement conforms.
    #[serde(rename = "ConformantStandard")]
    pub conformant_standard: Option<CIM_RegisteredProfile>,

/// The ManagedElement that conforms to the RegisteredProfile.
    #[serde(rename = "ManagedElement")]
    pub managed_element: Option<CIM_ManagedElement>,
}

impl CIM_ElementConformsToProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            conformant_standard: None,
            managed_element: None,
        }
    }


    /// Sets the value of ConformantStandard
    pub fn set_conformant_standard(&mut self, value: CIM_RegisteredProfile) {
        self.conformant_standard = Some(value);
    }

    /// Gets the value of ConformantStandard
    pub fn get_conformant_standard(&self) -> Option<&CIM_RegisteredProfile> {
        self.conformant_standard.as_ref()
    }

    /// Sets the value of ManagedElement
    pub fn set_managed_element(&mut self, value: CIM_ManagedElement) {
        self.managed_element = Some(value);
    }

    /// Gets the value of ManagedElement
    pub fn get_managed_element(&self) -> Option<&CIM_ManagedElement> {
        self.managed_element.as_ref()
    }
}

