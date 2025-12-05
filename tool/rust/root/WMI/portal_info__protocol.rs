// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PortalInfo_Protocol
//////////////////////////////////////////////

/// PortalInfo_Protocol enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PortalInfo_Protocol {
    /// TCP
    #[serde(rename = "TCP")]
    TCP = 6,
}

impl Default for PortalInfo_Protocol {
    fn default() -> Self {
        Self::TCP
    }
}

