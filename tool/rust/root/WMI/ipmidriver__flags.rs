// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source IPMIDRIVER_Flags
//////////////////////////////////////////////

/// IPMIDRIVER_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum IPMIDRIVER_Flags {
    /// TRACELEVELONE
    #[serde(rename = "TRACELEVELONE")]
    TRACELEVELONE = 1,
    /// TRACELEVELTWO
    #[serde(rename = "TRACELEVELTWO")]
    TRACELEVELTWO = 2,
}

impl Default for IPMIDRIVER_Flags {
    fn default() -> Self {
        Self::TRACELEVELONE
    }
}

