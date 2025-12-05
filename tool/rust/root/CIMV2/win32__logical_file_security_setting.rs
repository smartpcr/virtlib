// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_LogicalFileSecuritySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_LogicalFileSecuritySetting {
    #[serde(flatten)]
    pub base: Win32_SecuritySetting,

/// 
    #[serde(rename = "OwnerPermissions")]
    pub owner_permissions: Option<bool>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,
}

impl Win32_LogicalFileSecuritySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_SecuritySetting::new(),
            owner_permissions: None,
            path: None,
        }
    }


    /// Sets the value of OwnerPermissions
    pub fn set_owner_permissions(&mut self, value: bool) {
        self.owner_permissions = Some(value);
    }

    /// Gets the value of OwnerPermissions
    pub fn get_owner_permissions(&self) -> Option<&bool> {
        self.owner_permissions.as_ref()
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

