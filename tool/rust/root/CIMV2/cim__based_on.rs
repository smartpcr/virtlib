// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_BasedOn struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_BasedOn {
    #[serde(flatten)]
    pub base: CIM_Dependency,

/// 
    #[serde(rename = "EndingAddress")]
    pub ending_address: Option<u64>,

/// 
    #[serde(rename = "StartingAddress")]
    pub starting_address: Option<u64>,
}

impl CIM_BasedOn {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Dependency::new(),
            ending_address: None,
            starting_address: None,
        }
    }


    /// Sets the value of EndingAddress
    pub fn set_ending_address(&mut self, value: u64) {
        self.ending_address = Some(value);
    }

    /// Gets the value of EndingAddress
    pub fn get_ending_address(&self) -> Option<&u64> {
        self.ending_address.as_ref()
    }

    /// Sets the value of StartingAddress
    pub fn set_starting_address(&mut self, value: u64) {
        self.starting_address = Some(value);
    }

    /// Gets the value of StartingAddress
    pub fn get_starting_address(&self) -> Option<&u64> {
        self.starting_address.as_ref()
    }
}

