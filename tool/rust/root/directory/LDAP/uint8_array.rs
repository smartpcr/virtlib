// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.directory.LDAP
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Uint8Array struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Uint8Array {

/// 
    #[serde(rename = "value")]
    pub value: Vec<u8>,
}

impl Uint8Array {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            value: Vec::new(),
        }
    }


    /// Sets the value of value
    pub fn set_value(&mut self, value: Vec<u8>) {
        self.value = value;
    }

    /// Gets the value of value
    pub fn get_value(&self) -> &Vec<u8> {
        &self.value
    }
}

