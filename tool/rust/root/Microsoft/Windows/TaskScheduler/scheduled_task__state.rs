// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ScheduledTask_State
//////////////////////////////////////////////

/// ScheduledTask_State enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ScheduledTask_State {
    /// _17
    #[serde(rename = "_17")]
    V17 = 0,
    /// _18
    #[serde(rename = "_18")]
    V18 = 1,
    /// _19
    #[serde(rename = "_19")]
    V19 = 2,
    /// _20
    #[serde(rename = "_20")]
    V20 = 3,
    /// _21
    #[serde(rename = "_21")]
    V21 = 4,
}

impl Default for ScheduledTask_State {
    fn default() -> Self {
        Self::V17
    }
}

