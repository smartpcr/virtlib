// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ElementCapacity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ElementCapacity {

/// 
    #[serde(rename = "Capacity")]
    pub capacity: Option<CIM_PhysicalCapacity>,

/// 
    #[serde(rename = "Element")]
    pub element: Option<CIM_PhysicalElement>,
}

impl CIM_ElementCapacity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            capacity: None,
            element: None,
        }
    }


    /// Sets the value of Capacity
    pub fn set_capacity(&mut self, value: CIM_PhysicalCapacity) {
        self.capacity = Some(value);
    }

    /// Gets the value of Capacity
    pub fn get_capacity(&self) -> Option<&CIM_PhysicalCapacity> {
        self.capacity.as_ref()
    }

    /// Sets the value of Element
    pub fn set_element(&mut self, value: CIM_PhysicalElement) {
        self.element = Some(value);
    }

    /// Gets the value of Element
    pub fn get_element(&self) -> Option<&CIM_PhysicalElement> {
        self.element.as_ref()
    }
}

