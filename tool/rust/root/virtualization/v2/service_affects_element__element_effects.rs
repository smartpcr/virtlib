// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ServiceAffectsElement_ElementEffects
//////////////////////////////////////////////

/// ServiceAffectsElement_ElementEffects enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ServiceAffectsElement_ElementEffects {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Exclusive_Use
    #[serde(rename = "Exclusive_Use")]
    ExclusiveUse = 2,
    /// Performance_Impact
    #[serde(rename = "Performance_Impact")]
    PerformanceImpact = 3,
    /// Element_Integrity
    #[serde(rename = "Element_Integrity")]
    ElementIntegrity = 4,
    /// Manages
    #[serde(rename = "Manages")]
    Manages = 5,
    /// Consumes
    #[serde(rename = "Consumes")]
    Consumes = 6,
    /// Enhances_Integrity
    #[serde(rename = "Enhances_Integrity")]
    EnhancesIntegrity = 7,
    /// Degrades_Integrity
    #[serde(rename = "Degrades_Integrity")]
    DegradesIntegrity = 8,
    /// Enhances_Performance
    #[serde(rename = "Enhances_Performance")]
    EnhancesPerformance = 9,
    /// Degrades_Performance
    #[serde(rename = "Degrades_Performance")]
    DegradesPerformance = 10,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 11,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 12,
}

impl Default for ServiceAffectsElement_ElementEffects {
    fn default() -> Self {
        Self::Unknown
    }
}

