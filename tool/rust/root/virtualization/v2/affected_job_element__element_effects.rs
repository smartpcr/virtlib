// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source AffectedJobElement_ElementEffects
//////////////////////////////////////////////

/// AffectedJobElement_ElementEffects enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum AffectedJobElement_ElementEffects {
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
    /// Create
    #[serde(rename = "Create")]
    Create = 5,
}

impl Default for AffectedJobElement_ElementEffects {
    fn default() -> Self {
        Self::Unknown
    }
}

