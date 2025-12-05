// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TaskSettings_MultipleInstances
//////////////////////////////////////////////

/// TaskSettings_MultipleInstances enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TaskSettings_MultipleInstances {
    /// _13
    #[serde(rename = "_13")]
    V13 = 0,
    /// _14
    #[serde(rename = "_14")]
    V14 = 1,
    /// _15
    #[serde(rename = "_15")]
    V15 = 2,
    /// _16
    #[serde(rename = "_16")]
    V16 = 3,
}

impl Default for TaskSettings_MultipleInstances {
    fn default() -> Self {
        Self::V13
    }
}

