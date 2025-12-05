// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source IPMIPrv_Flags
//////////////////////////////////////////////

/// IPMIPrv_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum IPMIPrv_Flags {
    /// Function
    #[serde(rename = "Function")]
    Function = 1,
    /// FunctionParameter
    #[serde(rename = "FunctionParameter")]
    FunctionParameter = 2,
    /// FunctionDetail
    #[serde(rename = "FunctionDetail")]
    FunctionDetail = 3,
}

impl Default for IPMIPrv_Flags {
    fn default() -> Self {
        Self::Function
    }
}

