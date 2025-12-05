// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.directory.LDAP
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DN_With_String struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DN_With_String {

/// 
    #[serde(rename = "dnString")]
    pub dn_string: Option<String>,

/// 
    #[serde(rename = "value")]
    pub value: Option<String>,
}

impl DN_With_String {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dn_string: None,
            value: None,
        }
    }


    /// Sets the value of dnString
    pub fn set_dn_string(&mut self, value: String) {
        self.dn_string = Some(value);
    }

    /// Gets the value of dnString
    pub fn get_dn_string(&self) -> Option<&String> {
        self.dn_string.as_ref()
    }

    /// Sets the value of value
    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    /// Gets the value of value
    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }
}

