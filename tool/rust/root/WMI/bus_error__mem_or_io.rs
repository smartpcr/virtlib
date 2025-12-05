// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BusError_MemOrIo
//////////////////////////////////////////////

/// BusError_MemOrIo enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BusError_MemOrIo {
    /// Memory_Access
    #[serde(rename = "Memory_Access")]
    MemoryAccess = 0,
    /// Reserved
    #[serde(rename = "Reserved")]
    Reserved = 1,
    /// I_O
    #[serde(rename = "I_O")]
    IO = 2,
    /// Other_transaction
    #[serde(rename = "Other_transaction")]
    OtherTransaction = 3,
}

impl Default for BusError_MemOrIo {
    fn default() -> Self {
        Self::MemoryAccess
    }
}

