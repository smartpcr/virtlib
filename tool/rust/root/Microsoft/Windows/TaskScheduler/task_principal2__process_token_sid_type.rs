// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TaskPrincipal2_ProcessTokenSidType
//////////////////////////////////////////////

/// TaskPrincipal2_ProcessTokenSidType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TaskPrincipal2_ProcessTokenSidType {
    /// _109
    #[serde(rename = "_109")]
    V109 = 0,
    /// _110
    #[serde(rename = "_110")]
    V110 = 1,
    /// _111
    #[serde(rename = "_111")]
    V111 = 2,
}

impl Default for TaskPrincipal2_ProcessTokenSidType {
    fn default() -> Self {
        Self::V109
    }
}

