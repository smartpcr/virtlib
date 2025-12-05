// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BusError_RequestType
//////////////////////////////////////////////

/// BusError_RequestType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BusError_RequestType {
    /// Generic_Error
    #[serde(rename = "Generic_Error")]
    GenericError = 0,
    /// Generic_Read
    #[serde(rename = "Generic_Read")]
    GenericRead = 1,
    /// Generic_Write
    #[serde(rename = "Generic_Write")]
    GenericWrite = 2,
    /// Data_Read
    #[serde(rename = "Data_Read")]
    DataRead = 3,
    /// Data_Write
    #[serde(rename = "Data_Write")]
    DataWrite = 4,
    /// Instruction_Fetch
    #[serde(rename = "Instruction_Fetch")]
    InstructionFetch = 5,
    /// Prefetch
    #[serde(rename = "Prefetch")]
    Prefetch = 6,
    /// Injection
    #[serde(rename = "Injection")]
    Injection = 7,
    /// Snoop
    #[serde(rename = "Snoop")]
    Snoop = 8,
}

impl Default for BusError_RequestType {
    fn default() -> Self {
        Self::GenericError
    }
}

