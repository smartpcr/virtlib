// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PointingDevice_PointingType
//////////////////////////////////////////////

/// PointingDevice_PointingType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PointingDevice_PointingType {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
    /// Mouse
    #[serde(rename = "Mouse")]
    Mouse = 3,
    /// Track_Ball
    #[serde(rename = "Track_Ball")]
    TrackBall = 4,
    /// Track_Point
    #[serde(rename = "Track_Point")]
    TrackPoint = 5,
    /// Glide_Point
    #[serde(rename = "Glide_Point")]
    GlidePoint = 6,
    /// Touch_Pad
    #[serde(rename = "Touch_Pad")]
    TouchPad = 7,
    /// Touch_Screen
    #[serde(rename = "Touch_Screen")]
    TouchScreen = 8,
    /// Mouse___Optical_Sensor
    #[serde(rename = "Mouse___Optical_Sensor")]
    MouseOpticalSensor = 9,
}

impl Default for PointingDevice_PointingType {
    fn default() -> Self {
        Self::Other
    }
}

