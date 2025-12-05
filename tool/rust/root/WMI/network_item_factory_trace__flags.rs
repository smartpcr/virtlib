// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source NetworkItemFactoryTrace_Flags
//////////////////////////////////////////////

/// NetworkItemFactoryTrace_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum NetworkItemFactoryTrace_Flags {
    /// tagDefault
    #[serde(rename = "tagDefault")]
    TagDefault = 1,
    /// tagNetworkItemFactory
    #[serde(rename = "tagNetworkItemFactory")]
    TagNetworkItemFactory = 2,
}

impl Default for NetworkItemFactoryTrace_Flags {
    fn default() -> Self {
        Self::TagDefault
    }
}

