// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.SecurityClient
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ProtectionTechnologyStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProtectionTechnologyStatus {
    #[serde(flatten)]
    pub base: SerializableToXml,

/// Is protection technology enabled
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// Protection technology name
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// Protection technology version (major, minor, build, revision)
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl ProtectionTechnologyStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SerializableToXml::new(),
            enabled: None,
            name: None,
            version: None,
        }
    }


    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }
}

