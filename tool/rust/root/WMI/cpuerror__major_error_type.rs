// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source CPUError_MajorErrorType
//////////////////////////////////////////////

/// CPUError_MajorErrorType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum CPUError_MajorErrorType {
    /// Cache
    #[serde(rename = "Cache")]
    Cache = 0,
    /// TLB
    #[serde(rename = "TLB")]
    TLB = 1,
    /// Bus
    #[serde(rename = "Bus")]
    Bus = 2,
    /// Register_File
    #[serde(rename = "Register_File")]
    RegisterFile = 3,
    /// Microarchitecture
    #[serde(rename = "Microarchitecture")]
    Microarchitecture = 4,
}

impl Default for CPUError_MajorErrorType {
    fn default() -> Self {
        Self::Cache
    }
}

