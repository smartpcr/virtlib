// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SoftwareElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SoftwareElement {
    #[serde(flatten)]
    pub base: CIM_SoftwareElement,

/// 
    #[serde(rename = "Attributes")]
    pub attributes: Option<u16>,

/// 
    #[serde(rename = "InstallState")]
    pub install_state: Option<i16>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,
}

impl Win32_SoftwareElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SoftwareElement::new(),
            attributes: None,
            install_state: None,
            path: None,
        }
    }


    /// Sets the value of Attributes
    pub fn set_attributes(&mut self, value: u16) {
        self.attributes = Some(value);
    }

    /// Gets the value of Attributes
    pub fn get_attributes(&self) -> Option<&u16> {
        self.attributes.as_ref()
    }

    /// Sets the value of InstallState
    pub fn set_install_state(&mut self, value: i16) {
        self.install_state = Some(value);
    }

    /// Gets the value of InstallState
    pub fn get_install_state(&self) -> Option<&i16> {
        self.install_state.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }
}

