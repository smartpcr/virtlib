// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Synchronized_SyncState
//////////////////////////////////////////////

/// Synchronized_SyncState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Synchronized_SyncState {
    /// Initialized
    #[serde(rename = "Initialized")]
    Initialized = 2,
    /// PrepareInProgress
    #[serde(rename = "PrepareInProgress")]
    PrepareInProgress = 3,
    /// Prepared
    #[serde(rename = "Prepared")]
    Prepared = 4,
    /// ResyncInProgress
    #[serde(rename = "ResyncInProgress")]
    ResyncInProgress = 5,
    /// Synchronized
    #[serde(rename = "Synchronized")]
    Synchronized = 6,
    /// Fracture_In_Progress
    #[serde(rename = "Fracture_In_Progress")]
    FractureInProgress = 7,
    /// QuiesceInProgress
    #[serde(rename = "QuiesceInProgress")]
    QuiesceInProgress = 8,
    /// Quiesced
    #[serde(rename = "Quiesced")]
    Quiesced = 9,
    /// Restore_In_Progresss
    #[serde(rename = "Restore_In_Progresss")]
    RestoreInProgresss = 10,
    /// Idle
    #[serde(rename = "Idle")]
    Idle = 11,
    /// Broken
    #[serde(rename = "Broken")]
    Broken = 12,
    /// Fractured
    #[serde(rename = "Fractured")]
    Fractured = 13,
    /// Frozen
    #[serde(rename = "Frozen")]
    Frozen = 14,
    /// Copy_In_Progress
    #[serde(rename = "Copy_In_Progress")]
    CopyInProgress = 15,
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 16,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 17,
}

impl Default for Synchronized_SyncState {
    fn default() -> Self {
        Self::Initialized
    }
}

