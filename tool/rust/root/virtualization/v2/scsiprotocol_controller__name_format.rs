// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SCSIProtocolController_NameFormat
//////////////////////////////////////////////

/// SCSIProtocolController_NameFormat enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SCSIProtocolController_NameFormat {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// FC_Port_WWN
    #[serde(rename = "FC_Port_WWN")]
    FCPortWWN = 2,
    /// iSCSI_Name
    #[serde(rename = "iSCSI_Name")]
    ISCSIName = 3,
}

impl Default for SCSIProtocolController_NameFormat {
    fn default() -> Self {
        Self::Unknown
    }
}

