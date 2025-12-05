// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbWitnessClient_State
//////////////////////////////////////////////

/// SmbWitnessClient_State enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbWitnessClient_State {
    /// _1
    #[serde(rename = "_1")]
    V1 = 0,
    /// _2
    #[serde(rename = "_2")]
    V2 = 1,
    /// _3
    #[serde(rename = "_3")]
    V3 = 2,
    /// _4
    #[serde(rename = "_4")]
    V4 = 3,
    /// _5
    #[serde(rename = "_5")]
    V5 = 4,
    /// _6
    #[serde(rename = "_6")]
    V6 = 5,
}

impl Default for SmbWitnessClient_State {
    fn default() -> Self {
        Self::V1
    }
}

