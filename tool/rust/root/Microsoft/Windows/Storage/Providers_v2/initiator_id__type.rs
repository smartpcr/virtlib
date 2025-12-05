// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source InitiatorId_Type
//////////////////////////////////////////////

/// InitiatorId_Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum InitiatorId_Type {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// PortWWN
    #[serde(rename = "PortWWN")]
    PortWWN = 2,
    /// NodeWWN
    #[serde(rename = "NodeWWN")]
    NodeWWN = 3,
    /// Hostname
    #[serde(rename = "Hostname")]
    Hostname = 4,
    /// iSCSI_Name
    #[serde(rename = "iSCSI_Name")]
    ISCSIName = 5,
    /// SwitchWWN
    #[serde(rename = "SwitchWWN")]
    SwitchWWN = 6,
    /// SASAddress
    #[serde(rename = "SASAddress")]
    SASAddress = 7,
}

impl Default for InitiatorId_Type {
    fn default() -> Self {
        Self::Other
    }
}

