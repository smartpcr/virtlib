// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WNFNameSubRundown struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WNFNameSubRundown {
    #[serde(flatten)]
    pub base: WNFTrace,

/// 
    #[serde(rename = "NameSub")]
    pub name_sub: Option<u32>,

/// 
    #[serde(rename = "StateName")]
    pub state_name: Option<u64>,
}

impl WNFNameSubRundown {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WNFTrace::new(),
            name_sub: None,
            state_name: None,
        }
    }


    /// Sets the value of NameSub
    pub fn set_name_sub(&mut self, value: u32) {
        self.name_sub = Some(value);
    }

    /// Gets the value of NameSub
    pub fn get_name_sub(&self) -> Option<&u32> {
        self.name_sub.as_ref()
    }

    /// Sets the value of StateName
    pub fn set_state_name(&mut self, value: u64) {
        self.state_name = Some(value);
    }

    /// Gets the value of StateName
    pub fn get_state_name(&self) -> Option<&u64> {
        self.state_name.as_ref()
    }
}

