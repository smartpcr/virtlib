// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ElementSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ElementSetting {

/// 
    #[serde(rename = "Element")]
    pub element: Option<CIM_ManagedSystemElement>,

/// 
    #[serde(rename = "Setting")]
    pub setting: Option<CIM_Setting>,
}

impl CIM_ElementSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            element: None,
            setting: None,
        }
    }


    /// Sets the value of Element
    pub fn set_element(&mut self, value: CIM_ManagedSystemElement) {
        self.element = Some(value);
    }

    /// Gets the value of Element
    pub fn get_element(&self) -> Option<&CIM_ManagedSystemElement> {
        self.element.as_ref()
    }

    /// Sets the value of Setting
    pub fn set_setting(&mut self, value: CIM_Setting) {
        self.setting = Some(value);
    }

    /// Gets the value of Setting
    pub fn get_setting(&self) -> Option<&CIM_Setting> {
        self.setting.as_ref()
    }
}

