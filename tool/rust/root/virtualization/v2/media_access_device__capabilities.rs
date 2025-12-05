// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MediaAccessDevice_Capabilities
//////////////////////////////////////////////

/// MediaAccessDevice_Capabilities enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MediaAccessDevice_Capabilities {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Sequential_Access
    #[serde(rename = "Sequential_Access")]
    SequentialAccess = 2,
    /// Random_Access
    #[serde(rename = "Random_Access")]
    RandomAccess = 3,
    /// Supports_Writing
    #[serde(rename = "Supports_Writing")]
    SupportsWriting = 4,
    /// Encryption
    #[serde(rename = "Encryption")]
    Encryption = 5,
    /// Compression
    #[serde(rename = "Compression")]
    Compression = 6,
    /// Supports_Removeable_Media
    #[serde(rename = "Supports_Removeable_Media")]
    SupportsRemoveableMedia = 7,
    /// Manual_Cleaning
    #[serde(rename = "Manual_Cleaning")]
    ManualCleaning = 8,
    /// Automatic_Cleaning
    #[serde(rename = "Automatic_Cleaning")]
    AutomaticCleaning = 9,
    /// SMART_Notification
    #[serde(rename = "SMART_Notification")]
    SMARTNotification = 10,
    /// Supports_Dual_Sided_Media
    #[serde(rename = "Supports_Dual_Sided_Media")]
    SupportsDualSidedMedia = 11,
    /// Predismount_Eject_Not_Required
    #[serde(rename = "Predismount_Eject_Not_Required")]
    PredismountEjectNotRequired = 12,
}

impl Default for MediaAccessDevice_Capabilities {
    fn default() -> Self {
        Self::Unknown
    }
}

