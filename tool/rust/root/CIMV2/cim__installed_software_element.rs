// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_InstalledSoftwareElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_InstalledSoftwareElement {

/// 
    #[serde(rename = "Software")]
    pub software: Option<CIM_SoftwareElement>,

/// 
    #[serde(rename = "System")]
    pub system: Option<CIM_ComputerSystem>,
}

impl CIM_InstalledSoftwareElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            software: None,
            system: None,
        }
    }


    /// Sets the value of Software
    pub fn set_software(&mut self, value: CIM_SoftwareElement) {
        self.software = Some(value);
    }

    /// Gets the value of Software
    pub fn get_software(&self) -> Option<&CIM_SoftwareElement> {
        self.software.as_ref()
    }

    /// Sets the value of System
    pub fn set_system(&mut self, value: CIM_ComputerSystem) {
        self.system = Some(value);
    }

    /// Gets the value of System
    pub fn get_system(&self) -> Option<&CIM_ComputerSystem> {
        self.system.as_ref()
    }
}

