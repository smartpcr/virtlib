// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VersionCompatibilityCheck struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VersionCompatibilityCheck {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "AllowDownVersion")]
    pub allow_down_version: Option<bool>,

/// 
    #[serde(rename = "AllowMultipleVersions")]
    pub allow_multiple_versions: Option<bool>,

/// 
    #[serde(rename = "Reinstall")]
    pub reinstall: Option<bool>,
}

impl CIM_VersionCompatibilityCheck {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            allow_down_version: None,
            allow_multiple_versions: None,
            reinstall: None,
        }
    }


    /// Sets the value of AllowDownVersion
    pub fn set_allow_down_version(&mut self, value: bool) {
        self.allow_down_version = Some(value);
    }

    /// Gets the value of AllowDownVersion
    pub fn get_allow_down_version(&self) -> Option<&bool> {
        self.allow_down_version.as_ref()
    }

    /// Sets the value of AllowMultipleVersions
    pub fn set_allow_multiple_versions(&mut self, value: bool) {
        self.allow_multiple_versions = Some(value);
    }

    /// Gets the value of AllowMultipleVersions
    pub fn get_allow_multiple_versions(&self) -> Option<&bool> {
        self.allow_multiple_versions.as_ref()
    }

    /// Sets the value of Reinstall
    pub fn set_reinstall(&mut self, value: bool) {
        self.reinstall = Some(value);
    }

    /// Gets the value of Reinstall
    pub fn get_reinstall(&self) -> Option<&bool> {
        self.reinstall.as_ref()
    }
}

