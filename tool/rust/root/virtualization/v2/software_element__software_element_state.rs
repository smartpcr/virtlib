// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SoftwareElement_SoftwareElementState
//////////////////////////////////////////////

/// SoftwareElement_SoftwareElementState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SoftwareElement_SoftwareElementState {
    /// Deployable
    #[serde(rename = "Deployable")]
    Deployable = 0,
    /// Installable
    #[serde(rename = "Installable")]
    Installable = 1,
    /// Executable
    #[serde(rename = "Executable")]
    Executable = 2,
    /// Running
    #[serde(rename = "Running")]
    Running = 3,
}

impl Default for SoftwareElement_SoftwareElementState {
    fn default() -> Self {
        Self::Deployable
    }
}

