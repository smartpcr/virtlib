// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source AlertIndication_AlertingElementFormat
//////////////////////////////////////////////

/// AlertIndication_AlertingElementFormat enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum AlertIndication_AlertingElementFormat {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// CIMObjectPath
    #[serde(rename = "CIMObjectPath")]
    CIMObjectPath = 2,
}

impl Default for AlertIndication_AlertingElementFormat {
    fn default() -> Self {
        Self::Unknown
    }
}

