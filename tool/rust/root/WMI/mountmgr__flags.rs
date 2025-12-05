// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Mountmgr_Flags
//////////////////////////////////////////////

/// Mountmgr_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Mountmgr_Flags {
    /// TracingGeneral
    #[serde(rename = "TracingGeneral")]
    TracingGeneral = 1,
    /// TracingIoctl
    #[serde(rename = "TracingIoctl")]
    TracingIoctl = 2,
    /// TracingMemory
    #[serde(rename = "TracingMemory")]
    TracingMemory = 3,
    /// TracingCache
    #[serde(rename = "TracingCache")]
    TracingCache = 4,
}

impl Default for Mountmgr_Flags {
    fn default() -> Self {
        Self::TracingGeneral
    }
}

