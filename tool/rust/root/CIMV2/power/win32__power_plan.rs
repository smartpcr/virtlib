// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.power
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PowerPlan struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PowerPlan {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "IsActive")]
    pub is_active: Option<bool>,
}

impl Win32_PowerPlan {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            is_active: None,
        }
    }


    /// Sets the value of IsActive
    pub fn set_is_active(&mut self, value: bool) {
        self.is_active = Some(value);
    }

    /// Gets the value of IsActive
    pub fn get_is_active(&self) -> Option<&bool> {
        self.is_active.as_ref()
    }

/// 

    /// * `return_value` -  (bool)
    pub fn activate(&self) -> Result<(), WmiError> {
        self.invoke_method("Activate", &[])

    }

}

