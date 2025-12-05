// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __thisNAMESPACE struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __thisNAMESPACE {
    #[serde(flatten)]
    pub base: __SystemClass,

/// 
    #[serde(rename = "SECURITY_DESCRIPTOR")]
    pub security__descriptor: Vec<u8>,
}

impl __thisNAMESPACE {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SystemClass::new(),
            security__descriptor: Vec::new(),
        }
    }


    /// Sets the value of SECURITY_DESCRIPTOR
    pub fn set_security__descriptor(&mut self, value: Vec<u8>) {
        self.security__descriptor = value;
    }

    /// Gets the value of SECURITY_DESCRIPTOR
    pub fn get_security__descriptor(&self) -> &Vec<u8> {
        &self.security__descriptor
    }
}

