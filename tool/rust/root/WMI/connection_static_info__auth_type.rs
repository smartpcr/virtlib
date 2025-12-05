// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ConnectionStaticInfo_AuthType
//////////////////////////////////////////////

/// ConnectionStaticInfo_AuthType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ConnectionStaticInfo_AuthType {
    /// No_Authentication
    #[serde(rename = "No_Authentication")]
    NoAuthentication = 0,
    /// CHAP
    #[serde(rename = "CHAP")]
    CHAP = 1,
    /// Mutual_CHAP
    #[serde(rename = "Mutual_CHAP")]
    MutualCHAP = 2,
}

impl Default for ConnectionStaticInfo_AuthType {
    fn default() -> Self {
        Self::NoAuthentication
    }
}

