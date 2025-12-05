// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TokenPrivileges struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TokenPrivileges {

/// 
    #[serde(rename = "PrivilegeCount")]
    pub privilege_count: Option<u32>,

/// 
    #[serde(rename = "Privileges")]
    pub privileges: Vec<Win32_LUIDandAttributes>,
}

impl Win32_TokenPrivileges {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            privilege_count: None,
            privileges: Vec::new(),
        }
    }


    /// Sets the value of PrivilegeCount
    pub fn set_privilege_count(&mut self, value: u32) {
        self.privilege_count = Some(value);
    }

    /// Gets the value of PrivilegeCount
    pub fn get_privilege_count(&self) -> Option<&u32> {
        self.privilege_count.as_ref()
    }

    /// Sets the value of Privileges
    pub fn set_privileges(&mut self, value: Vec<Win32_LUIDandAttributes>) {
        self.privileges = value;
    }

    /// Gets the value of Privileges
    pub fn get_privileges(&self) -> &Vec<Win32_LUIDandAttributes> {
        &self.privileges
    }
}

