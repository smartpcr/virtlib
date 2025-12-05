// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageSubSystem_MaskingValidInitiatorIdTypes
//////////////////////////////////////////////

/// StorageSubSystem_MaskingValidInitiatorIdTypes enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageSubSystem_MaskingValidInitiatorIdTypes {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Port_WWN
    #[serde(rename = "Port_WWN")]
    PortWWN = 2,
    /// Node_WWN
    #[serde(rename = "Node_WWN")]
    NodeWWN = 3,
    /// Host_Name
    #[serde(rename = "Host_Name")]
    HostName = 4,
    /// iSCSI_Name
    #[serde(rename = "iSCSI_Name")]
    ISCSIName = 5,
    /// Switch_WWN
    #[serde(rename = "Switch_WWN")]
    SwitchWWN = 6,
    /// SAS_Address
    #[serde(rename = "SAS_Address")]
    SASAddress = 7,
}

impl Default for StorageSubSystem_MaskingValidInitiatorIdTypes {
    fn default() -> Self {
        Self::Other
    }
}

