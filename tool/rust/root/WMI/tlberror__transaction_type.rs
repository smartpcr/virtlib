// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TLBError_TransactionType
//////////////////////////////////////////////

/// TLBError_TransactionType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TLBError_TransactionType {
    /// Instruction_Cache
    #[serde(rename = "Instruction_Cache")]
    InstructionCache = 0,
    /// Data_Cache
    #[serde(rename = "Data_Cache")]
    DataCache = 1,
    /// Generic
    #[serde(rename = "Generic")]
    Generic = 2,
}

impl Default for TLBError_TransactionType {
    fn default() -> Self {
        Self::InstructionCache
    }
}

