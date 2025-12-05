// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Path_ConnectionStatus
//////////////////////////////////////////////

/// Path_ConnectionStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Path_ConnectionStatus {
    /// Connected
    #[serde(rename = "Connected")]
    Connected = 1,
    /// Disconnected
    #[serde(rename = "Disconnected")]
    Disconnected = 2,
    /// Reconnecting
    #[serde(rename = "Reconnecting")]
    Reconnecting = 3,
}

impl Default for Path_ConnectionStatus {
    fn default() -> Self {
        Self::Connected
    }
}

