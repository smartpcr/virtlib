// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ParticipatesInSet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ParticipatesInSet {

/// 
    #[serde(rename = "Element")]
    pub element: Option<CIM_PhysicalElement>,

/// 
    #[serde(rename = "Set")]
    pub set: Option<CIM_ReplacementSet>,
}

impl CIM_ParticipatesInSet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            element: None,
            set: None,
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

    /// Sets the value of Set
    pub fn set_set(&mut self, value: CIM_ReplacementSet) {
        self.set = Some(value);
    }

    /// Gets the value of Set
    pub fn get_set(&self) -> Option<&CIM_ReplacementSet> {
        self.set.as_ref()
    }
}

