// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Error_ErrorType
//////////////////////////////////////////////

/// Error_ErrorType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Error_ErrorType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Communications_Error
    #[serde(rename = "Communications_Error")]
    CommunicationsError = 2,
    /// Quality_of_Service_Error
    #[serde(rename = "Quality_of_Service_Error")]
    QualityOfServiceError = 3,
    /// Software_Error
    #[serde(rename = "Software_Error")]
    SoftwareError = 4,
    /// Hardware_Error
    #[serde(rename = "Hardware_Error")]
    HardwareError = 5,
    /// Environmental_Error
    #[serde(rename = "Environmental_Error")]
    EnvironmentalError = 6,
    /// Security_Error
    #[serde(rename = "Security_Error")]
    SecurityError = 7,
    /// Oversubscription_Error
    #[serde(rename = "Oversubscription_Error")]
    OversubscriptionError = 8,
    /// Unavailable_Resource_Error
    #[serde(rename = "Unavailable_Resource_Error")]
    UnavailableResourceError = 9,
    /// Unsupported_Operation_Error
    #[serde(rename = "Unsupported_Operation_Error")]
    UnsupportedOperationError = 10,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 11,
}

impl Default for Error_ErrorType {
    fn default() -> Self {
        Self::Unknown
    }
}

