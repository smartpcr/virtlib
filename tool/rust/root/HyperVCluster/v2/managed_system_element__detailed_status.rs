// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ManagedSystemElement_DetailedStatus
//////////////////////////////////////////////

/// ManagedSystemElement_DetailedStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ManagedSystemElement_DetailedStatus {
    /// Not_Available
    #[serde(rename = "Not_Available")]
    NotAvailable = 0,
    /// No_Additional_Information
    #[serde(rename = "No_Additional_Information")]
    NoAdditionalInformation = 1,
    /// Stressed
    #[serde(rename = "Stressed")]
    Stressed = 2,
    /// Predictive_Failure
    #[serde(rename = "Predictive_Failure")]
    PredictiveFailure = 3,
    /// Non_Recoverable_Error
    #[serde(rename = "Non_Recoverable_Error")]
    NonRecoverableError = 4,
    /// Supporting_Entity_in_Error
    #[serde(rename = "Supporting_Entity_in_Error")]
    SupportingEntityInError = 5,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 6,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 7,
}

impl Default for ManagedSystemElement_DetailedStatus {
    fn default() -> Self {
        Self::NotAvailable
    }
}

