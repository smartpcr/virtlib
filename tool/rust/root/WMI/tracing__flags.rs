// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Tracing_Flags
//////////////////////////////////////////////

/// Tracing_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Tracing_Flags {
    /// _CORE_
    #[serde(rename = "_CORE_")]
    CORE = 1,
    /// _ESS_
    #[serde(rename = "_ESS_")]
    ESS = 2,
    /// _PROVIDERS_
    #[serde(rename = "_PROVIDERS_")]
    PROVIDERS = 3,
    /// _DO_NOT_USE_
    #[serde(rename = "_DO_NOT_USE_")]
    DONOTUSE = 4,
}

impl Default for Tracing_Flags {
    fn default() -> Self {
        Self::CORE
    }
}

