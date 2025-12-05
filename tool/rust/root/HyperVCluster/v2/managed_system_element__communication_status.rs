// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ManagedSystemElement_CommunicationStatus
//////////////////////////////////////////////

/// ManagedSystemElement_CommunicationStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ManagedSystemElement_CommunicationStatus {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Not_Available
    #[serde(rename = "Not_Available")]
    NotAvailable = 1,
    /// Communication_OK
    #[serde(rename = "Communication_OK")]
    CommunicationOK = 2,
    /// Lost_Communication
    #[serde(rename = "Lost_Communication")]
    LostCommunication = 3,
    /// No_Contact
    #[serde(rename = "No_Contact")]
    NoContact = 4,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 5,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 6,
}

impl Default for ManagedSystemElement_CommunicationStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

