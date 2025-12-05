// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_FRUPhysicalElements struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_FRUPhysicalElements {

/// 
    #[serde(rename = "Component")]
    pub component: Option<CIM_PhysicalElement>,

/// 
    #[serde(rename = "FRU")]
    pub fru: Option<CIM_FRU>,
}

impl CIM_FRUPhysicalElements {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            component: None,
            fru: None,
        }
    }


    /// Sets the value of Component
    pub fn set_component(&mut self, value: CIM_PhysicalElement) {
        self.component = Some(value);
    }

    /// Gets the value of Component
    pub fn get_component(&self) -> Option<&CIM_PhysicalElement> {
        self.component.as_ref()
    }

    /// Sets the value of FRU
    pub fn set_fru(&mut self, value: CIM_FRU) {
        self.fru = Some(value);
    }

    /// Gets the value of FRU
    pub fn get_fru(&self) -> Option<&CIM_FRU> {
        self.fru.as_ref()
    }
}

