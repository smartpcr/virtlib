// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSAcpiInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSAcpiInfo {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "BootArchitecture")]
    pub boot_architecture: Option<u32>,

/// 
    #[serde(rename = "Capabilities")]
    pub capabilities: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "PreferredProfile")]
    pub preferred_profile: Option<u32>,
}

impl MSAcpiInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            boot_architecture: None,
            capabilities: None,
            instance_name: None,
            preferred_profile: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of BootArchitecture
    pub fn set_boot_architecture(&mut self, value: u32) {
        self.boot_architecture = Some(value);
    }

    /// Gets the value of BootArchitecture
    pub fn get_boot_architecture(&self) -> Option<&u32> {
        self.boot_architecture.as_ref()
    }

    /// Sets the value of Capabilities
    pub fn set_capabilities(&mut self, value: u32) {
        self.capabilities = Some(value);
    }

    /// Gets the value of Capabilities
    pub fn get_capabilities(&self) -> Option<&u32> {
        self.capabilities.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of PreferredProfile
    pub fn set_preferred_profile(&mut self, value: u32) {
        self.preferred_profile = Some(value);
    }

    /// Gets the value of PreferredProfile
    pub fn get_preferred_profile(&self) -> Option<&u32> {
        self.preferred_profile.as_ref()
    }
}

