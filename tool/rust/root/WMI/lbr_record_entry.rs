// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// LbrRecordEntry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LbrRecordEntry {

/// 
    #[serde(rename = "FromAddress")]
    pub from_address: Option<u64>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u64>,

/// 
    #[serde(rename = "ToAddress")]
    pub to_address: Option<u64>,
}

impl LbrRecordEntry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            from_address: None,
            reserved: None,
            to_address: None,
        }
    }


    /// Sets the value of FromAddress
    pub fn set_from_address(&mut self, value: u64) {
        self.from_address = Some(value);
    }

    /// Gets the value of FromAddress
    pub fn get_from_address(&self) -> Option<&u64> {
        self.from_address.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u64) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u64> {
        self.reserved.as_ref()
    }

    /// Sets the value of ToAddress
    pub fn set_to_address(&mut self, value: u64) {
        self.to_address = Some(value);
    }

    /// Gets the value of ToAddress
    pub fn get_to_address(&self) -> Option<&u64> {
        self.to_address.as_ref()
    }
}

