// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISCSI_LUNList struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISCSI_LUNList {

/// 
    #[serde(rename = "OSLUN")]
    pub oslun: Option<u32>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u32>,

/// 
    #[serde(rename = "TargetLUN")]
    pub target_lun: Option<u64>,
}

impl ISCSI_LUNList {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            oslun: None,
            reserved: None,
            target_lun: None,
        }
    }


    /// Sets the value of OSLUN
    pub fn set_oslun(&mut self, value: u32) {
        self.oslun = Some(value);
    }

    /// Gets the value of OSLUN
    pub fn get_oslun(&self) -> Option<&u32> {
        self.oslun.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u32) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u32> {
        self.reserved.as_ref()
    }

    /// Sets the value of TargetLUN
    pub fn set_target_lun(&mut self, value: u64) {
        self.target_lun = Some(value);
    }

    /// Gets the value of TargetLUN
    pub fn get_target_lun(&self) -> Option<&u64> {
        self.target_lun.as_ref()
    }
}

