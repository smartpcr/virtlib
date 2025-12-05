// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source IHVServiceFrameworkCtlGuid_Flags
//////////////////////////////////////////////

/// IHVServiceFrameworkCtlGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum IHVServiceFrameworkCtlGuid_Flags {
    /// DOT11_ISF
    #[serde(rename = "DOT11_ISF")]
    DOT11ISF = 1,
}

impl Default for IHVServiceFrameworkCtlGuid_Flags {
    fn default() -> Self {
        Self::DOT11ISF
    }
}

