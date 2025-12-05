// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PhysicalDisk_CannotPoolReason
//////////////////////////////////////////////

/// PhysicalDisk_CannotPoolReason enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PhysicalDisk_CannotPoolReason {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// In_a_Pool
    #[serde(rename = "In_a_Pool")]
    InAPool = 2,
    /// Not_Healthy
    #[serde(rename = "Not_Healthy")]
    NotHealthy = 3,
    /// Removable_Media
    #[serde(rename = "Removable_Media")]
    RemovableMedia = 4,
    /// In_Use_by_Cluster
    #[serde(rename = "In_Use_by_Cluster")]
    InUseByCluster = 5,
    /// Offline
    #[serde(rename = "Offline")]
    Offline = 6,
    /// Insufficient_Capacity
    #[serde(rename = "Insufficient_Capacity")]
    InsufficientCapacity = 7,
    /// Spare_Disk
    #[serde(rename = "Spare_Disk")]
    SpareDisk = 8,
    /// Reserved_by_subsystem
    #[serde(rename = "Reserved_by_subsystem")]
    ReservedBySubsystem = 9,
    /// Starting
    #[serde(rename = "Starting")]
    Starting = 10,
    /// Partial_SCM
    #[serde(rename = "Partial_SCM")]
    PartialSCM = 11,
    /// Discovery_Disabled
    #[serde(rename = "Discovery_Disabled")]
    DiscoveryDisabled = 12,
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 13,
    /// Verification_in_progress
    #[serde(rename = "Verification_in_progress")]
    VerificationInProgress = 14,
    /// Verification_failed
    #[serde(rename = "Verification_failed")]
    VerificationFailed = 15,
    /// Firmware_not_compliant
    #[serde(rename = "Firmware_not_compliant")]
    FirmwareNotCompliant = 16,
    /// Hardware_not_compliant
    #[serde(rename = "Hardware_not_compliant")]
    HardwareNotCompliant = 17,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 18,
}

impl Default for PhysicalDisk_CannotPoolReason {
    fn default() -> Self {
        Self::Unknown
    }
}

