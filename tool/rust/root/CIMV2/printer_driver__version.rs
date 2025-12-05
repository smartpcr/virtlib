// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PrinterDriver_Version
//////////////////////////////////////////////

/// PrinterDriver_Version enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PrinterDriver_Version {
    /// Win9x
    #[serde(rename = "Win9x")]
    Win9x = 0,
    /// Win351
    #[serde(rename = "Win351")]
    Win351 = 1,
    /// NT40
    #[serde(rename = "NT40")]
    NT40 = 2,
    /// Win2k
    #[serde(rename = "Win2k")]
    Win2k = 3,
}

impl Default for PrinterDriver_Version {
    fn default() -> Self {
        Self::Win9x
    }
}

