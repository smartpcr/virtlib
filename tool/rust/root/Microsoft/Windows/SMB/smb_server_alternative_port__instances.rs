// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbServerAlternativePort_Instances
//////////////////////////////////////////////

/// SmbServerAlternativePort_Instances enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbServerAlternativePort_Instances {
    /// _18
    #[serde(rename = "_18")]
    V18 = 0,
    /// _9
    #[serde(rename = "_9")]
    V9 = 1,
    /// _10
    #[serde(rename = "_10")]
    V10 = 2,
    /// _56
    #[serde(rename = "_56")]
    V56 = 4,
    /// _57
    #[serde(rename = "_57")]
    V57 = 8,
    /// _83
    #[serde(rename = "_83")]
    V83 = 3,
    /// _84
    #[serde(rename = "_84")]
    V84 = 5,
    /// _85
    #[serde(rename = "_85")]
    V85 = 9,
    /// _86
    #[serde(rename = "_86")]
    V86 = 6,
    /// _87
    #[serde(rename = "_87")]
    V87 = 10,
    /// _88
    #[serde(rename = "_88")]
    V88 = 12,
    /// _89
    #[serde(rename = "_89")]
    V89 = 7,
    /// _90
    #[serde(rename = "_90")]
    V90 = 11,
    /// _91
    #[serde(rename = "_91")]
    V91 = 13,
    /// _92
    #[serde(rename = "_92")]
    V92 = 14,
    /// _93
    #[serde(rename = "_93")]
    V93 = 15,
}

impl Default for SmbServerAlternativePort_Instances {
    fn default() -> Self {
        Self::V18
    }
}

