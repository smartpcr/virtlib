// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_NetworkShortAddress struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_NetworkShortAddress {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Address")]
    pub address: Vec<u8>,
}

impl MSNdis_NetworkShortAddress {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            address: Vec::new(),
        }
    }


    /// Sets the value of Address
    pub fn set_address(&mut self, value: Vec<u8>) {
        self.address = value;
    }

    /// Gets the value of Address
    pub fn get_address(&self) -> &Vec<u8> {
        &self.address
    }
}

