// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SerialController_Capabilities
//////////////////////////////////////////////

/// SerialController_Capabilities enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SerialController_Capabilities {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
    /// XT_AT_Compatible
    #[serde(rename = "XT_AT_Compatible")]
    XTATCompatible = 3,
    /// _16450_Compatible
    #[serde(rename = "_16450_Compatible")]
    V16450Compatible = 4,
    /// _16550_Compatible
    #[serde(rename = "_16550_Compatible")]
    V16550Compatible = 5,
    /// _16550A_Compatible
    #[serde(rename = "_16550A_Compatible")]
    V16550ACompatible = 6,
    /// _8251_Compatible
    #[serde(rename = "_8251_Compatible")]
    V8251Compatible = 160,
    /// _8251FIFO_Compatible
    #[serde(rename = "_8251FIFO_Compatible")]
    V8251FIFOCompatible = 161,
}

impl Default for SerialController_Capabilities {
    fn default() -> Self {
        Self::Other
    }
}

