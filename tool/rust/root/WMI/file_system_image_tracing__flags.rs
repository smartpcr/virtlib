// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FileSystemImageTracing_Flags
//////////////////////////////////////////////

/// FileSystemImageTracing_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FileSystemImageTracing_Flags {
    /// Constructor
    #[serde(rename = "Constructor")]
    Constructor = 1,
    /// Destructor
    #[serde(rename = "Destructor")]
    Destructor = 2,
    /// General
    #[serde(rename = "General")]
    General = 3,
    /// FunctionEntryExit
    #[serde(rename = "FunctionEntryExit")]
    FunctionEntryExit = 4,
}

impl Default for FileSystemImageTracing_Flags {
    fn default() -> Self {
        Self::Constructor
    }
}

