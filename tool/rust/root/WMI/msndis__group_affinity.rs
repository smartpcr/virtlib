// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_GroupAffinity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_GroupAffinity {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Group")]
    pub group: Option<u16>,

/// 
    #[serde(rename = "Mask")]
    pub mask: Option<u64>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Vec<u16>,
}

impl MSNdis_GroupAffinity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            group: None,
            mask: None,
            reserved: Vec::new(),
        }
    }


    /// Sets the value of Group
    pub fn set_group(&mut self, value: u16) {
        self.group = Some(value);
    }

    /// Gets the value of Group
    pub fn get_group(&self) -> Option<&u16> {
        self.group.as_ref()
    }

    /// Sets the value of Mask
    pub fn set_mask(&mut self, value: u64) {
        self.mask = Some(value);
    }

    /// Gets the value of Mask
    pub fn get_mask(&self) -> Option<&u64> {
        self.mask.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: Vec<u16>) {
        self.reserved = value;
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> &Vec<u16> {
        &self.reserved
    }
}

