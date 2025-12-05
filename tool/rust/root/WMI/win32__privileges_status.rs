// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PrivilegesStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PrivilegesStatus {
    #[serde(flatten)]
    pub base: __ExtendedStatus,

/// 
    #[serde(rename = "PrivilegesNotHeld")]
    pub privileges_not_held: Vec<String>,

/// 
    #[serde(rename = "PrivilegesRequired")]
    pub privileges_required: Vec<String>,
}

impl Win32_PrivilegesStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtendedStatus::new(),
            privileges_not_held: Vec::new(),
            privileges_required: Vec::new(),
        }
    }


    /// Sets the value of PrivilegesNotHeld
    pub fn set_privileges_not_held(&mut self, value: Vec<String>) {
        self.privileges_not_held = value;
    }

    /// Gets the value of PrivilegesNotHeld
    pub fn get_privileges_not_held(&self) -> &Vec<String> {
        &self.privileges_not_held
    }

    /// Sets the value of PrivilegesRequired
    pub fn set_privileges_required(&mut self, value: Vec<String>) {
        self.privileges_required = value;
    }

    /// Gets the value of PrivilegesRequired
    pub fn get_privileges_required(&self) -> &Vec<String> {
        &self.privileges_required
    }
}

