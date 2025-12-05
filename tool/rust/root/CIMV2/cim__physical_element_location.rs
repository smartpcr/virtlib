// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PhysicalElementLocation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PhysicalElementLocation {

/// 
    #[serde(rename = "Element")]
    pub element: Option<CIM_PhysicalElement>,

/// 
    #[serde(rename = "PhysicalLocation")]
    pub physical_location: Option<CIM_Location>,
}

impl CIM_PhysicalElementLocation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            element: None,
            physical_location: None,
        }
    }


    /// Sets the value of Element
    pub fn set_element(&mut self, value: CIM_PhysicalElement) {
        self.element = Some(value);
    }

    /// Gets the value of Element
    pub fn get_element(&self) -> Option<&CIM_PhysicalElement> {
        self.element.as_ref()
    }

    /// Sets the value of PhysicalLocation
    pub fn set_physical_location(&mut self, value: CIM_Location) {
        self.physical_location = Some(value);
    }

    /// Gets the value of PhysicalLocation
    pub fn get_physical_location(&self) -> Option<&CIM_Location> {
        self.physical_location.as_ref()
    }
}

