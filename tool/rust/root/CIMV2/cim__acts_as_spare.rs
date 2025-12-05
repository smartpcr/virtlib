// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ActsAsSpare struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ActsAsSpare {

/// 
    #[serde(rename = "Group")]
    pub group: Option<CIM_SpareGroup>,

/// 
    #[serde(rename = "HotStandby")]
    pub hot_standby: Option<bool>,

/// 
    #[serde(rename = "Spare")]
    pub spare: Option<CIM_ManagedSystemElement>,
}

impl CIM_ActsAsSpare {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            group: None,
            hot_standby: None,
            spare: None,
        }
    }


    /// Sets the value of Group
    pub fn set_group(&mut self, value: CIM_SpareGroup) {
        self.group = Some(value);
    }

    /// Gets the value of Group
    pub fn get_group(&self) -> Option<&CIM_SpareGroup> {
        self.group.as_ref()
    }

    /// Sets the value of HotStandby
    pub fn set_hot_standby(&mut self, value: bool) {
        self.hot_standby = Some(value);
    }

    /// Gets the value of HotStandby
    pub fn get_hot_standby(&self) -> Option<&bool> {
        self.hot_standby.as_ref()
    }

    /// Sets the value of Spare
    pub fn set_spare(&mut self, value: CIM_ManagedSystemElement) {
        self.spare = Some(value);
    }

    /// Gets the value of Spare
    pub fn get_spare(&self) -> Option<&CIM_ManagedSystemElement> {
        self.spare.as_ref()
    }
}

