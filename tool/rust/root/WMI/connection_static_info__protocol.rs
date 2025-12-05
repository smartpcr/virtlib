// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ConnectionStaticInfo_Protocol
//////////////////////////////////////////////

/// ConnectionStaticInfo_Protocol enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ConnectionStaticInfo_Protocol {
    /// TCP
    #[serde(rename = "TCP")]
    TCP = 6,
}

impl Default for ConnectionStaticInfo_Protocol {
    fn default() -> Self {
        Self::TCP
    }
}

