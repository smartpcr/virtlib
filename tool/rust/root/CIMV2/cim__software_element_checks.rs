// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SoftwareElementChecks struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SoftwareElementChecks {

/// 
    #[serde(rename = "Check")]
    pub check: Option<CIM_Check>,

/// 
    #[serde(rename = "Element")]
    pub element: Option<CIM_SoftwareElement>,

/// 
    #[serde(rename = "Phase")]
    pub phase: Option<u16>,
}

impl CIM_SoftwareElementChecks {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            check: None,
            element: None,
            phase: None,
        }
    }


    /// Sets the value of Check
    pub fn set_check(&mut self, value: CIM_Check) {
        self.check = Some(value);
    }

    /// Gets the value of Check
    pub fn get_check(&self) -> Option<&CIM_Check> {
        self.check.as_ref()
    }

    /// Sets the value of Element
    pub fn set_element(&mut self, value: CIM_SoftwareElement) {
        self.element = Some(value);
    }

    /// Gets the value of Element
    pub fn get_element(&self) -> Option<&CIM_SoftwareElement> {
        self.element.as_ref()
    }

    /// Sets the value of Phase
    pub fn set_phase(&mut self, value: u16) {
        self.phase = Some(value);
    }

    /// Gets the value of Phase
    pub fn get_phase(&self) -> Option<&u16> {
        self.phase.as_ref()
    }
}

