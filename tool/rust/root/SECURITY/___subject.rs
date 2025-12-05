// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SECURITY
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __Subject struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __Subject {
    #[serde(flatten)]
    pub base: __SecurityRelatedClass,

/// 
    #[serde(rename = "Authority")]
    pub authority: Option<String>,

/// 
    #[serde(rename = "EditSecurity")]
    pub edit_security: Option<bool>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "ExecuteMethods")]
    pub execute_methods: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Permissions")]
    pub permissions: Option<i32>,
}

impl __Subject {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SecurityRelatedClass::new(),
            authority: None,
            edit_security: None,
            enabled: None,
            execute_methods: None,
            name: None,
            permissions: None,
        }
    }


    /// Sets the value of Authority
    pub fn set_authority(&mut self, value: String) {
        self.authority = Some(value);
    }

    /// Gets the value of Authority
    pub fn get_authority(&self) -> Option<&String> {
        self.authority.as_ref()
    }

    /// Sets the value of EditSecurity
    pub fn set_edit_security(&mut self, value: bool) {
        self.edit_security = Some(value);
    }

    /// Gets the value of EditSecurity
    pub fn get_edit_security(&self) -> Option<&bool> {
        self.edit_security.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of ExecuteMethods
    pub fn set_execute_methods(&mut self, value: bool) {
        self.execute_methods = Some(value);
    }

    /// Gets the value of ExecuteMethods
    pub fn get_execute_methods(&self) -> Option<&bool> {
        self.execute_methods.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Permissions
    pub fn set_permissions(&mut self, value: i32) {
        self.permissions = Some(value);
    }

    /// Gets the value of Permissions
    pub fn get_permissions(&self) -> Option<&i32> {
        self.permissions.as_ref()
    }
}

