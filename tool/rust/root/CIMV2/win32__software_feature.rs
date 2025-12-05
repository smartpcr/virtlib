// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SoftwareFeature struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SoftwareFeature {
    #[serde(flatten)]
    pub base: CIM_SoftwareFeature,

/// 
    #[serde(rename = "Accesses")]
    pub accesses: Option<u16>,

/// 
    #[serde(rename = "Attributes")]
    pub attributes: Option<u16>,

/// 
    #[serde(rename = "InstallState")]
    pub install_state: Option<i16>,

/// 
    #[serde(rename = "LastUse")]
    pub last_use: Option<String>,
}

impl Win32_SoftwareFeature {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SoftwareFeature::new(),
            accesses: None,
            attributes: None,
            install_state: None,
            last_use: None,
        }
    }


    /// Sets the value of Accesses
    pub fn set_accesses(&mut self, value: u16) {
        self.accesses = Some(value);
    }

    /// Gets the value of Accesses
    pub fn get_accesses(&self) -> Option<&u16> {
        self.accesses.as_ref()
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

    /// Sets the value of LastUse
    pub fn set_last_use(&mut self, value: String) {
        self.last_use = Some(value);
    }

    /// Gets the value of LastUse
    pub fn get_last_use(&self) -> Option<&String> {
        self.last_use.as_ref()
    }

/// 

    /// * `reinstall_mode` -  (u16)

    /// * `return_value` -  (u32)
    pub fn reinstall(&self, reinstall_mode: u16) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReinstallMode".to_string(), value: reinstall_mode.into() });
        self.invoke_method("Reinstall", &args)

    }


/// 

    /// * `install_state` -  (u16)

    /// * `return_value` -  (u32)
    pub fn configure(&self, install_state: u16) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InstallState".to_string(), value: install_state.into() });
        self.invoke_method("Configure", &args)

    }

}

