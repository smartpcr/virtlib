// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DedupProperties_DedupMode
//////////////////////////////////////////////

/// DedupProperties_DedupMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DedupProperties_DedupMode {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 0,
    /// GeneralPurpose
    #[serde(rename = "GeneralPurpose")]
    GeneralPurpose = 1,
    /// HyperV
    #[serde(rename = "HyperV")]
    HyperV = 2,
    /// Backup
    #[serde(rename = "Backup")]
    Backup = 3,
    /// NotAvailable
    #[serde(rename = "NotAvailable")]
    NotAvailable = 4,
}

impl Default for DedupProperties_DedupMode {
    fn default() -> Self {
        Self::Disabled
    }
}

