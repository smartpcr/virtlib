// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_LUNList struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_LUNList {

/// 
    #[serde(rename = "OSLunNumber")]
    pub oslun_number: Option<u32>,

/// 
    #[serde(rename = "TargetLun")]
    pub target_lun: Option<u64>,
}

impl MSiSCSIInitiator_LUNList {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            oslun_number: None,
            target_lun: None,
        }
    }


    /// Sets the value of OSLunNumber
    pub fn set_oslun_number(&mut self, value: u32) {
        self.oslun_number = Some(value);
    }

    /// Gets the value of OSLunNumber
    pub fn get_oslun_number(&self) -> Option<&u32> {
        self.oslun_number.as_ref()
    }

    /// Sets the value of TargetLun
    pub fn set_target_lun(&mut self, value: u64) {
        self.target_lun = Some(value);
    }

    /// Gets the value of TargetLun
    pub fn get_target_lun(&self) -> Option<&u64> {
        self.target_lun.as_ref()
    }
}

