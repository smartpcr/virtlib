// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.directory.LDAP
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DN_Class struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DN_Class {

/// 
    #[serde(rename = "DN")]
    pub dn: Option<String>,
}

impl DN_Class {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dn: None,
        }
    }


    /// Sets the value of DN
    pub fn set_dn(&mut self, value: String) {
        self.dn = Some(value);
    }

    /// Gets the value of DN
    pub fn get_dn(&self) -> Option<&String> {
        self.dn.as_ref()
    }
}

