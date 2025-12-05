// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSSystemInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSSystemInfo {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// The version number of this WMI Provider
    #[serde(rename = "ProviderVersion")]
    pub provider_version: Option<u32>,

/// The Remote Desktop Users group, in SDDL format
    #[serde(rename = "RDUGroup")]
    pub rdugroup: Option<String>,
}

impl Win32_TSSystemInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            provider_version: None,
            rdugroup: None,
        }
    }


    /// Sets the value of ProviderVersion
    pub fn set_provider_version(&mut self, value: u32) {
        self.provider_version = Some(value);
    }

    /// Gets the value of ProviderVersion
    pub fn get_provider_version(&self) -> Option<&u32> {
        self.provider_version.as_ref()
    }

    /// Sets the value of RDUGroup
    pub fn set_rdugroup(&mut self, value: String) {
        self.rdugroup = Some(value);
    }

    /// Gets the value of RDUGroup
    pub fn get_rdugroup(&self) -> Option<&String> {
        self.rdugroup.as_ref()
    }
}

