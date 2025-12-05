// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TargetClass_ProtocolType
//////////////////////////////////////////////

/// TargetClass_ProtocolType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TargetClass_ProtocolType {
    /// ISCSI_TCP_PROTOCOL_TYPE
    #[serde(rename = "ISCSI_TCP_PROTOCOL_TYPE")]
    ISCSITCPPROTOCOLTYPE = 0,
}

impl Default for TargetClass_ProtocolType {
    fn default() -> Self {
        Self::ISCSITCPPROTOCOLTYPE
    }
}

