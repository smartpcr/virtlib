// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PointingDevice_Handedness
//////////////////////////////////////////////

/// PointingDevice_Handedness enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PointingDevice_Handedness {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Not_Applicable
    #[serde(rename = "Not_Applicable")]
    NotApplicable = 1,
    /// Right_Handed_Operation
    #[serde(rename = "Right_Handed_Operation")]
    RightHandedOperation = 2,
    /// Left_Handed_Operation
    #[serde(rename = "Left_Handed_Operation")]
    LeftHandedOperation = 3,
}

impl Default for PointingDevice_Handedness {
    fn default() -> Self {
        Self::Unknown
    }
}

