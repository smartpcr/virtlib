// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbShareChangeEvent_EventType
//////////////////////////////////////////////

/// SmbShareChangeEvent_EventType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbShareChangeEvent_EventType {
    /// _47
    #[serde(rename = "_47")]
    V47 = 0,
    /// _48
    #[serde(rename = "_48")]
    V48 = 1,
    /// _49
    #[serde(rename = "_49")]
    V49 = 2,
}

impl Default for SmbShareChangeEvent_EventType {
    fn default() -> Self {
        Self::V47
    }
}

