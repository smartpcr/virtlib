// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PnPDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PnPDevice {

/// 
    #[serde(rename = "SameElement")]
    pub same_element: Option<CIM_LogicalDevice>,

/// 
    #[serde(rename = "SystemElement")]
    pub system_element: Option<Win32_PnPEntity>,
}

impl Win32_PnPDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            same_element: None,
            system_element: None,
        }
    }


    /// Sets the value of SameElement
    pub fn set_same_element(&mut self, value: CIM_LogicalDevice) {
        self.same_element = Some(value);
    }

    /// Gets the value of SameElement
    pub fn get_same_element(&self) -> Option<&CIM_LogicalDevice> {
        self.same_element.as_ref()
    }

    /// Sets the value of SystemElement
    pub fn set_system_element(&mut self, value: Win32_PnPEntity) {
        self.system_element = Some(value);
    }

    /// Gets the value of SystemElement
    pub fn get_system_element(&self) -> Option<&Win32_PnPEntity> {
        self.system_element.as_ref()
    }
}

