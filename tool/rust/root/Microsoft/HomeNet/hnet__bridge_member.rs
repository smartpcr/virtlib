// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.HomeNet
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HNet_BridgeMember struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HNet_BridgeMember {

/// 
    #[serde(rename = "Bridge")]
    pub bridge: Option<HNet_Connection>,

/// 
    #[serde(rename = "Member")]
    pub member: Option<HNet_Connection>,
}

impl HNet_BridgeMember {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bridge: None,
            member: None,
        }
    }


    /// Sets the value of Bridge
    pub fn set_bridge(&mut self, value: HNet_Connection) {
        self.bridge = Some(value);
    }

    /// Gets the value of Bridge
    pub fn get_bridge(&self) -> Option<&HNet_Connection> {
        self.bridge.as_ref()
    }

    /// Sets the value of Member
    pub fn set_member(&mut self, value: HNet_Connection) {
        self.member = Some(value);
    }

    /// Gets the value of Member
    pub fn get_member(&self) -> Option<&HNet_Connection> {
        self.member.as_ref()
    }
}

