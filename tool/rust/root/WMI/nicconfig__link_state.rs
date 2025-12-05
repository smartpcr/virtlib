// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source NICConfig_LinkState
//////////////////////////////////////////////

/// NICConfig_LinkState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum NICConfig_LinkState {
    /// Media_Disconnected
    #[serde(rename = "Media_Disconnected")]
    MediaDisconnected = 0,
    /// Media_Connected
    #[serde(rename = "Media_Connected")]
    MediaConnected = 1,
}

impl Default for NICConfig_LinkState {
    fn default() -> Self {
        Self::MediaDisconnected
    }
}

