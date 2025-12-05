// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Synchronized_ProgressStatus
//////////////////////////////////////////////

/// Synchronized_ProgressStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Synchronized_ProgressStatus {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Completed
    #[serde(rename = "Completed")]
    Completed = 2,
    /// Dormant
    #[serde(rename = "Dormant")]
    Dormant = 3,
    /// Initializing
    #[serde(rename = "Initializing")]
    Initializing = 4,
    /// Preparing
    #[serde(rename = "Preparing")]
    Preparing = 5,
    /// Synchronizing
    #[serde(rename = "Synchronizing")]
    Synchronizing = 6,
    /// Resyncing
    #[serde(rename = "Resyncing")]
    Resyncing = 7,
    /// Restoring
    #[serde(rename = "Restoring")]
    Restoring = 8,
    /// Fracturing
    #[serde(rename = "Fracturing")]
    Fracturing = 9,
    /// Splitting
    #[serde(rename = "Splitting")]
    Splitting = 10,
    /// Failing_over
    #[serde(rename = "Failing_over")]
    FailingOver = 11,
    /// Failing_back
    #[serde(rename = "Failing_back")]
    FailingBack = 12,
    /// Aborting
    #[serde(rename = "Aborting")]
    Aborting = 13,
    /// Mixed
    #[serde(rename = "Mixed")]
    Mixed = 14,
    /// Not_Applicable
    #[serde(rename = "Not_Applicable")]
    NotApplicable = 15,
    /// Suspending
    #[serde(rename = "Suspending")]
    Suspending = 16,
    /// Requires_fracture
    #[serde(rename = "Requires_fracture")]
    RequiresFracture = 17,
    /// Requires_resync
    #[serde(rename = "Requires_resync")]
    RequiresResync = 18,
    /// Requires_activate
    #[serde(rename = "Requires_activate")]
    RequiresActivate = 19,
    /// Pending
    #[serde(rename = "Pending")]
    Pending = 20,
    /// Detaching
    #[serde(rename = "Detaching")]
    Detaching = 21,
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 22,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 23,
}

impl Default for Synchronized_ProgressStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

