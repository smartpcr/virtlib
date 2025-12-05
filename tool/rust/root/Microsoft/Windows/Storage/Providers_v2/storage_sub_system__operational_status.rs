// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageSubSystem_OperationalStatus
//////////////////////////////////////////////

/// StorageSubSystem_OperationalStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageSubSystem_OperationalStatus {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// OK
    #[serde(rename = "OK")]
    OK = 2,
    /// Degraded
    #[serde(rename = "Degraded")]
    Degraded = 3,
    /// Stressed
    #[serde(rename = "Stressed")]
    Stressed = 4,
    /// Predictive_Failure
    #[serde(rename = "Predictive_Failure")]
    PredictiveFailure = 5,
    /// Error
    #[serde(rename = "Error")]
    Error = 6,
    /// Non_Recoverable_Error
    #[serde(rename = "Non_Recoverable_Error")]
    NonRecoverableError = 7,
    /// Starting
    #[serde(rename = "Starting")]
    Starting = 8,
    /// Stopping
    #[serde(rename = "Stopping")]
    Stopping = 9,
    /// Stopped
    #[serde(rename = "Stopped")]
    Stopped = 10,
    /// In_Service
    #[serde(rename = "In_Service")]
    InService = 11,
    /// No_Contact
    #[serde(rename = "No_Contact")]
    NoContact = 12,
    /// Lost_Communication
    #[serde(rename = "Lost_Communication")]
    LostCommunication = 13,
    /// Aborted
    #[serde(rename = "Aborted")]
    Aborted = 14,
    /// Dormant
    #[serde(rename = "Dormant")]
    Dormant = 15,
    /// Supporting_Entity_in_Error
    #[serde(rename = "Supporting_Entity_in_Error")]
    SupportingEntityInError = 16,
    /// Completed
    #[serde(rename = "Completed")]
    Completed = 17,
    /// Power_Mode
    #[serde(rename = "Power_Mode")]
    PowerMode = 18,
    /// Relocating
    #[serde(rename = "Relocating")]
    Relocating = 19,
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 20,
}

impl Default for StorageSubSystem_OperationalStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

