// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetLbfoTeamNic struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetLbfoTeamNic {
    #[serde(flatten)]
    pub base: MSFT_NetImPlatAdapter,

/// 401
    #[serde(rename = "Default")]
    pub default: Option<bool>,

/// 400
    #[serde(rename = "Primary")]
    pub primary: Option<bool>,

/// 399
    #[serde(rename = "VlanID")]
    pub vlan_id: Option<u32>,
}

impl MSFT_NetLbfoTeamNic {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetImPlatAdapter::new(),
            default: None,
            primary: None,
            vlan_id: None,
        }
    }


    /// Sets the value of Default
    pub fn set_default(&mut self, value: bool) {
        self.default = Some(value);
    }

    /// Gets the value of Default
    pub fn get_default(&self) -> Option<&bool> {
        self.default.as_ref()
    }

    /// Sets the value of Primary
    pub fn set_primary(&mut self, value: bool) {
        self.primary = Some(value);
    }

    /// Gets the value of Primary
    pub fn get_primary(&self) -> Option<&bool> {
        self.primary.as_ref()
    }

    /// Sets the value of VlanID
    pub fn set_vlan_id(&mut self, value: u32) {
        self.vlan_id = Some(value);
    }

    /// Gets the value of VlanID
    pub fn get_vlan_id(&self) -> Option<&u32> {
        self.vlan_id.as_ref()
    }
}

