// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BaseMetricDefinition_DataType
//////////////////////////////////////////////

/// BaseMetricDefinition_DataType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BaseMetricDefinition_DataType {
    /// boolean
    #[serde(rename = "boolean")]
    Boolean = 1,
    /// char16
    #[serde(rename = "char16")]
    Char16 = 2,
    /// datetime
    #[serde(rename = "datetime")]
    Datetime = 3,
    /// real32
    #[serde(rename = "real32")]
    Real32 = 4,
    /// real64
    #[serde(rename = "real64")]
    Real64 = 5,
    /// sint16
    #[serde(rename = "sint16")]
    Sint16 = 6,
    /// sint32
    #[serde(rename = "sint32")]
    Sint32 = 7,
    /// sint64
    #[serde(rename = "sint64")]
    Sint64 = 8,
    /// sint8
    #[serde(rename = "sint8")]
    Sint8 = 9,
    /// _string
    #[serde(rename = "_string")]
    String = 10,
    /// uint16
    #[serde(rename = "uint16")]
    Uint16 = 11,
    /// uint32
    #[serde(rename = "uint32")]
    Uint32 = 12,
    /// uint64
    #[serde(rename = "uint64")]
    Uint64 = 13,
    /// uint8
    #[serde(rename = "uint8")]
    Uint8 = 14,
}

impl Default for BaseMetricDefinition_DataType {
    fn default() -> Self {
        Self::Boolean
    }
}

