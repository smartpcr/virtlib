// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PhysicalComponent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PhysicalComponent {
    #[serde(flatten)]
    pub base: CIM_PhysicalElement,

/// 
    #[serde(rename = "HotSwappable")]
    pub hot_swappable: Option<bool>,

/// 
    #[serde(rename = "Removable")]
    pub removable: Option<bool>,

/// 
    #[serde(rename = "Replaceable")]
    pub replaceable: Option<bool>,
}

impl CIM_PhysicalComponent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalElement::new(),
            hot_swappable: None,
            removable: None,
            replaceable: None,
        }
    }


    /// Sets the value of HotSwappable
    pub fn set_hot_swappable(&mut self, value: bool) {
        self.hot_swappable = Some(value);
    }

    /// Gets the value of HotSwappable
    pub fn get_hot_swappable(&self) -> Option<&bool> {
        self.hot_swappable.as_ref()
    }

    /// Sets the value of Removable
    pub fn set_removable(&mut self, value: bool) {
        self.removable = Some(value);
    }

    /// Gets the value of Removable
    pub fn get_removable(&self) -> Option<&bool> {
        self.removable.as_ref()
    }

    /// Sets the value of Replaceable
    pub fn set_replaceable(&mut self, value: bool) {
        self.replaceable = Some(value);
    }

    /// Gets the value of Replaceable
    pub fn get_replaceable(&self) -> Option<&bool> {
        self.replaceable.as_ref()
    }
}

