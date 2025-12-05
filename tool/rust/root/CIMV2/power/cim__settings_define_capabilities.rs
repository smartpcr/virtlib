// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.power
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SettingsDefineCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SettingsDefineCapabilities {
    #[serde(flatten)]
    pub base: CIM_Component,

/// 
    #[serde(rename = "PropertyPolicy")]
    pub property_policy: Option<u16>,

/// 
    #[serde(rename = "ValueRange")]
    pub value_range: Option<u16>,

/// 
    #[serde(rename = "ValueRole")]
    pub value_role: Option<u16>,
}

impl CIM_SettingsDefineCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Component::new(),
            property_policy: None,
            value_range: None,
            value_role: None,
        }
    }


    /// Sets the value of PropertyPolicy
    pub fn set_property_policy(&mut self, value: u16) {
        self.property_policy = Some(value);
    }

    /// Gets the value of PropertyPolicy
    pub fn get_property_policy(&self) -> Option<&u16> {
        self.property_policy.as_ref()
    }

    /// Sets the value of ValueRange
    pub fn set_value_range(&mut self, value: u16) {
        self.value_range = Some(value);
    }

    /// Gets the value of ValueRange
    pub fn get_value_range(&self) -> Option<&u16> {
        self.value_range.as_ref()
    }

    /// Sets the value of ValueRole
    pub fn set_value_role(&mut self, value: u16) {
        self.value_role = Some(value);
    }

    /// Gets the value of ValueRole
    pub fn get_value_role(&self) -> Option<&u16> {
        self.value_role.as_ref()
    }
}

