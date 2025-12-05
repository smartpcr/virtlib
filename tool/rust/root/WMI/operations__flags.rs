// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Operations_Flags
//////////////////////////////////////////////

/// Operations_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Operations_Flags {
    /// _WMI_TRACE_CLIENT_OPERATIONS_
    #[serde(rename = "_WMI_TRACE_CLIENT_OPERATIONS_")]
    WMITRACECLIENTOPERATIONS = 1,
}

impl Default for Operations_Flags {
    fn default() -> Self {
        Self::WMITRACECLIENTOPERATIONS
    }
}

